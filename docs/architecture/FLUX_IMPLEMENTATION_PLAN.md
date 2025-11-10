# Flux Architecture Implementation Plan for Purdah-UI

## Executive Summary

This document outlines a comprehensive plan to implement the Flux architecture pattern in purdah-ui. Flux provides a unidirectional data flow architecture with centralized state management, originally created by Facebook for React applications.

**Timeline**: 10-13 weeks
**Complexity**: High
**Team Size**: 3-4 developers
**Risk Level**: Medium-High

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Components](#core-components)
3. [Implementation Phases](#implementation-phases)
4. [Detailed Phase Breakdown](#detailed-phase-breakdown)
5. [Code Examples](#code-examples)
6. [Testing Strategy](#testing-strategy)
7. [Documentation Plan](#documentation-plan)
8. [Migration Path](#migration-path)
9. [Trade-offs & Risks](#trade-offs--risks)
10. [Success Metrics](#success-metrics)

---

## Architecture Overview

### Flux Pattern Structure

```
┌─────────────────────────────────────────────────────────────┐
│                   Unidirectional Data Flow                   │
│                                                               │
│  ┌──────────┐   Action    ┌────────────┐                    │
│  │   View   │ ──────────> │ Dispatcher │                    │
│  │          │             │            │                    │
│  └──────────┘             └────────────┘                    │
│       ▲                         │                           │
│       │                         │ Dispatch                  │
│       │                         ▼                           │
│       │                   ┌──────────┐                      │
│       │                   │   Store  │                      │
│       │                   │          │                      │
│       │                   └──────────┘                      │
│       │                         │                           │
│       │ Change Event            │ Update                    │
│       └─────────────────────────┘                           │
│                                                               │
│  Side Effects (Middleware): API calls, logging, etc.         │
└─────────────────────────────────────────────────────────────┘
```

### Key Principles

1. **Unidirectional Data Flow**: Actions → Dispatcher → Store → View
2. **Single Dispatcher**: Central hub for all actions
3. **Stores as State Containers**: Multiple stores, each managing domain state
4. **No Store-to-Store Communication**: Stores update independently
5. **Views React to Store Changes**: Automatic re-rendering

---

## Core Components

### 1. Action System

```rust
// src/flux/action.rs

/// Action trait for all dispatched events
pub trait Action: Clone + Send + Sync + Debug + 'static {
    /// Action type identifier
    fn action_type(&self) -> &'static str;

    /// Optional payload for debugging
    fn payload(&self) -> Option<serde_json::Value> {
        None
    }
}

/// Action metadata for middleware
#[derive(Debug, Clone)]
pub struct ActionMetadata {
    pub timestamp: Instant,
    pub source: Option<String>,
    pub sequence_id: u64,
}

/// Typed action wrapper
#[derive(Clone, Debug)]
pub struct TypedAction<A: Action> {
    pub action: A,
    pub metadata: ActionMetadata,
}
```

### 2. Dispatcher

```rust
// src/flux/dispatcher.rs

/// Central event dispatcher
pub struct Dispatcher {
    callbacks: Arc<RwLock<Vec<DispatchCallback>>>,
    middleware: Arc<RwLock<Vec<Box<dyn Middleware>>>>,
    sequence: Arc<AtomicU64>,
    dispatching: Arc<AtomicBool>,
}

type DispatchCallback = Arc<dyn Fn(&dyn Any) + Send + Sync>;

impl Dispatcher {
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(RwLock::new(Vec::new())),
            middleware: Arc::new(RwLock::new(Vec::new())),
            sequence: Arc::new(AtomicU64::new(0)),
            dispatching: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Register a store callback
    pub fn register<A: Action>(&self, callback: impl Fn(&A) + Send + Sync + 'static) -> DispatchToken {
        let callback: DispatchCallback = Arc::new(move |action| {
            if let Some(typed_action) = (action as &dyn Any).downcast_ref::<A>() {
                callback(typed_action);
            }
        });

        let mut callbacks = self.callbacks.write().unwrap();
        callbacks.push(callback);

        DispatchToken(callbacks.len() - 1)
    }

    /// Dispatch an action to all registered stores
    pub fn dispatch<A: Action>(&self, action: A) {
        // Prevent nested dispatches (Flux invariant)
        if self.dispatching.swap(true, Ordering::SeqCst) {
            panic!("Cannot dispatch in the middle of a dispatch");
        }

        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let typed_action = TypedAction {
            action: action.clone(),
            metadata: ActionMetadata {
                timestamp: Instant::now(),
                source: None,
                sequence_id: sequence,
            },
        };

        // Run middleware
        let middleware = self.middleware.read().unwrap();
        for mw in middleware.iter() {
            mw.process(&typed_action);
        }

        // Dispatch to stores
        let callbacks = self.callbacks.read().unwrap();
        for callback in callbacks.iter() {
            callback(&action as &dyn Any);
        }

        self.dispatching.store(false, Ordering::SeqCst);
    }

    /// Add middleware
    pub fn add_middleware(&self, middleware: Box<dyn Middleware>) {
        self.middleware.write().unwrap().push(middleware);
    }
}

/// Token for unregistering callbacks
#[derive(Debug, Clone, Copy)]
pub struct DispatchToken(usize);
```

### 3. Store System

```rust
// src/flux/store.rs

/// Store trait for state containers
pub trait Store: Send + Sync + 'static {
    type State: Clone + Send + Sync;
    type Action: Action;

    /// Get current state
    fn state(&self) -> Self::State;

    /// Handle action dispatch
    fn reduce(&mut self, action: &Self::Action);

    /// Register change listener
    fn on_change(&self, callback: Box<dyn Fn(&Self::State) + Send + Sync>) -> ListenerId;

    /// Unregister listener
    fn remove_listener(&self, id: ListenerId);

    /// Initialize store with dispatcher
    fn init(&mut self, dispatcher: &Dispatcher) {
        let self_ptr = self as *mut Self;
        dispatcher.register(move |action: &Self::Action| {
            unsafe {
                (*self_ptr).reduce(action);
                (*self_ptr).emit_change();
            }
        });
    }

    /// Emit change event to all listeners
    fn emit_change(&self) {
        let state = self.state();
        let listeners = self.listeners();
        for listener in listeners.iter() {
            listener(&state);
        }
    }

    /// Get all listeners (implementation detail)
    fn listeners(&self) -> Vec<Box<dyn Fn(&Self::State) + Send + Sync>>;
}

/// Store implementation helper
pub struct StoreImpl<S, A>
where
    S: Clone + Send + Sync + 'static,
    A: Action,
{
    state: Arc<RwLock<S>>,
    listeners: Arc<RwLock<Vec<Box<dyn Fn(&S) + Send + Sync>>>>,
    reducer: Arc<dyn Fn(&mut S, &A) + Send + Sync>,
}

impl<S, A> StoreImpl<S, A>
where
    S: Clone + Send + Sync + 'static,
    A: Action,
{
    pub fn new(initial_state: S, reducer: impl Fn(&mut S, &A) + Send + Sync + 'static) -> Self {
        Self {
            state: Arc::new(RwLock::new(initial_state)),
            listeners: Arc::new(RwLock::new(Vec::new())),
            reducer: Arc::new(reducer),
        }
    }

    pub fn get_state(&self) -> S {
        self.state.read().unwrap().clone()
    }

    pub fn handle_action(&self, action: &A) {
        let mut state = self.state.write().unwrap();
        (self.reducer)(&mut *state, action);
    }

    pub fn subscribe(&self, callback: Box<dyn Fn(&S) + Send + Sync>) -> ListenerId {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(callback);
        ListenerId(listeners.len() - 1)
    }

    pub fn emit(&self) {
        let state = self.get_state();
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener(&state);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListenerId(usize);
```

### 4. Middleware System

```rust
// src/flux/middleware.rs

/// Middleware for intercepting actions
pub trait Middleware: Send + Sync {
    fn process(&self, action: &dyn Any);
}

/// Logger middleware
pub struct LoggerMiddleware {
    enabled: bool,
}

impl Middleware for LoggerMiddleware {
    fn process(&self, action: &dyn Any) {
        if self.enabled {
            println!("[Action] {:?}", action);
        }
    }
}

/// Thunk middleware for async actions
pub struct ThunkMiddleware {
    dispatcher: Arc<Dispatcher>,
}

impl ThunkMiddleware {
    pub fn new(dispatcher: Arc<Dispatcher>) -> Self {
        Self { dispatcher }
    }
}

pub type Thunk = Box<dyn FnOnce(&Dispatcher) + Send>;

impl Middleware for ThunkMiddleware {
    fn process(&self, action: &dyn Any) {
        if let Some(thunk) = action.downcast_ref::<Thunk>() {
            // Execute thunk with dispatcher
        }
    }
}

/// DevTools middleware for time-travel debugging
pub struct DevToolsMiddleware {
    history: Arc<RwLock<Vec<ActionEntry>>>,
}

#[derive(Clone)]
struct ActionEntry {
    action: String,
    timestamp: Instant,
}

impl Middleware for DevToolsMiddleware {
    fn process(&self, action: &dyn Any) {
        let mut history = self.history.write().unwrap();
        history.push(ActionEntry {
            action: format!("{:?}", action),
            timestamp: Instant::now(),
        });
    }
}
```

### 5. GPUI Integration

```rust
// src/flux/container.rs

/// Flux container that wraps a store and provides GPUI integration
pub struct FluxContainer<S: Store> {
    store: Arc<S>,
    dispatcher: Arc<Dispatcher>,
}

impl<S: Store> FluxContainer<S> {
    pub fn new(store: S, dispatcher: Arc<Dispatcher>) -> View<Self> {
        let store = Arc::new(store);

        // Register store with dispatcher
        let store_clone = Arc::clone(&store);
        dispatcher.register(move |action: &S::Action| {
            store_clone.reduce(action);
        });

        Self { store, dispatcher }
    }

    pub fn dispatch(&self, action: S::Action) {
        self.dispatcher.dispatch(action);
    }

    pub fn state(&self) -> S::State {
        self.store.state()
    }
}

/// Hook for connecting components to stores
pub trait ConnectStore<S: Store> {
    fn connect(&self, store: Arc<S>, cx: &mut ViewContext<Self>) -> S::State;
}

/// Component that connects to a store
pub struct Connected<S: Store, V> {
    store: Arc<S>,
    view_fn: Arc<dyn Fn(&S::State, &Dispatcher) -> V>,
    dispatcher: Arc<Dispatcher>,
}

impl<S: Store, V: IntoElement> Render for Connected<S, V> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.store.state();

        // Subscribe to store changes
        let cx_handle = cx.view().clone();
        self.store.on_change(Box::new(move |_| {
            // Request re-render
            cx_handle.update(|_, cx| cx.notify());
        }));

        (self.view_fn)(&state, &self.dispatcher)
    }
}
```

---

## Implementation Phases

### Phase 1: Core Infrastructure (Weeks 1-3)
- Action system and traits
- Dispatcher implementation
- Basic store implementation
- Middleware foundation
- Unit tests

### Phase 2: Store Architecture (Weeks 4-5)
- Store helper macros
- Multiple store coordination
- Store composition utilities
- Async action creators
- Integration tests

### Phase 3: GPUI Integration (Weeks 6-7)
- FluxContainer component
- Store connection hooks
- Component event integration
- Automatic re-rendering
- Integration tests

### Phase 4: Component Library Updates (Weeks 8-9)
- Update all atoms with action dispatch
- Update molecules and organisms
- Create connected component variants
- Performance optimizations

### Phase 5: Developer Tools & Middleware (Weeks 10-11)
- Logger middleware
- DevTools middleware
- Thunk middleware for async
- Performance monitoring
- Time-travel debugging

### Phase 6: Documentation & Examples (Week 12)
- Comprehensive API docs
- Example applications
- Migration guides
- Best practices documentation

### Phase 7: Testing & Refinement (Week 13)
- Comprehensive test suite
- Performance benchmarks
- Real-world app testing
- Community feedback

---

## Detailed Phase Breakdown

### Phase 1: Core Infrastructure (Weeks 1-3)

#### Week 1: Action System

**Day 1-2: Action Trait & Definitions**
```rust
// src/flux/action.rs

/// Core action trait
pub trait Action: Clone + Send + Sync + Debug + 'static {
    fn action_type(&self) -> &'static str;
}

/// Macro for defining actions
#[macro_export]
macro_rules! define_actions {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident $({ $($field:ident: $ty:ty),* $(,)? })?
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant $({ $($field: $ty),* })?
            ),*
        }

        impl $crate::flux::Action for $name {
            fn action_type(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant { .. } => stringify!($variant),
                    )*
                }
            }
        }
    };
}
```

**Tasks:**
- [ ] Define `Action` trait
- [ ] Create `define_actions!` macro
- [ ] Add action creators pattern
- [ ] Add action metadata support
- [ ] Write unit tests

**Day 3-5: Dispatcher Implementation**
```rust
// src/flux/dispatcher.rs

pub struct Dispatcher {
    callbacks: Arc<RwLock<HashMap<TypeId, Vec<DispatchCallback>>>>,
    is_dispatching: Arc<AtomicBool>,
    pending_actions: Arc<Mutex<VecDeque<Box<dyn Any + Send>>>>,
}

impl Dispatcher {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            callbacks: Arc::new(RwLock::new(HashMap::new())),
            is_dispatching: Arc::new(AtomicBool::new(false)),
            pending_actions: Arc::new(Mutex::new(VecDeque::new())),
        })
    }

    pub fn register<A: Action>(
        &self,
        callback: impl Fn(&A) + Send + Sync + 'static,
    ) -> DispatchToken {
        let type_id = TypeId::of::<A>();
        let callback = Arc::new(callback);

        let mut callbacks = self.callbacks.write().unwrap();
        let type_callbacks = callbacks.entry(type_id).or_insert_with(Vec::new);

        let wrapper: DispatchCallback = Arc::new(move |action_any| {
            if let Some(action) = action_any.downcast_ref::<A>() {
                callback(action);
            }
        });

        type_callbacks.push(wrapper);
        DispatchToken {
            type_id,
            index: type_callbacks.len() - 1,
        }
    }

    pub fn dispatch<A: Action>(&self, action: A) {
        // Enforce no nested dispatches
        if self.is_dispatching.load(Ordering::SeqCst) {
            panic!("Cannot dispatch while dispatching");
        }

        self.is_dispatching.store(true, Ordering::SeqCst);

        let type_id = TypeId::of::<A>();

        // Get callbacks for this action type
        if let Some(callbacks) = self.callbacks.read().unwrap().get(&type_id) {
            for callback in callbacks {
                callback(&action as &dyn Any);
            }
        }

        self.is_dispatching.store(false, Ordering::SeqCst);

        // Process any pending actions
        self.flush_pending();
    }

    fn flush_pending(&self) {
        let mut pending = self.pending_actions.lock().unwrap();
        while let Some(action) = pending.pop_front() {
            // Dispatch pending action
        }
    }
}
```

**Tasks:**
- [ ] Implement dispatcher core
- [ ] Add callback registration
- [ ] Enforce dispatch invariants
- [ ] Add dispatch token system
- [ ] Handle pending actions
- [ ] Write unit tests
- [ ] Test thread safety

#### Week 2: Store Foundation

**Day 1-3: Store Trait & Implementation**
```rust
// src/flux/store.rs

pub trait Store: Send + Sync + 'static {
    type State: Clone + Send + Sync;
    type Action: Action;

    /// Get current state (immutable)
    fn state(&self) -> Self::State;

    /// Reduce action into state
    fn reduce(&mut self, action: &Self::Action);

    /// Register change listener
    fn subscribe(&self, listener: Box<dyn Fn(&Self::State) + Send + Sync>) -> SubscriptionId;

    /// Emit change to listeners
    fn emit_change(&self);
}

/// Helper for creating stores
pub struct StoreBuilder<S, A>
where
    S: Clone + Send + Sync + 'static,
    A: Action,
{
    initial_state: S,
    reducer: Option<Box<dyn Fn(&mut S, &A) + Send + Sync>>,
}

impl<S, A> StoreBuilder<S, A>
where
    S: Clone + Send + Sync + 'static,
    A: Action,
{
    pub fn new(initial_state: S) -> Self {
        Self {
            initial_state,
            reducer: None,
        }
    }

    pub fn reducer(mut self, reducer: impl Fn(&mut S, &A) + Send + Sync + 'static) -> Self {
        self.reducer = Some(Box::new(reducer));
        self
    }

    pub fn build(self) -> impl Store<State = S, Action = A> {
        let reducer = self.reducer.expect("Reducer required");

        BasicStore {
            state: Arc::new(RwLock::new(self.initial_state)),
            listeners: Arc::new(RwLock::new(Vec::new())),
            reducer: Arc::new(reducer),
        }
    }
}

struct BasicStore<S, A>
where
    S: Clone + Send + Sync,
    A: Action,
{
    state: Arc<RwLock<S>>,
    listeners: Arc<RwLock<Vec<Box<dyn Fn(&S) + Send + Sync>>>>,
    reducer: Arc<dyn Fn(&mut S, &A) + Send + Sync>,
}

impl<S, A> Store for BasicStore<S, A>
where
    S: Clone + Send + Sync + 'static,
    A: Action,
{
    type State = S;
    type Action = A;

    fn state(&self) -> S {
        self.state.read().unwrap().clone()
    }

    fn reduce(&mut self, action: &A) {
        let mut state = self.state.write().unwrap();
        (self.reducer)(&mut *state, action);
    }

    fn subscribe(&self, listener: Box<dyn Fn(&S) + Send + Sync>) -> SubscriptionId {
        let mut listeners = self.listeners.write().unwrap();
        listeners.push(listener);
        SubscriptionId(listeners.len() - 1)
    }

    fn emit_change(&self) {
        let state = self.state();
        let listeners = self.listeners.read().unwrap();
        for listener in listeners.iter() {
            listener(&state);
        }
    }
}
```

**Tasks:**
- [ ] Define `Store` trait
- [ ] Implement `StoreBuilder`
- [ ] Create `BasicStore` implementation
- [ ] Add listener management
- [ ] Write unit tests
- [ ] Document store patterns

**Day 4-5: Store Macros**
```rust
// src/flux/macros.rs

#[macro_export]
macro_rules! define_store {
    (
        state: $state:ty,
        action: $action:ty,
        initial: $initial:expr,
        reducer: |$state_param:ident, $action_param:ident| $body:block
    ) => {
        StoreBuilder::new($initial)
            .reducer(|$state_param: &mut $state, $action_param: &$action| $body)
            .build()
    };
}
```

**Tasks:**
- [ ] Create `define_store!` macro
- [ ] Add reducer helper macros
- [ ] Create action creator utilities
- [ ] Write macro tests
- [ ] Document macro usage

#### Week 3: Middleware & Advanced Features

**Day 1-2: Middleware System**
```rust
// src/flux/middleware.rs

pub trait Middleware: Send + Sync {
    fn before_dispatch(&self, action: &dyn Any) -> MiddlewareResult;
    fn after_dispatch(&self, action: &dyn Any);
}

pub enum MiddlewareResult {
    Continue,
    Stop,
    Replace(Box<dyn Any + Send>),
}

/// Logger middleware
pub struct Logger {
    prefix: String,
}

impl Middleware for Logger {
    fn before_dispatch(&self, action: &dyn Any) -> MiddlewareResult {
        println!("{} {:?}", self.prefix, action);
        MiddlewareResult::Continue
    }

    fn after_dispatch(&self, _action: &dyn Any) {}
}

/// Async middleware (thunk pattern)
pub struct AsyncMiddleware {
    dispatcher: Weak<Dispatcher>,
    runtime: tokio::runtime::Runtime,
}

impl AsyncMiddleware {
    pub fn dispatch_async<F, A>(&self, f: F)
    where
        F: FnOnce() -> A + Send + 'static,
        A: Action,
    {
        if let Some(dispatcher) = self.dispatcher.upgrade() {
            self.runtime.spawn(async move {
                let action = f();
                dispatcher.dispatch(action);
            });
        }
    }
}
```

**Tasks:**
- [ ] Define `Middleware` trait
- [ ] Implement logger middleware
- [ ] Implement async middleware
- [ ] Add middleware chain management
- [ ] Write unit tests

**Day 3-5: Advanced Dispatcher Features**
```rust
// Enhanced dispatcher with middleware
impl Dispatcher {
    pub fn with_middleware(middleware: Vec<Box<dyn Middleware>>) -> Arc<Self> {
        let dispatcher = Self::new();
        for mw in middleware {
            dispatcher.add_middleware(mw);
        }
        Arc::new(dispatcher)
    }

    pub fn dispatch_with_middleware<A: Action>(&self, action: A) {
        let mut current_action: Box<dyn Any + Send> = Box::new(action);

        // Run before_dispatch middleware
        for middleware in self.middleware.read().unwrap().iter() {
            match middleware.before_dispatch(&*current_action) {
                MiddlewareResult::Continue => {}
                MiddlewareResult::Stop => return,
                MiddlewareResult::Replace(new_action) => {
                    current_action = new_action;
                }
            }
        }

        // Dispatch action
        if let Some(action) = current_action.downcast_ref::<A>() {
            self.dispatch(action.clone());
        }

        // Run after_dispatch middleware
        for middleware in self.middleware.read().unwrap().iter() {
            middleware.after_dispatch(&*current_action);
        }
    }
}
```

**Tasks:**
- [ ] Add middleware integration to dispatcher
- [ ] Implement middleware chain
- [ ] Add error handling for middleware
- [ ] Write integration tests
- [ ] Document middleware patterns

---

### Phase 2: Store Architecture (Weeks 4-5)

#### Week 4: Multiple Store Coordination

**Day 1-2: Container Pattern**
```rust
// src/flux/container.rs

/// Container for multiple stores
pub struct StoreContainer {
    stores: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    dispatcher: Arc<Dispatcher>,
}

impl StoreContainer {
    pub fn new(dispatcher: Arc<Dispatcher>) -> Self {
        Self {
            stores: HashMap::new(),
            dispatcher,
        }
    }

    pub fn add_store<S: Store>(&mut self, mut store: S) {
        // Register store with dispatcher
        let store_ptr = &mut store as *mut S;
        self.dispatcher.register(move |action: &S::Action| {
            unsafe {
                (*store_ptr).reduce(action);
                (*store_ptr).emit_change();
            }
        });

        let type_id = TypeId::of::<S>();
        self.stores.insert(type_id, Box::new(store));
    }

    pub fn get_store<S: Store + 'static>(&self) -> Option<&S> {
        let type_id = TypeId::of::<S>();
        self.stores
            .get(&type_id)
            .and_then(|store| store.downcast_ref::<S>())
    }

    pub fn dispatch<A: Action>(&self, action: A) {
        self.dispatcher.dispatch(action);
    }
}
```

**Tasks:**
- [ ] Implement `StoreContainer`
- [ ] Add store registration
- [ ] Add store retrieval
- [ ] Write integration tests
- [ ] Document container patterns

**Day 3-5: Store Composition**
```rust
// src/flux/composition.rs

/// Combine multiple stores into one
pub struct ComposedStore<S1, S2, A>
where
    S1: Store<Action = A>,
    S2: Store<Action = A>,
    A: Action,
{
    store1: S1,
    store2: S2,
    _phantom: PhantomData<A>,
}

impl<S1, S2, A> ComposedStore<S1, S2, A>
where
    S1: Store<Action = A>,
    S2: Store<Action = A>,
    A: Action,
{
    pub fn new(store1: S1, store2: S2) -> Self {
        Self {
            store1,
            store2,
            _phantom: PhantomData,
        }
    }
}

/// Derived stores (computed state)
pub struct DerivedStore<S, D, A>
where
    S: Store<Action = A>,
    D: Clone + Send + Sync,
    A: Action,
{
    source: Arc<S>,
    selector: Arc<dyn Fn(&S::State) -> D + Send + Sync>,
    _phantom: PhantomData<A>,
}

impl<S, D, A> DerivedStore<S, D, A>
where
    S: Store<Action = A>,
    D: Clone + Send + Sync + 'static,
    A: Action,
{
    pub fn new(source: Arc<S>, selector: impl Fn(&S::State) -> D + Send + Sync + 'static) -> Self {
        Self {
            source,
            selector: Arc::new(selector),
            _phantom: PhantomData,
        }
    }

    pub fn select(&self) -> D {
        let state = self.source.state();
        (self.selector)(&state)
    }
}
```

**Tasks:**
- [ ] Implement composed stores
- [ ] Implement derived stores (selectors)
- [ ] Add store combinators
- [ ] Write unit tests
- [ ] Document composition patterns

#### Week 5: Async Action Creators

**Day 1-3: Async Actions (Thunks)**
```rust
// src/flux/async_actions.rs

/// Thunk action creator
pub struct Thunk<A: Action> {
    executor: Box<dyn FnOnce(&Dispatcher) + Send>,
    _phantom: PhantomData<A>,
}

impl<A: Action> Thunk<A> {
    pub fn new(executor: impl FnOnce(&Dispatcher) + Send + 'static) -> Self {
        Self {
            executor: Box::new(executor),
            _phantom: PhantomData,
        }
    }

    pub fn execute(self, dispatcher: &Dispatcher) {
        (self.executor)(dispatcher);
    }
}

/// Async action helpers
pub fn async_action<F, Fut, A>(f: F) -> Thunk<A>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = A> + Send + 'static,
    A: Action,
{
    Thunk::new(move |dispatcher| {
        let dispatcher = dispatcher.clone();
        tokio::spawn(async move {
            let action = f().await;
            dispatcher.dispatch(action);
        });
    })
}

/// Example: HTTP request action creator
pub fn fetch_user(user_id: u64) -> Thunk<UserAction> {
    async_action(move || async move {
        match reqwest::get(format!("https://api.example.com/users/{}", user_id)).await {
            Ok(response) => {
                let user = response.json::<User>().await.unwrap();
                UserAction::FetchSuccess { user }
            }
            Err(error) => UserAction::FetchError {
                error: error.to_string(),
            },
        }
    })
}
```

**Tasks:**
- [ ] Implement thunk pattern
- [ ] Add async action creators
- [ ] Add promise-based actions
- [ ] Handle cancellation
- [ ] Write integration tests

**Day 4-5: Side Effect Management**
```rust
// src/flux/effects.rs

/// Side effect manager
pub struct EffectManager {
    dispatcher: Arc<Dispatcher>,
    runtime: tokio::runtime::Runtime,
}

impl EffectManager {
    pub fn new(dispatcher: Arc<Dispatcher>) -> Self {
        Self {
            dispatcher,
            runtime: tokio::runtime::Runtime::new().unwrap(),
        }
    }

    pub fn run_effect<F, Fut>(&self, effect: F)
    where
        F: FnOnce(Arc<Dispatcher>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let dispatcher = Arc::clone(&self.dispatcher);
        self.runtime.spawn(effect(dispatcher));
    }

    pub fn run_effect_sync<F>(&self, effect: F)
    where
        F: FnOnce(&Dispatcher) + Send + 'static,
    {
        effect(&self.dispatcher);
    }
}
```

**Tasks:**
- [ ] Implement effect manager
- [ ] Add effect scheduling
- [ ] Handle effect errors
- [ ] Add effect cancellation
- [ ] Write unit tests

---

### Phase 3: GPUI Integration (Weeks 6-7)

#### Week 6: FluxContainer Component

**Day 1-3: Container Implementation**
```rust
// src/flux/gpui/container.rs

pub struct FluxContainer {
    container: StoreContainer,
}

impl FluxContainer {
    pub fn new(dispatcher: Arc<Dispatcher>) -> Arc<Self> {
        Arc::new(Self {
            container: StoreContainer::new(dispatcher),
        })
    }

    pub fn add_store<S: Store>(&mut self, store: S) {
        self.container.add_store(store);
    }

    pub fn get_store<S: Store + 'static>(&self) -> Option<&S> {
        self.container.get_store::<S>()
    }

    pub fn dispatch<A: Action>(&self, action: A) {
        self.container.dispatch(action);
    }
}

/// GPUI Provider component
pub struct FluxProvider {
    container: Arc<FluxContainer>,
}

impl FluxProvider {
    pub fn new(container: Arc<FluxContainer>) -> Self {
        Self { container }
    }
}

impl Render for FluxProvider {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Store container in GPUI context
        cx.set_global(self.container.clone());

        // Render children
        div()
    }
}
```

**Tasks:**
- [ ] Implement `FluxContainer`
- [ ] Create `FluxProvider` component
- [ ] Add context integration
- [ ] Wire up to GPUI globals
- [ ] Write integration tests

**Day 4-5: Connection Hooks**
```rust
// src/flux/gpui/connect.rs

/// Connect component to store
pub trait Connect<S: Store> {
    fn connect(&self, cx: &ViewContext<Self>) -> StoreConnection<S>;
}

pub struct StoreConnection<S: Store> {
    store: Arc<S>,
    dispatcher: Arc<Dispatcher>,
}

impl<S: Store> StoreConnection<S> {
    pub fn state(&self) -> S::State {
        self.store.state()
    }

    pub fn dispatch(&self, action: S::Action) {
        self.dispatcher.dispatch(action);
    }

    pub fn subscribe<V>(&self, view: View<V>, cx: &mut ViewContext<V>)
    where
        V: Render,
    {
        let view_handle = view.clone();
        self.store.subscribe(Box::new(move |_state| {
            // Trigger re-render
            view_handle.update(|_, cx| cx.notify());
        }));
    }
}

/// Macro for connecting components
#[macro_export]
macro_rules! connect {
    ($store_type:ty) => {
        impl Connect<$store_type> for Self {
            fn connect(&self, cx: &ViewContext<Self>) -> StoreConnection<$store_type> {
                let container = cx.global::<Arc<FluxContainer>>();
                let store = container.get_store::<$store_type>().unwrap();
                let dispatcher = container.dispatcher.clone();

                StoreConnection {
                    store: Arc::new(store),
                    dispatcher,
                }
            }
        }
    };
}
```

**Tasks:**
- [ ] Implement `Connect` trait
- [ ] Create `StoreConnection`
- [ ] Add `connect!` macro
- [ ] Handle subscription lifecycle
- [ ] Write integration tests

#### Week 7: Component Integration

**Day 1-3: Connected Components**
```rust
// src/flux/gpui/connected.rs

/// Component connected to a store
pub struct Connected<S, V, F>
where
    S: Store,
    V: IntoElement,
    F: Fn(&S::State, &Dispatcher) -> V,
{
    store: Arc<S>,
    dispatcher: Arc<Dispatcher>,
    render_fn: Arc<F>,
    subscription_id: Option<SubscriptionId>,
}

impl<S, V, F> Connected<S, V, F>
where
    S: Store + 'static,
    V: IntoElement,
    F: Fn(&S::State, &Dispatcher) -> V + 'static,
{
    pub fn new(store: Arc<S>, dispatcher: Arc<Dispatcher>, render_fn: F) -> View<Self> {
        cx.new_view(|cx| {
            let mut connected = Self {
                store,
                dispatcher,
                render_fn: Arc::new(render_fn),
                subscription_id: None,
            };

            // Subscribe to store changes
            let view = cx.view().clone();
            let sub_id = connected.store.subscribe(Box::new(move |_| {
                view.update(|_, cx| cx.notify());
            }));
            connected.subscription_id = Some(sub_id);

            connected
        })
    }
}

impl<S, V, F> Render for Connected<S, V, F>
where
    S: Store,
    V: IntoElement,
    F: Fn(&S::State, &Dispatcher) -> V,
{
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.store.state();
        (self.render_fn)(&state, &self.dispatcher)
    }
}

impl<S, V, F> Drop for Connected<S, V, F>
where
    S: Store,
    V: IntoElement,
    F: Fn(&S::State, &Dispatcher) -> V,
{
    fn drop(&mut self) {
        if let Some(sub_id) = self.subscription_id {
            // Unsubscribe on component unmount
        }
    }
}
```

**Tasks:**
- [ ] Implement `Connected` component
- [ ] Add automatic subscription management
- [ ] Handle component lifecycle
- [ ] Optimize re-renders
- [ ] Write integration tests

**Day 4-5: Higher-Order Components**
```rust
// src/flux/gpui/hoc.rs

/// Higher-order component for Flux integration
pub fn with_store<S, P, V>(
    component: impl Fn(&S::State, P, &Dispatcher) -> V + 'static,
) -> impl Fn(P, &ViewContext) -> Connected<S, V, _>
where
    S: Store + 'static,
    P: Clone + 'static,
    V: IntoElement,
{
    move |props, cx| {
        let container = cx.global::<Arc<FluxContainer>>();
        let store = Arc::clone(container.get_store::<S>().unwrap());
        let dispatcher = Arc::clone(&container.dispatcher);

        Connected::new(store, dispatcher, move |state| {
            component(state, props.clone(), &dispatcher)
        })
    }
}
```

**Tasks:**
- [ ] Implement HOC pattern
- [ ] Add prop injection
- [ ] Create utility HOCs
- [ ] Write examples
- [ ] Write tests

---

### Phase 4: Component Library Updates (Weeks 8-9)

#### Week 8: Atom Components

**Update Pattern:**
```rust
// src/atoms/button.rs (Flux-enabled)

pub struct FluxButton<A: Action> {
    props: ButtonProps,
    action: Option<A>,
    _phantom: PhantomData<A>,
}

impl<A: Action> FluxButton<A> {
    pub fn new() -> Self {
        Self {
            props: ButtonProps::default(),
            action: None,
            _phantom: PhantomData,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.props.label = label.into();
        self
    }

    pub fn on_click(mut self, action: A) -> Self {
        self.action = Some(action);
        self
    }
}

impl<A: Action> IntoElement for FluxButton<A> {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();

        div()
            .id("flux-button")
            .when_some(self.action, |div, action| {
                div.on_click(move |_event, cx| {
                    let container = cx.global::<Arc<FluxContainer>>();
                    container.dispatch(action.clone());
                    cx.notify();
                })
            })
            // ... rest of styling
    }
}
```

**Components to update:**
- [ ] Button
- [ ] Input
- [ ] Checkbox
- [ ] Radio
- [ ] Switch
- [ ] All other atoms

**Tasks per component:**
- [ ] Create Flux-enabled variant
- [ ] Add action dispatch
- [ ] Maintain props-based variant for compatibility
- [ ] Write integration tests
- [ ] Update documentation

#### Week 9: Molecules & Organisms

**Day 1-3: Molecule Updates**
```rust
// src/molecules/form_group.rs (Flux-enabled)

pub struct FluxFormGroup<A: Action> {
    label: Option<SharedString>,
    input: FluxInput<A>,
    error: Option<SharedString>,
}
```

**Tasks:**
- [ ] Update FormGroup
- [ ] Update SearchBar
- [ ] Update Card
- [ ] Write tests

**Day 4-5: Organism Updates**
```rust
// src/organisms/dialog.rs (Flux-enabled)

pub struct FluxDialog<A: Action> {
    title: SharedString,
    content: Box<dyn IntoElement>,
    on_close_action: Option<A>,
}
```

**Tasks:**
- [ ] Update Dialog
- [ ] Update Drawer
- [ ] Update Table
- [ ] Update CommandPalette
- [ ] Write tests

---

### Phase 5: Developer Tools & Middleware (Weeks 10-11)

#### Week 10: DevTools

**Day 1-3: Action Logger**
```rust
// src/flux/devtools/logger.rs

pub struct ActionLogger {
    actions: Arc<RwLock<Vec<LoggedAction>>>,
    max_history: usize,
}

#[derive(Clone, Debug)]
pub struct LoggedAction {
    pub action: String,
    pub timestamp: Instant,
    pub duration: Duration,
}

impl ActionLogger {
    pub fn new(max_history: usize) -> Self {
        Self {
            actions: Arc::new(RwLock::new(Vec::with_capacity(max_history))),
            max_history,
        }
    }

    pub fn log(&self, action: &str) {
        let mut actions = self.actions.write().unwrap();
        if actions.len() >= self.max_history {
            actions.remove(0);
        }
        actions.push(LoggedAction {
            action: action.to_string(),
            timestamp: Instant::now(),
            duration: Duration::default(),
        });
    }

    pub fn export(&self) -> String {
        let actions = self.actions.read().unwrap();
        serde_json::to_string_pretty(&*actions).unwrap()
    }
}

impl Middleware for ActionLogger {
    fn before_dispatch(&self, action: &dyn Any) -> MiddlewareResult {
        self.log(&format!("{:?}", action));
        MiddlewareResult::Continue
    }

    fn after_dispatch(&self, _action: &dyn Any) {}
}
```

**Tasks:**
- [ ] Implement action logger
- [ ] Add filtering capabilities
- [ ] Create export functionality
- [ ] Build UI panel (optional)
- [ ] Write tests

**Day 4-5: Time-Travel Debugger**
```rust
// src/flux/devtools/time_travel.rs

pub struct TimeTravelDebugger<S: Store> {
    history: Vec<Snapshot<S>>,
    current_index: usize,
    enabled: bool,
}

#[derive(Clone)]
struct Snapshot<S: Store> {
    state: S::State,
    action: String,
    timestamp: Instant,
}

impl<S: Store> TimeTravelDebugger<S> {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: 0,
            enabled: cfg!(debug_assertions),
        }
    }

    pub fn record(&mut self, state: S::State, action: &str) {
        if !self.enabled {
            return;
        }

        // Truncate future history
        self.history.truncate(self.current_index + 1);

        self.history.push(Snapshot {
            state,
            action: action.to_string(),
            timestamp: Instant::now(),
        });

        self.current_index = self.history.len() - 1;
    }

    pub fn jump_to(&mut self, index: usize) -> Option<&S::State> {
        if index < self.history.len() {
            self.current_index = index;
            Some(&self.history[index].state)
        } else {
            None
        }
    }

    pub fn step_back(&mut self) -> Option<&S::State> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.history[self.current_index].state)
        } else {
            None
        }
    }

    pub fn step_forward(&mut self) -> Option<&S::State> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            Some(&self.history[self.current_index].state)
        } else {
            None
        }
    }
}
```

**Tasks:**
- [ ] Implement time-travel debugger
- [ ] Add snapshot/restore
- [ ] Create debugger UI
- [ ] Add export/import
- [ ] Write tests

#### Week 11: Additional Middleware

**Day 1-2: Performance Monitor**
```rust
// src/flux/middleware/performance.rs

pub struct PerformanceMonitor {
    metrics: Arc<RwLock<HashMap<String, PerformanceMetrics>>>,
}

#[derive(Default)]
struct PerformanceMetrics {
    count: u64,
    total_duration: Duration,
    max_duration: Duration,
    min_duration: Duration,
}

impl Middleware for PerformanceMonitor {
    fn before_dispatch(&self, action: &dyn Any) -> MiddlewareResult {
        let action_name = format!("{:?}", action);
        let start = Instant::now();

        // Store start time for this dispatch
        MiddlewareResult::Continue
    }

    fn after_dispatch(&self, action: &dyn Any) {
        let duration = /* calculate from start */;
        let action_name = format!("{:?}", action);

        let mut metrics = self.metrics.write().unwrap();
        let entry = metrics.entry(action_name).or_default();
        entry.count += 1;
        entry.total_duration += duration;
        entry.max_duration = entry.max_duration.max(duration);
        entry.min_duration = entry.min_duration.min(duration);
    }
}
```

**Tasks:**
- [ ] Implement performance monitor
- [ ] Track dispatch times
- [ ] Generate reports
- [ ] Add alerts for slow actions
- [ ] Write tests

**Day 3-5: Persistence Middleware**
```rust
// src/flux/middleware/persistence.rs

pub struct PersistenceMiddleware {
    storage_key: String,
}

impl PersistenceMiddleware {
    pub fn new(storage_key: impl Into<String>) -> Self {
        Self {
            storage_key: storage_key.into(),
        }
    }

    fn save_state<S: Serialize>(&self, state: &S) {
        let json = serde_json::to_string(state).unwrap();
        // Save to localStorage or file system
    }

    fn load_state<S: DeserializeOwned>(&self) -> Option<S> {
        // Load from storage
        None
    }
}

impl Middleware for PersistenceMiddleware {
    fn after_dispatch(&self, _action: &dyn Any) {
        // Save state after each action
    }
}
```

**Tasks:**
- [ ] Implement persistence middleware
- [ ] Support localStorage
- [ ] Support file system
- [ ] Add state rehydration
- [ ] Write tests

---

### Phase 6: Documentation & Examples (Week 12)

#### Week 12: Complete Documentation

**Day 1-2: API Documentation**
- [ ] Write comprehensive rustdoc
- [ ] Add code examples to every public item
- [ ] Create architecture guide
- [ ] Document best practices
- [ ] Add troubleshooting guide

**Day 3: Example Applications**

**Counter Example:**
```rust
// examples/flux_counter.rs

define_actions! {
    pub enum CounterAction {
        Increment,
        Decrement,
        Reset,
    }
}

#[derive(Clone)]
pub struct CounterState {
    count: i32,
}

fn counter_reducer(state: &mut CounterState, action: &CounterAction) {
    match action {
        CounterAction::Increment => state.count += 1,
        CounterAction::Decrement => state.count -= 1,
        CounterAction::Reset => state.count = 0,
    }
}

fn main() {
    let dispatcher = Dispatcher::new();
    let store = StoreBuilder::new(CounterState { count: 0 })
        .reducer(counter_reducer)
        .build();

    let container = FluxContainer::new(dispatcher);
    container.add_store(store);

    App::new().run(|cx| {
        cx.set_global(container);
        cx.open_window(WindowOptions::default(), |cx| {
            // Render counter UI
        });
    });
}
```

**Example applications:**
- [ ] Counter
- [ ] Todo List
- [ ] Shopping Cart
- [ ] User Management (CRUD)
- [ ] Real-time Chat
- [ ] Dashboard with multiple stores

**Day 4-5: Migration & Integration Guides**
- [ ] Migration from props-based
- [ ] Integration with existing apps
- [ ] Comparison with TEA
- [ ] Performance optimization guide
- [ ] Testing guide for Flux apps

---

### Phase 7: Testing & Refinement (Week 13)

#### Week 13: Final Testing & Polish

**Day 1-2: Comprehensive Testing**
- [ ] Unit tests (100% coverage goal)
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] Memory leak tests
- [ ] Stress tests

**Day 3: Real-World Application**
- [ ] Build complete application
- [ ] Test all patterns
- [ ] Identify edge cases
- [ ] Optimize performance

**Day 4: Community Feedback**
- [ ] Share with early adopters
- [ ] Collect feedback
- [ ] Address concerns
- [ ] Iterate on ergonomics

**Day 5: Final Polish**
- [ ] Fix discovered bugs
- [ ] Improve error messages
- [ ] Optimize hot paths
- [ ] Finalize documentation
- [ ] Prepare release

---

## Code Examples

### Complete Todo App Example

```rust
// examples/flux_todo_app.rs

use purdah_gpui_components::prelude::*;
use purdah_gpui_components::flux::*;

// 1. Define actions
define_actions! {
    pub enum TodoAction {
        AddTodo { text: String },
        ToggleTodo { id: usize },
        RemoveTodo { id: usize },
        SetFilter { filter: TodoFilter },
    }
}

// 2. Define state
#[derive(Clone, Serialize, Deserialize)]
pub struct TodoState {
    todos: Vec<Todo>,
    filter: TodoFilter,
    next_id: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

// 3. Define reducer
fn todo_reducer(state: &mut TodoState, action: &TodoAction) {
    match action {
        TodoAction::AddTodo { text } => {
            state.todos.push(Todo {
                id: state.next_id,
                text: text.clone(),
                completed: false,
            });
            state.next_id += 1;
        }
        TodoAction::ToggleTodo { id } => {
            if let Some(todo) = state.todos.iter_mut().find(|t| t.id == *id) {
                todo.completed = !todo.completed;
            }
        }
        TodoAction::RemoveTodo { id } => {
            state.todos.retain(|t| t.id != *id);
        }
        TodoAction::SetFilter { filter } => {
            state.filter = *filter;
        }
    }
}

// 4. Create store
type TodoStore = BasicStore<TodoState, TodoAction>;

fn create_todo_store() -> TodoStore {
    StoreBuilder::new(TodoState {
        todos: Vec::new(),
        filter: TodoFilter::All,
        next_id: 1,
    })
    .reducer(todo_reducer)
    .build()
}

// 5. Create views
fn todo_list_view(state: &TodoState, dispatcher: &Dispatcher) -> impl IntoElement {
    let visible_todos: Vec<_> = state
        .todos
        .iter()
        .filter(|todo| match state.filter {
            TodoFilter::All => true,
            TodoFilter::Active => !todo.completed,
            TodoFilter::Completed => todo.completed,
        })
        .collect();

    VStack::new()
        .gap(Spacing::Md)
        .children(visible_todos.iter().map(|todo| {
            todo_item_view(todo, dispatcher)
        }))
}

fn todo_item_view(todo: &Todo, dispatcher: &Dispatcher) -> impl IntoElement {
    HStack::new()
        .gap(Spacing::Sm)
        .child(
            Checkbox::new()
                .checked(todo.completed)
                .on_change(dispatcher.clone(), TodoAction::ToggleTodo { id: todo.id })
        )
        .child(
            Label::new(todo.text.clone())
                .strikethrough(todo.completed)
        )
        .child(
            Button::new()
                .label("×")
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Sm)
                .on_click(dispatcher.clone(), TodoAction::RemoveTodo { id: todo.id })
        )
}

// 6. Main app
fn main() {
    let dispatcher = Dispatcher::new();
    let store = create_todo_store();

    let mut container = FluxContainer::new(Arc::clone(&dispatcher));
    container.add_store(store);

    App::new().run(move |cx| {
        cx.set_global(Arc::new(container));

        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|cx| {
                let connection = Connect::<TodoStore>::connect(cx);

                TodoAppView { connection }
            })
        });
    });
}

struct TodoAppView {
    connection: StoreConnection<TodoStore>,
}

impl Render for TodoAppView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.connection.state();

        VStack::new()
            .gap(Spacing::Lg)
            .child(
                Label::new("Todo App")
                    .size(LabelSize::Xl)
            )
            .child(
                // Add todo input
                HStack::new()
                    .child(
                        Input::new()
                            .placeholder("What needs to be done?")
                            .on_submit(|text| {
                                TodoAction::AddTodo { text }
                            })
                    )
            )
            .child(todo_list_view(&state, &self.connection.dispatcher))
            .child(
                // Filter buttons
                HStack::new()
                    .gap(Spacing::Sm)
                    .child(
                        Button::new()
                            .label("All")
                            .on_click(TodoAction::SetFilter { filter: TodoFilter::All })
                    )
                    .child(
                        Button::new()
                            .label("Active")
                            .on_click(TodoAction::SetFilter { filter: TodoFilter::Active })
                    )
                    .child(
                        Button::new()
                            .label("Completed")
                            .on_click(TodoAction::SetFilter { filter: TodoFilter::Completed })
                    )
            )
    }
}
```

---

## Testing Strategy

### Unit Tests
- Action creation and types
- Reducer pure functions
- Store state management
- Dispatcher dispatch logic
- Middleware execution

### Integration Tests
- Store-dispatcher integration
- Component-store connection
- Async action execution
- Multiple store coordination

### End-to-End Tests
- Complete user flows
- Cross-store communication
- Performance under load

---

## Trade-offs & Risks

### Advantages
✅ Familiar to React/Redux developers
✅ Clear separation of concerns
✅ Excellent DevTools support
✅ Well-documented pattern
✅ Scalable for large apps

### Disadvantages
⚠️ More boilerplate than TEA
⚠️ Mutable stores (less idiomatic Rust)
⚠️ Potential for store sprawl
⚠️ Complexity with multiple stores
⚠️ Learning curve for Rust developers

### Risks
| Risk | Mitigation |
|------|------------|
| Performance overhead | Benchmark and optimize |
| Memory leaks in subscriptions | Automatic cleanup |
| Thread safety issues | Use Arc/RwLock correctly |
| API complexity | Excellent documentation |

---

## Timeline Summary

| Phase | Duration | Key Deliverables |
|-------|----------|------------------|
| 1. Core Infrastructure | 3 weeks | Action, Dispatcher, Store, Middleware |
| 2. Store Architecture | 2 weeks | Multiple stores, composition, async |
| 3. GPUI Integration | 2 weeks | FluxContainer, connections, HOCs |
| 4. Component Updates | 2 weeks | All components Flux-enabled |
| 5. DevTools | 2 weeks | Logger, time-travel, monitoring |
| 6. Documentation | 1 week | Docs, examples, guides |
| 7. Testing | 1 week | Tests, benchmarks, polish |

**Total: 13 weeks**

---

## Success Metrics

- [ ] All components support Flux actions
- [ ] >90% test coverage
- [ ] <10% performance overhead
- [ ] 5+ complete examples
- [ ] Comprehensive documentation
- [ ] Community adoption

---

## Next Steps

1. Review and approve plan
2. Assign development team
3. Setup project infrastructure
4. Begin Phase 1 implementation
5. Iterative review and feedback
