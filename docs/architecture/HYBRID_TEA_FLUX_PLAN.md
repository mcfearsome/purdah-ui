# Hybrid TEA-Flux Architecture Implementation Plan for Purdah-UI

## Executive Summary

This document outlines a comprehensive plan to implement a **hybrid architecture** that combines the best features of both The Elm Architecture (TEA) and Flux patterns. This unified approach allows developers to choose the pattern that best fits their needs while sharing common infrastructure.

**Timeline**: 12-15 weeks
**Complexity**: High
**Team Size**: 3-4 developers
**Risk Level**: Medium-High

### Vision

Create a **flexible, dual-mode state management framework** where:
- Developers can choose TEA (functional) or Flux (imperative) per feature
- Both patterns share the same runtime and GPUI integration
- Components work seamlessly with either pattern
- Progressive adoption path from simple to complex

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Philosophy](#core-philosophy)
3. [Unified Infrastructure](#unified-infrastructure)
4. [Implementation Phases](#implementation-phases)
5. [Detailed Phase Breakdown](#detailed-phase-breakdown)
6. [Pattern Selection Guide](#pattern-selection-guide)
7. [Code Examples](#code-examples)
8. [Testing Strategy](#testing-strategy)
9. [Documentation Plan](#documentation-plan)
10. [Trade-offs & Risks](#trade-offs--risks)
11. [Success Metrics](#success-metrics)

---

## Architecture Overview

### Hybrid Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                    Purdah Hybrid Framework                       │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │              Unified Runtime Layer                       │   │
│  │  - Event Loop                                            │   │
│  │  - GPUI Integration                                      │   │
│  │  - Component Registry                                    │   │
│  └─────────────────────────────────────────────────────────┘   │
│                          ▲                                       │
│                          │                                       │
│         ┌────────────────┴────────────────┐                     │
│         │                                  │                     │
│  ┌──────▼──────┐                  ┌───────▼──────┐             │
│  │  TEA Mode   │                  │  Flux Mode   │             │
│  │             │                  │              │             │
│  │ Model       │                  │ Actions      │             │
│  │ Message     │                  │ Dispatcher   │             │
│  │ Update      │                  │ Store        │             │
│  │ Command     │                  │ Middleware   │             │
│  └─────────────┘                  └──────────────┘             │
│         │                                  │                     │
│         └────────────────┬─────────────────┘                    │
│                          │                                       │
│                          ▼                                       │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │           Unified Component System                       │   │
│  │  - Props-based components                                │   │
│  │  - Event handlers (work with both patterns)              │   │
│  │  - Automatic re-rendering                                │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Principles

1. **Unified Foundation**: Single runtime supports both patterns
2. **Pattern Independence**: TEA and Flux don't depend on each other
3. **Interoperability**: Components from both patterns can coexist
4. **Progressive Complexity**: Start simple, add complexity as needed
5. **Type Safety**: Full Rust type checking for both patterns
6. **Zero-Cost Abstraction**: No runtime overhead for unused features

---

## Core Philosophy

### Design Goals

#### 1. **Choice Without Commitment**
```rust
// Use TEA for simple, local state
struct CounterView {
    model: TeaModel<CounterModel>,
}

// Use Flux for complex, global state
struct AppView {
    store: FluxStore<AppStore>,
}

// Mix both in the same application
struct HybridApp {
    counter: TeaModel<CounterModel>,
    user_store: FluxStore<UserStore>,
}
```

#### 2. **Shared Infrastructure**
Both patterns share:
- Component library (Button, Input, etc.)
- GPUI integration layer
- Event system
- Rendering pipeline
- DevTools and debugging

#### 3. **Clear Migration Paths**
- Start with props-based components (Phase 0)
- Add TEA for local state (Phase 1)
- Add Flux for global state (Phase 2)
- Mix and match as needed (Phase 3)

---

## Unified Infrastructure

### 1. Common Event System

```rust
// src/unified/event.rs

/// Unified event trait that works with both TEA and Flux
pub trait Event: Clone + Send + Sync + Debug + 'static {
    /// Event type identifier
    fn event_type(&self) -> &'static str;

    /// Convert to TEA message (if applicable)
    fn as_message(&self) -> Option<Box<dyn Any>> {
        None
    }

    /// Convert to Flux action (if applicable)
    fn as_action(&self) -> Option<Box<dyn Any>> {
        None
    }
}

/// Event dispatcher that works with both patterns
pub struct UnifiedDispatcher {
    tea_handlers: Arc<RwLock<Vec<TeaHandler>>>,
    flux_handlers: Arc<RwLock<Vec<FluxHandler>>>,
}

impl UnifiedDispatcher {
    pub fn dispatch<E: Event>(&self, event: E) {
        // Dispatch to TEA handlers
        if let Some(msg) = event.as_message() {
            for handler in self.tea_handlers.read().unwrap().iter() {
                handler.handle(msg.as_ref());
            }
        }

        // Dispatch to Flux handlers
        if let Some(action) = event.as_action() {
            for handler in self.flux_handlers.read().unwrap().iter() {
                handler.handle(action.as_ref());
            }
        }
    }
}
```

### 2. Unified State Container

```rust
// src/unified/container.rs

/// Container that can hold both TEA models and Flux stores
pub struct StateContainer {
    tea_models: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    flux_stores: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    dispatcher: Arc<UnifiedDispatcher>,
}

impl StateContainer {
    pub fn new() -> Self {
        Self {
            tea_models: HashMap::new(),
            flux_stores: HashMap::new(),
            dispatcher: Arc::new(UnifiedDispatcher::new()),
        }
    }

    /// Add a TEA model
    pub fn add_tea_model<M: TeaModel>(&mut self, model: M) {
        let type_id = TypeId::of::<M>();
        self.tea_models.insert(type_id, Box::new(model));
    }

    /// Add a Flux store
    pub fn add_flux_store<S: FluxStore>(&mut self, store: S) {
        let type_id = TypeId::of::<S>();
        self.flux_stores.insert(type_id, Box::new(store));
    }

    /// Get TEA model
    pub fn get_tea_model<M: TeaModel + 'static>(&self) -> Option<&M> {
        let type_id = TypeId::of::<M>();
        self.tea_models
            .get(&type_id)
            .and_then(|m| m.downcast_ref::<M>())
    }

    /// Get Flux store
    pub fn get_flux_store<S: FluxStore + 'static>(&self) -> Option<&S> {
        let type_id = TypeId::of::<S>();
        self.flux_stores
            .get(&type_id)
            .and_then(|s| s.downcast_ref::<S>())
    }
}
```

### 3. Unified Component System

```rust
// src/unified/component.rs

/// Component that can work with either TEA or Flux
pub trait UnifiedComponent {
    /// Render with access to both patterns
    fn render(&self, context: &RenderContext) -> impl IntoElement;
}

pub struct RenderContext {
    tea_context: TeaContext,
    flux_context: FluxContext,
    dispatcher: Arc<UnifiedDispatcher>,
}

impl RenderContext {
    /// Get TEA model
    pub fn tea_model<M: TeaModel + 'static>(&self) -> Option<&M> {
        self.tea_context.get_model()
    }

    /// Get Flux store state
    pub fn flux_state<S: FluxStore + 'static>(&self) -> Option<S::State> {
        self.flux_context.get_state()
    }

    /// Dispatch event (works with both patterns)
    pub fn dispatch<E: Event>(&self, event: E) {
        self.dispatcher.dispatch(event);
    }
}
```

### 4. Unified Runtime

```rust
// src/unified/runtime.rs

/// Hybrid runtime that manages both TEA and Flux
pub struct HybridRuntime {
    container: StateContainer,
    tea_runtime: TeaRuntime,
    flux_runtime: FluxRuntime,
}

impl HybridRuntime {
    pub fn new() -> Self {
        let container = StateContainer::new();
        let dispatcher = Arc::clone(&container.dispatcher);

        Self {
            tea_runtime: TeaRuntime::new(Arc::clone(&dispatcher)),
            flux_runtime: FluxRuntime::new(Arc::clone(&dispatcher)),
            container,
        }
    }

    /// Process events from both patterns
    pub fn process_events(&mut self) {
        // Process TEA messages
        self.tea_runtime.process_messages();

        // Process Flux actions
        self.flux_runtime.process_actions();
    }

    /// Integrate with GPUI render loop
    pub fn setup_gpui(&self, cx: &mut WindowContext) {
        cx.set_global(Arc::new(self.container.clone()));

        // Setup frame callback
        cx.on_next_frame(|cx| {
            // Process events before each frame
            let runtime = cx.global::<Arc<HybridRuntime>>();
            runtime.process_events();
        });
    }
}
```

---

## Implementation Phases

### Phase 0: Foundation (Weeks 1-2)
**Goal**: Establish unified infrastructure
- Unified event system
- State container
- Runtime integration
- Component abstractions

### Phase 1: TEA Implementation (Weeks 3-5)
**Goal**: Full TEA pattern support
- Model, Message, Update
- Command system
- Subscriptions
- TEA-specific components

### Phase 2: Flux Implementation (Weeks 6-8)
**Goal**: Full Flux pattern support
- Action, Dispatcher, Store
- Middleware system
- Async actions
- Flux-specific utilities

### Phase 3: Integration & Interop (Weeks 9-10)
**Goal**: Seamless pattern mixing
- TEA ↔ Flux bridges
- Shared component library
- Cross-pattern communication
- Conversion utilities

### Phase 4: Developer Experience (Weeks 11-12)
**Goal**: Excellent DX for both patterns
- Unified DevTools
- Pattern selection guide
- Code generation tools
- Migration utilities

### Phase 5: Testing & Examples (Weeks 13-14)
**Goal**: Production-ready quality
- Comprehensive tests
- Example applications
- Performance benchmarks
- Real-world validation

### Phase 6: Documentation & Polish (Week 15)
**Goal**: Release preparation
- Complete documentation
- API stabilization
- Community feedback
- Release candidate

---

## Detailed Phase Breakdown

### Phase 0: Foundation (Weeks 1-2)

#### Week 1: Unified Event System

**Day 1-2: Event Trait**
```rust
// src/unified/event.rs

pub trait Event: Clone + Send + Sync + Debug + 'static {
    fn event_type(&self) -> &'static str;
}

/// Macro for defining unified events
#[macro_export]
macro_rules! define_event {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident $({ $($field:ident: $ty:ty),* $(,)? })?
            ),* $(,)?
        }

        // Optional TEA implementation
        $(tea: |$tea_self:ident| -> $tea_msg:ty $tea_body:block)?

        // Optional Flux implementation
        $(flux: |$flux_self:ident| -> $flux_action:ty $flux_body:block)?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant $({ $($field: $ty),* })?
            ),*
        }

        impl Event for $name {
            fn event_type(&self) -> &'static str {
                stringify!($name)
            }

            $(
                fn as_message(&self) -> Option<Box<dyn Any>> {
                    let $tea_self = self;
                    Some(Box::new($tea_body))
                }
            )?

            $(
                fn as_action(&self) -> Option<Box<dyn Any>> {
                    let $flux_self = self;
                    Some(Box::new($flux_body))
                }
            )?
        }
    };
}
```

**Usage Example:**
```rust
define_event! {
    pub enum UserEvent {
        Login { username: String, password: String },
        Logout,
        UpdateProfile { name: String },
    }

    // Convert to TEA message
    tea: |event| -> UserMsg {
        match event {
            UserEvent::Login { username, password } =>
                UserMsg::AttemptLogin { username: username.clone(), password: password.clone() },
            UserEvent::Logout => UserMsg::Logout,
            UserEvent::UpdateProfile { name } => UserMsg::UpdateProfile { name: name.clone() },
        }
    }

    // Convert to Flux action
    flux: |event| -> UserAction {
        match event {
            UserEvent::Login { username, password } =>
                UserAction::Login { username: username.clone(), password: password.clone() },
            UserEvent::Logout => UserAction::Logout,
            UserEvent::UpdateProfile { name } => UserAction::UpdateProfile { name: name.clone() },
        }
    }
}
```

**Tasks:**
- [ ] Define `Event` trait
- [ ] Create `define_event!` macro
- [ ] Implement conversion traits
- [ ] Write unit tests
- [ ] Document event system

**Day 3-5: Unified Dispatcher**
```rust
// src/unified/dispatcher.rs

pub struct UnifiedDispatcher {
    inner: Arc<DispatcherInner>,
}

struct DispatcherInner {
    tea_handlers: RwLock<HashMap<TypeId, Vec<TeaHandlerFn>>>,
    flux_handlers: RwLock<HashMap<TypeId, Vec<FluxHandlerFn>>>,
    middleware: RwLock<Vec<Box<dyn Middleware>>>,
    event_queue: Mutex<VecDeque<Box<dyn Any + Send>>>,
}

type TeaHandlerFn = Arc<dyn Fn(&dyn Any) + Send + Sync>;
type FluxHandlerFn = Arc<dyn Fn(&dyn Any) + Send + Sync>;

impl UnifiedDispatcher {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DispatcherInner {
                tea_handlers: RwLock::new(HashMap::new()),
                flux_handlers: RwLock::new(HashMap::new()),
                middleware: RwLock::new(Vec::new()),
                event_queue: Mutex::new(VecDeque::new()),
            }),
        }
    }

    /// Register TEA message handler
    pub fn register_tea<M: Message>(
        &self,
        handler: impl Fn(&M) + Send + Sync + 'static,
    ) -> HandlerId {
        let type_id = TypeId::of::<M>();
        let handler: TeaHandlerFn = Arc::new(move |msg| {
            if let Some(typed_msg) = msg.downcast_ref::<M>() {
                handler(typed_msg);
            }
        });

        let mut handlers = self.inner.tea_handlers.write().unwrap();
        let type_handlers = handlers.entry(type_id).or_insert_with(Vec::new);
        type_handlers.push(handler);

        HandlerId::Tea(type_id, type_handlers.len() - 1)
    }

    /// Register Flux action handler
    pub fn register_flux<A: Action>(
        &self,
        handler: impl Fn(&A) + Send + Sync + 'static,
    ) -> HandlerId {
        let type_id = TypeId::of::<A>();
        let handler: FluxHandlerFn = Arc::new(move |action| {
            if let Some(typed_action) = action.downcast_ref::<A>() {
                handler(typed_action);
            }
        });

        let mut handlers = self.inner.flux_handlers.write().unwrap();
        let type_handlers = handlers.entry(type_id).or_insert_with(Vec::new);
        type_handlers.push(handler);

        HandlerId::Flux(type_id, type_handlers.len() - 1)
    }

    /// Dispatch event to all registered handlers
    pub fn dispatch<E: Event>(&self, event: E) {
        // Run middleware
        for middleware in self.inner.middleware.read().unwrap().iter() {
            middleware.before_dispatch(&event);
        }

        // Dispatch to TEA handlers
        if let Some(msg) = event.as_message() {
            let type_id = msg.as_ref().type_id();
            if let Some(handlers) = self.inner.tea_handlers.read().unwrap().get(&type_id) {
                for handler in handlers {
                    handler(msg.as_ref());
                }
            }
        }

        // Dispatch to Flux handlers
        if let Some(action) = event.as_action() {
            let type_id = action.as_ref().type_id();
            if let Some(handlers) = self.inner.flux_handlers.read().unwrap().get(&type_id) {
                for handler in handlers {
                    handler(action.as_ref());
                }
            }
        }

        // Run middleware
        for middleware in self.inner.middleware.read().unwrap().iter() {
            middleware.after_dispatch(&event);
        }
    }

    /// Add middleware
    pub fn add_middleware(&self, middleware: Box<dyn Middleware>) {
        self.inner.middleware.write().unwrap().push(middleware);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HandlerId {
    Tea(TypeId, usize),
    Flux(TypeId, usize),
}

pub trait Middleware: Send + Sync {
    fn before_dispatch(&self, event: &dyn Any);
    fn after_dispatch(&self, event: &dyn Any);
}
```

**Tasks:**
- [ ] Implement unified dispatcher
- [ ] Add handler registration
- [ ] Support both TEA and Flux
- [ ] Add middleware support
- [ ] Write unit tests
- [ ] Benchmark performance

#### Week 2: State Container & Runtime

**Day 1-3: State Container**
```rust
// src/unified/container.rs

pub struct StateContainer {
    tea_models: Arc<RwLock<HashMap<TypeId, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,
    flux_stores: Arc<RwLock<HashMap<TypeId, Arc<RwLock<Box<dyn Any + Send + Sync>>>>>>,
    dispatcher: Arc<UnifiedDispatcher>,
}

impl StateContainer {
    pub fn new(dispatcher: Arc<UnifiedDispatcher>) -> Self {
        Self {
            tea_models: Arc::new(RwLock::new(HashMap::new())),
            flux_stores: Arc::new(RwLock::new(HashMap::new())),
            dispatcher,
        }
    }

    /// Add TEA model
    pub fn add_tea<M>(&self, model: M) -> TeaHandle<M>
    where
        M: TeaModel + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<M>();
        let model_arc = Arc::new(RwLock::new(Box::new(model) as Box<dyn Any + Send + Sync>));

        self.tea_models.write().unwrap().insert(type_id, Arc::clone(&model_arc));

        // Register with dispatcher
        let model_clone = Arc::clone(&model_arc);
        self.dispatcher.register_tea(move |msg: &M::Msg| {
            let mut model = model_clone.write().unwrap();
            if let Some(tea_model) = model.downcast_mut::<M>() {
                tea_model.update(msg);
            }
        });

        TeaHandle {
            model: model_arc,
            dispatcher: Arc::clone(&self.dispatcher),
        }
    }

    /// Add Flux store
    pub fn add_flux<S>(&self, store: S) -> FluxHandle<S>
    where
        S: FluxStore + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<S>();
        let store_arc = Arc::new(RwLock::new(Box::new(store) as Box<dyn Any + Send + Sync>));

        self.flux_stores.write().unwrap().insert(type_id, Arc::clone(&store_arc));

        // Register with dispatcher
        let store_clone = Arc::clone(&store_arc);
        self.dispatcher.register_flux(move |action: &S::Action| {
            let mut store = store_clone.write().unwrap();
            if let Some(flux_store) = store.downcast_mut::<S>() {
                flux_store.reduce(action);
            }
        });

        FluxHandle {
            store: store_arc,
            dispatcher: Arc::clone(&self.dispatcher),
        }
    }

    /// Get TEA model
    pub fn get_tea<M: TeaModel + 'static>(&self) -> Option<TeaHandle<M>> {
        let type_id = TypeId::of::<M>();
        self.tea_models.read().unwrap().get(&type_id).map(|model| {
            TeaHandle {
                model: Arc::clone(model),
                dispatcher: Arc::clone(&self.dispatcher),
            }
        })
    }

    /// Get Flux store
    pub fn get_flux<S: FluxStore + 'static>(&self) -> Option<FluxHandle<S>> {
        let type_id = TypeId::of::<S>();
        self.flux_stores.read().unwrap().get(&type_id).map(|store| {
            FluxHandle {
                store: Arc::clone(store),
                dispatcher: Arc::clone(&self.dispatcher),
            }
        })
    }
}

/// Handle to a TEA model
pub struct TeaHandle<M: TeaModel> {
    model: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
    dispatcher: Arc<UnifiedDispatcher>,
}

impl<M: TeaModel + 'static> TeaHandle<M> {
    pub fn state(&self) -> M::State {
        let model = self.model.read().unwrap();
        let tea_model = model.downcast_ref::<M>().unwrap();
        tea_model.state()
    }

    pub fn dispatch(&self, msg: M::Msg) {
        // Convert to event if possible, or dispatch directly
        self.dispatcher.register_tea(move |msg: &M::Msg| {
            // Handle message
        });
    }
}

/// Handle to a Flux store
pub struct FluxHandle<S: FluxStore> {
    store: Arc<RwLock<Box<dyn Any + Send + Sync>>>,
    dispatcher: Arc<UnifiedDispatcher>,
}

impl<S: FluxStore + 'static> FluxHandle<S> {
    pub fn state(&self) -> S::State {
        let store = self.store.read().unwrap();
        let flux_store = store.downcast_ref::<S>().unwrap();
        flux_store.state()
    }

    pub fn dispatch(&self, action: S::Action) {
        self.dispatcher.dispatch(action);
    }
}
```

**Tasks:**
- [ ] Implement state container
- [ ] Add TEA model management
- [ ] Add Flux store management
- [ ] Create handle types
- [ ] Write unit tests
- [ ] Test thread safety

**Day 4-5: Hybrid Runtime**
```rust
// src/unified/runtime.rs

pub struct HybridRuntime {
    container: StateContainer,
    dispatcher: Arc<UnifiedDispatcher>,
}

impl HybridRuntime {
    pub fn new() -> Arc<Self> {
        let dispatcher = Arc::new(UnifiedDispatcher::new());
        let container = StateContainer::new(Arc::clone(&dispatcher));

        Arc::new(Self {
            container,
            dispatcher,
        })
    }

    /// Setup GPUI integration
    pub fn setup(self: Arc<Self>, cx: &mut WindowContext) {
        // Store runtime globally
        cx.set_global(Arc::clone(&self));

        // Setup frame callback for processing events
        let runtime = Arc::clone(&self);
        cx.observe_global::<FrameTrigger>(move |_view, cx| {
            runtime.process_frame(cx);
        }).detach();
    }

    /// Process events each frame
    fn process_frame(&self, cx: &mut WindowContext) {
        // Process any queued events
        self.dispatcher.process_queue();

        // Notify views that need re-rendering
        cx.notify();
    }

    /// Get container reference
    pub fn container(&self) -> &StateContainer {
        &self.container
    }

    /// Get dispatcher reference
    pub fn dispatcher(&self) -> Arc<UnifiedDispatcher> {
        Arc::clone(&self.dispatcher)
    }
}
```

**Tasks:**
- [ ] Implement hybrid runtime
- [ ] Integrate with GPUI
- [ ] Add frame processing
- [ ] Setup global context
- [ ] Write integration tests

---

### Phase 1: TEA Implementation (Weeks 3-5)

#### Week 3: TEA Core

**Day 1-2: Model & Message**
```rust
// src/tea/model.rs

pub trait TeaModel: Clone + Send + Sync + 'static {
    type State: Clone + Send + Sync;
    type Msg: Message;

    fn init() -> (Self, Command<Self::Msg>);
    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;
    fn state(&self) -> Self::State;
}

pub trait Message: Clone + Send + Sync + Debug + 'static {}

// Implement bridge to unified event system
impl<M: Message> Event for M {
    fn event_type(&self) -> &'static str {
        std::any::type_name::<M>()
    }

    fn as_message(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.clone()))
    }
}
```

**Tasks:**
- [ ] Define TEA traits
- [ ] Implement model management
- [ ] Bridge to unified events
- [ ] Write unit tests
- [ ] Document TEA usage

**Day 3-5: Commands & Subscriptions**
```rust
// src/tea/command.rs

pub enum Command<Msg> {
    None,
    Single(Box<dyn CommandExecutor<Msg>>),
    Batch(Vec<Command<Msg>>),
}

pub trait CommandExecutor<Msg>: Send + 'static {
    fn execute(self: Box<Self>, dispatcher: Arc<UnifiedDispatcher>);
}

// Subscriptions similar to TEA plan
```

**Tasks:**
- [ ] Implement command system
- [ ] Add command executors
- [ ] Implement subscriptions
- [ ] Write integration tests

#### Weeks 4-5: TEA Components & Integration
- Complete TEA implementation as per TEA plan
- Integrate with unified runtime
- Add TEA-specific utilities
- Write comprehensive tests

---

### Phase 2: Flux Implementation (Weeks 6-8)

#### Week 6: Flux Core

**Day 1-3: Actions & Stores**
```rust
// src/flux/action.rs

pub trait Action: Clone + Send + Sync + Debug + 'static {
    fn action_type(&self) -> &'static str;
}

// Implement bridge to unified event system
impl<A: Action> Event for A {
    fn event_type(&self) -> &'static str {
        self.action_type()
    }

    fn as_action(&self) -> Option<Box<dyn Any>> {
        Some(Box::new(self.clone()))
    }
}

// src/flux/store.rs
pub trait FluxStore: Send + Sync + 'static {
    type State: Clone + Send + Sync;
    type Action: Action;

    fn state(&self) -> Self::State;
    fn reduce(&mut self, action: &Self::Action);
}
```

**Tasks:**
- [ ] Define Flux traits
- [ ] Implement store management
- [ ] Bridge to unified events
- [ ] Write unit tests

**Day 4-5: Middleware**
- Implement middleware system
- Add common middleware (logger, thunk, etc.)
- Integrate with unified dispatcher

#### Weeks 7-8: Flux Features & Integration
- Complete Flux implementation as per Flux plan
- Integrate with unified runtime
- Add Flux-specific utilities
- Write comprehensive tests

---

### Phase 3: Integration & Interop (Weeks 9-10)

#### Week 9: Cross-Pattern Communication

**Day 1-3: TEA → Flux Bridges**
```rust
// src/bridges/tea_to_flux.rs

/// Bridge TEA messages to Flux actions
pub struct TeaToFluxBridge<M, A>
where
    M: Message,
    A: Action,
{
    converter: Arc<dyn Fn(&M) -> Option<A> + Send + Sync>,
}

impl<M, A> TeaToFluxBridge<M, A>
where
    M: Message,
    A: Action,
{
    pub fn new(converter: impl Fn(&M) -> Option<A> + Send + Sync + 'static) -> Self {
        Self {
            converter: Arc::new(converter),
        }
    }

    pub fn setup(&self, runtime: &HybridRuntime) {
        let converter = Arc::clone(&self.converter);
        let dispatcher = runtime.dispatcher();

        runtime.dispatcher().register_tea(move |msg: &M| {
            if let Some(action) = converter(msg) {
                dispatcher.dispatch(action);
            }
        });
    }
}

/// Example usage
impl TeaToFluxBridge<CounterMsg, AppAction> {
    pub fn counter_bridge() -> Self {
        Self::new(|msg| match msg {
            CounterMsg::Increment => Some(AppAction::CounterChanged { delta: 1 }),
            CounterMsg::Decrement => Some(AppAction::CounterChanged { delta: -1 }),
            _ => None,
        })
    }
}
```

**Tasks:**
- [ ] Implement TEA → Flux bridge
- [ ] Add automatic conversion helpers
- [ ] Write integration tests
- [ ] Document bridge patterns

**Day 4-5: Flux → TEA Bridges**
```rust
// src/bridges/flux_to_tea.rs

/// Bridge Flux actions to TEA messages
pub struct FluxToTeaBridge<A, M>
where
    A: Action,
    M: Message,
{
    converter: Arc<dyn Fn(&A) -> Option<M> + Send + Sync>,
}

// Similar implementation to TeaToFluxBridge
```

**Tasks:**
- [ ] Implement Flux → TEA bridge
- [ ] Add bi-directional bridges
- [ ] Write integration tests
- [ ] Document patterns

#### Week 10: Shared Component Library

**Day 1-3: Unified Component Traits**
```rust
// src/components/unified.rs

/// Component that works with both TEA and Flux
pub trait UnifiedComponent: IntoElement {
    /// Create from TEA context
    fn from_tea<M: TeaModel>(handle: &TeaHandle<M>) -> Self
    where
        Self: Sized;

    /// Create from Flux context
    fn from_flux<S: FluxStore>(handle: &FluxHandle<S>) -> Self
    where
        Self: Sized;
}

/// Button that works with both patterns
pub struct Button<E: Event> {
    props: ButtonProps,
    on_click: Option<E>,
    _phantom: PhantomData<E>,
}

impl<E: Event> Button<E> {
    pub fn new() -> Self {
        Self {
            props: ButtonProps::default(),
            on_click: None,
            _phantom: PhantomData,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.props.label = label.into();
        self
    }

    /// Works with any event (TEA message, Flux action, or unified event)
    pub fn on_click(mut self, event: E) -> Self {
        self.on_click = Some(event);
        self
    }
}

impl<E: Event> IntoElement for Button<E> {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        div()
            .when_some(self.on_click, |div, event| {
                div.on_click(move |_evt, cx| {
                    let runtime = cx.global::<Arc<HybridRuntime>>();
                    runtime.dispatcher().dispatch(event.clone());
                    cx.notify();
                })
            })
            // ... rest of button styling
    }
}
```

**Tasks:**
- [ ] Update all components to unified trait
- [ ] Support both TEA and Flux
- [ ] Maintain backward compatibility
- [ ] Write integration tests
- [ ] Update documentation

**Day 4-5: Component Examples**
- Create examples using both patterns
- Demonstrate pattern mixing
- Show best practices
- Write tutorials

---

### Phase 4: Developer Experience (Weeks 11-12)

#### Week 11: Unified DevTools

**Day 1-3: Unified Logger**
```rust
// src/devtools/logger.rs

pub struct UnifiedLogger {
    tea_events: Arc<RwLock<Vec<TeaLogEntry>>>,
    flux_events: Arc<RwLock<Vec<FluxLogEntry>>>,
    max_history: usize,
}

#[derive(Clone, Debug)]
struct TeaLogEntry {
    message: String,
    model_snapshot: Option<String>,
    timestamp: Instant,
}

#[derive(Clone, Debug)]
struct FluxLogEntry {
    action: String,
    state_snapshot: Option<String>,
    timestamp: Instant,
}

impl Middleware for UnifiedLogger {
    fn before_dispatch(&self, event: &dyn Any) {
        // Log both TEA messages and Flux actions
        if let Some(msg) = event.downcast_ref::<dyn Message>() {
            self.log_tea(msg);
        }
        if let Some(action) = event.downcast_ref::<dyn Action>() {
            self.log_flux(action);
        }
    }

    fn after_dispatch(&self, _event: &dyn Any) {}
}

impl UnifiedLogger {
    /// Export all events in unified timeline
    pub fn export_timeline(&self) -> String {
        let mut timeline = Vec::new();

        // Merge TEA and Flux events by timestamp
        for entry in self.tea_events.read().unwrap().iter() {
            timeline.push(TimelineEntry::Tea(entry.clone()));
        }
        for entry in self.flux_events.read().unwrap().iter() {
            timeline.push(TimelineEntry::Flux(entry.clone()));
        }

        timeline.sort_by_key(|e| e.timestamp());

        serde_json::to_string_pretty(&timeline).unwrap()
    }
}
```

**Tasks:**
- [ ] Implement unified logger
- [ ] Track both patterns
- [ ] Create timeline view
- [ ] Add export functionality
- [ ] Write tests

**Day 4-5: Time-Travel Debugger**
```rust
// src/devtools/time_travel.rs

pub struct UnifiedTimeTravelDebugger {
    tea_snapshots: HashMap<TypeId, Vec<TeaSnapshot>>,
    flux_snapshots: HashMap<TypeId, Vec<FluxSnapshot>>,
    current_index: usize,
}

impl UnifiedTimeTravelDebugger {
    /// Restore to a specific point in time
    pub fn jump_to(&mut self, index: usize, runtime: &HybridRuntime) {
        // Restore both TEA models and Flux stores
        for (type_id, snapshots) in &self.tea_snapshots {
            if let Some(snapshot) = snapshots.get(index) {
                runtime.container().restore_tea(type_id, snapshot);
            }
        }

        for (type_id, snapshots) in &self.flux_snapshots {
            if let Some(snapshot) = snapshots.get(index) {
                runtime.container().restore_flux(type_id, snapshot);
            }
        }

        self.current_index = index;
    }
}
```

**Tasks:**
- [ ] Implement time-travel for both patterns
- [ ] Add snapshot management
- [ ] Create debugger UI
- [ ] Write integration tests

#### Week 12: Code Generation & Tooling

**Day 1-3: Pattern Scaffolding**
```rust
// src/cli/scaffold.rs

/// CLI tool for generating boilerplate
pub struct Scaffolder {
    pattern: Pattern,
}

pub enum Pattern {
    Tea,
    Flux,
    Hybrid,
}

impl Scaffolder {
    /// Generate TEA module
    pub fn scaffold_tea(&self, name: &str) -> Result<Vec<(PathBuf, String)>> {
        let files = vec![
            (
                PathBuf::from(format!("src/{}/model.rs", name)),
                self.tea_model_template(name),
            ),
            (
                PathBuf::from(format!("src/{}/message.rs", name)),
                self.tea_message_template(name),
            ),
            (
                PathBuf::from(format!("src/{}/view.rs", name)),
                self.tea_view_template(name),
            ),
        ];
        Ok(files)
    }

    /// Generate Flux module
    pub fn scaffold_flux(&self, name: &str) -> Result<Vec<(PathBuf, String)>> {
        let files = vec![
            (
                PathBuf::from(format!("src/{}/actions.rs", name)),
                self.flux_action_template(name),
            ),
            (
                PathBuf::from(format!("src/{}/store.rs", name)),
                self.flux_store_template(name),
            ),
            (
                PathBuf::from(format!("src/{}/view.rs", name)),
                self.flux_view_template(name),
            ),
        ];
        Ok(files)
    }

    /// Generate hybrid module
    pub fn scaffold_hybrid(&self, name: &str) -> Result<Vec<(PathBuf, String)>> {
        // Combine both patterns
        let mut files = self.scaffold_tea(name)?;
        files.extend(self.scaffold_flux(name)?);
        files.push((
            PathBuf::from(format!("src/{}/bridge.rs", name)),
            self.bridge_template(name),
        ));
        Ok(files)
    }
}
```

**Tasks:**
- [ ] Create scaffold CLI tool
- [ ] Add templates for both patterns
- [ ] Support hybrid scaffolding
- [ ] Add migration tools
- [ ] Write documentation

**Day 4-5: Migration Utilities**
```rust
// src/migration/mod.rs

/// Convert props-based component to TEA
pub fn migrate_to_tea(component_path: &Path) -> Result<String> {
    // Parse component code
    // Identify state
    // Generate Model, Message, Update
    // Generate new component code
}

/// Convert props-based component to Flux
pub fn migrate_to_flux(component_path: &Path) -> Result<String> {
    // Parse component code
    // Identify state
    // Generate Actions, Store
    // Generate new component code
}
```

**Tasks:**
- [ ] Create migration utilities
- [ ] Add AST parsing
- [ ] Generate pattern code
- [ ] Write tests
- [ ] Document migration process

---

### Phase 5: Testing & Examples (Weeks 13-14)

#### Week 13: Comprehensive Testing

**Day 1-2: Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_dispatcher_tea() {
        let dispatcher = UnifiedDispatcher::new();
        let mut received = Vec::new();

        dispatcher.register_tea(|msg: &TestMsg| {
            received.push(msg.clone());
        });

        dispatcher.dispatch(TestMsg::Increment);
        assert_eq!(received.len(), 1);
    }

    #[test]
    fn test_unified_dispatcher_flux() {
        let dispatcher = UnifiedDispatcher::new();
        let mut received = Vec::new();

        dispatcher.register_flux(|action: &TestAction| {
            received.push(action.clone());
        });

        dispatcher.dispatch(TestAction::Increment);
        assert_eq!(received.len(), 1);
    }

    #[test]
    fn test_pattern_mixing() {
        // Test TEA and Flux working together
    }
}
```

**Testing categories:**
- [ ] Unified event system
- [ ] State container
- [ ] Runtime integration
- [ ] TEA-specific features
- [ ] Flux-specific features
- [ ] Cross-pattern communication
- [ ] GPUI integration

**Day 3-4: Integration Tests**
```rust
#[gpui::test]
async fn test_hybrid_app(cx: &mut TestAppContext) {
    let runtime = HybridRuntime::new();

    // Add TEA model
    let counter_handle = runtime.container().add_tea(CounterModel::init().0);

    // Add Flux store
    let user_handle = runtime.container().add_flux(UserStore::new());

    // Test interaction
    counter_handle.dispatch(CounterMsg::Increment);
    user_handle.dispatch(UserAction::Login { /* ... */ });

    // Verify state changes
    assert_eq!(counter_handle.state().count, 1);
    assert!(user_handle.state().logged_in);
}
```

**Tasks:**
- [ ] Write integration tests
- [ ] Test GPUI integration
- [ ] Test pattern interaction
- [ ] Test component library
- [ ] Performance benchmarks

**Day 5: Performance Benchmarks**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_unified_dispatch(c: &mut Criterion) {
    c.bench_function("dispatch 10000 events", |b| {
        let runtime = HybridRuntime::new();
        b.iter(|| {
            for _ in 0..10000 {
                runtime.dispatcher().dispatch(black_box(TestEvent::Ping));
            }
        });
    });
}

fn bench_tea_vs_flux(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_comparison");

    group.bench_function("TEA update", |b| {
        // Benchmark TEA
    });

    group.bench_function("Flux reduce", |b| {
        // Benchmark Flux
    });

    group.finish();
}

criterion_group!(benches, bench_unified_dispatch, bench_tea_vs_flux);
criterion_main!(benches);
```

**Tasks:**
- [ ] Benchmark dispatch performance
- [ ] Compare TEA vs Flux overhead
- [ ] Measure memory usage
- [ ] Optimize hot paths
- [ ] Document performance

#### Week 14: Example Applications

**Complete Example Apps:**

1. **Counter (TEA)** - Simple local state
2. **Todo List (Flux)** - Global state management
3. **Hybrid Dashboard** - Mix both patterns
4. **Shopping Cart (Flux + TEA)** - Complex state with local UI state
5. **Chat Application (Hybrid)** - Real-time with subscriptions

**Example: Hybrid Dashboard**
```rust
// examples/hybrid_dashboard/main.rs

// TEA for UI state (sidebar open/closed)
#[derive(Clone)]
struct UIModel {
    sidebar_open: bool,
    theme: ThemeMode,
}

impl TeaModel for UIModel {
    type State = UIState;
    type Msg = UIMsg;

    fn init() -> (Self, Command<Self::Msg>) {
        (Self { sidebar_open: true, theme: ThemeMode::Light }, Command::none())
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            UIMsg::ToggleSidebar => self.sidebar_open = !self.sidebar_open,
            UIMsg::SetTheme(theme) => self.theme = theme,
        }
        Command::none()
    }

    fn state(&self) -> UIState {
        UIState {
            sidebar_open: self.sidebar_open,
            theme: self.theme,
        }
    }
}

// Flux for data state (users, stats)
#[derive(Clone)]
struct DataState {
    users: Vec<User>,
    stats: Stats,
    loading: bool,
}

struct DataStore {
    state: DataState,
}

impl FluxStore for DataStore {
    type State = DataState;
    type Action = DataAction;

    fn state(&self) -> DataState {
        self.state.clone()
    }

    fn reduce(&mut self, action: &DataAction) {
        match action {
            DataAction::FetchUsers => self.state.loading = true,
            DataAction::UsersLoaded(users) => {
                self.state.users = users.clone();
                self.state.loading = false;
            }
            // ...
        }
    }
}

// Main app combines both
fn main() {
    let runtime = HybridRuntime::new();

    // Setup TEA model for UI
    let ui_handle = runtime.container().add_tea(UIModel::init().0);

    // Setup Flux store for data
    let data_handle = runtime.container().add_flux(DataStore::new());

    App::new().run(move |cx| {
        runtime.setup(cx);

        cx.open_window(WindowOptions::default(), |cx| {
            DashboardView::new(ui_handle, data_handle, cx)
        });
    });
}
```

**Tasks:**
- [ ] Build 5 complete example apps
- [ ] Document each example
- [ ] Show pattern selection reasoning
- [ ] Add inline comments
- [ ] Create video tutorials

---

### Phase 6: Documentation & Polish (Week 15)

#### Week 15: Final Documentation

**Day 1-2: Complete API Docs**
```markdown
# Purdah Hybrid Framework Documentation

## Quick Start

### Choosing a Pattern

**Use TEA when:**
- Local component state
- Functional programming preference
- Simpler state management needs
- Want pure functions and testability

**Use Flux when:**
- Global application state
- Multiple components need same state
- Complex state interactions
- Team familiar with Redux

**Use Both when:**
- Large application with varied needs
- Want to optimize per-feature
- Need flexibility

## Basic Usage

### TEA Example
...

### Flux Example
...

### Hybrid Example
...
```

**Tasks:**
- [ ] Write complete API documentation
- [ ] Create pattern selection guide
- [ ] Document all examples
- [ ] Add troubleshooting guide
- [ ] Create cheat sheets

**Day 3-4: Migration Guides**
- [ ] Props-based → TEA migration
- [ ] Props-based → Flux migration
- [ ] Choosing between patterns
- [ ] Mixing patterns effectively
- [ ] Performance optimization

**Day 5: Release Preparation**
- [ ] Final bug fixes
- [ ] API stabilization
- [ ] Release notes
- [ ] Community announcement
- [ ] Launch plan

---

## Pattern Selection Guide

### Decision Matrix

| Criteria | TEA | Flux | Hybrid |
|----------|-----|------|--------|
| **State Scope** | Local | Global | Mixed |
| **Complexity** | Simple-Medium | Medium-High | High |
| **Learning Curve** | Medium | Low (if know Redux) | High |
| **Boilerplate** | Medium | High | Highest |
| **Testability** | Excellent | Good | Excellent |
| **Performance** | Fast | Good | Fast |
| **Debugging** | Time-travel | Redux DevTools | Both |
| **Best For** | UI state, forms | App state, data | Large apps |

### When to Use TEA

✅ **Perfect for:**
- Local component state
- Form validation
- UI state (modals, sidebars, tabs)
- Animations
- Functional programming fans

**Example scenarios:**
- Accordion open/closed state
- Form with validation
- Multi-step wizard
- Local search/filter

### When to Use Flux

✅ **Perfect for:**
- Global app state
- User authentication
- Shopping cart
- Data fetching
- Redux-familiar teams

**Example scenarios:**
- User session management
- Application-wide settings
- Cached API data
- Undo/redo across features

### When to Mix Both

✅ **Perfect for:**
- Large applications
- Teams with varied preferences
- Optimizing per-feature
- Progressive complexity

**Example scenarios:**
- Dashboard with global data (Flux) + local UI state (TEA)
- E-commerce with cart (Flux) + product filters (TEA)
- Admin panel with auth (Flux) + form states (TEA)

---

## Code Examples

### Complete Hybrid Application

```rust
// examples/hybrid_ecommerce/main.rs

use purdah_gpui_components::prelude::*;
use purdah_gpui_components::hybrid::*;

// ===== FLUX: Global Cart Store =====

define_actions! {
    pub enum CartAction {
        AddItem { product: Product },
        RemoveItem { id: ProductId },
        UpdateQuantity { id: ProductId, quantity: u32 },
        Checkout,
        CheckoutSuccess,
        CheckoutError { error: String },
    }
}

#[derive(Clone)]
pub struct CartState {
    items: Vec<CartItem>,
    total: f64,
    checkout_status: CheckoutStatus,
}

pub struct CartStore {
    state: CartState,
}

impl FluxStore for CartStore {
    type State = CartState;
    type Action = CartAction;

    fn state(&self) -> CartState {
        self.state.clone()
    }

    fn reduce(&mut self, action: &CartAction) {
        match action {
            CartAction::AddItem { product } => {
                self.state.items.push(CartItem::from(product.clone()));
                self.recalculate_total();
            }
            CartAction::RemoveItem { id } => {
                self.state.items.retain(|item| item.product.id != *id);
                self.recalculate_total();
            }
            CartAction::UpdateQuantity { id, quantity } => {
                if let Some(item) = self.state.items.iter_mut().find(|i| i.product.id == *id) {
                    item.quantity = *quantity;
                }
                self.recalculate_total();
            }
            CartAction::Checkout => {
                self.state.checkout_status = CheckoutStatus::Processing;
            }
            CartAction::CheckoutSuccess => {
                self.state.items.clear();
                self.state.total = 0.0;
                self.state.checkout_status = CheckoutStatus::Success;
            }
            CartAction::CheckoutError { error } => {
                self.state.checkout_status = CheckoutStatus::Error(error.clone());
            }
        }
    }
}

// ===== TEA: Local Product Filter State =====

define_msg! {
    pub enum FilterMsg {
        SetCategory { category: String },
        SetPriceRange { min: f64, max: f64 },
        SetSearchQuery { query: String },
        Reset,
    }
}

#[derive(Clone)]
pub struct FilterModel {
    category: Option<String>,
    price_range: (f64, f64),
    search_query: String,
}

impl TeaModel for FilterModel {
    type State = FilterState;
    type Msg = FilterMsg;

    fn init() -> (Self, Command<Self::Msg>) {
        (
            Self {
                category: None,
                price_range: (0.0, 1000.0),
                search_query: String::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            FilterMsg::SetCategory { category } => {
                self.category = Some(category);
            }
            FilterMsg::SetPriceRange { min, max } => {
                self.price_range = (min, max);
            }
            FilterMsg::SetSearchQuery { query } => {
                self.search_query = query;
            }
            FilterMsg::Reset => {
                self.category = None;
                self.price_range = (0.0, 1000.0);
                self.search_query.clear();
            }
        }
        Command::none()
    }

    fn state(&self) -> FilterState {
        FilterState {
            category: self.category.clone(),
            price_range: self.price_range,
            search_query: self.search_query.clone(),
        }
    }
}

// ===== Main Application =====

struct EcommerceApp {
    runtime: Arc<HybridRuntime>,
    cart: FluxHandle<CartStore>,
    filters: TeaHandle<FilterModel>,
}

impl EcommerceApp {
    fn new(cx: &mut WindowContext) -> View<Self> {
        let runtime = HybridRuntime::new();
        runtime.clone().setup(cx);

        let cart = runtime.container().add_flux(CartStore::new());
        let filters = runtime.container().add_tea(FilterModel::init().0);

        cx.new_view(|_cx| Self {
            runtime,
            cart,
            filters,
        })
    }
}

impl Render for EcommerceApp {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        let cart_state = self.cart.state();
        let filter_state = self.filters.state();
        let dispatcher = self.runtime.dispatcher();

        VStack::new()
            .child(
                // Header with cart
                HStack::new()
                    .child(Label::new("E-Commerce Store"))
                    .child(
                        Button::new()
                            .label(format!("Cart ({})", cart_state.items.len()))
                            .on_click(CartAction::Checkout)
                    )
            )
            .child(
                HStack::new()
                    .child(
                        // Sidebar with TEA filters
                        VStack::new()
                            .child(Label::new("Filters"))
                            .child(
                                Input::new()
                                    .placeholder("Search...")
                                    .value(filter_state.search_query.clone())
                                    .on_change(|query| FilterMsg::SetSearchQuery { query })
                            )
                            .child(
                                // Category select
                                Select::new()
                                    .on_change(|cat| FilterMsg::SetCategory { category: cat })
                            )
                            .child(
                                Button::new()
                                    .label("Reset Filters")
                                    .on_click(FilterMsg::Reset)
                            )
                    )
                    .child(
                        // Product grid (filtered)
                        product_grid(&filter_state, &dispatcher)
                    )
            )
    }
}

fn product_grid(filters: &FilterState, dispatcher: &UnifiedDispatcher) -> impl IntoElement {
    let products = get_filtered_products(filters);

    VStack::new()
        .children(products.iter().map(|product| {
            HStack::new()
                .child(Label::new(&product.name))
                .child(Label::new(format!("${}", product.price)))
                .child(
                    Button::new()
                        .label("Add to Cart")
                        .on_click(CartAction::AddItem { product: product.clone() })
                )
        }))
}

fn main() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            EcommerceApp::new(cx)
        });
    });
}
```

---

## Testing Strategy

### Unit Testing
- Unified event system
- State container operations
- TEA model updates
- Flux store reducers
- Pattern bridges

### Integration Testing
- GPUI integration
- Component rendering
- Event dispatching
- State synchronization
- Pattern mixing

### End-to-End Testing
- Complete user flows
- Cross-pattern communication
- Performance under load
- Memory leak detection

---

## Trade-offs & Risks

### Advantages

✅ **Maximum Flexibility**: Choose best pattern per feature
✅ **Progressive Adoption**: Start simple, add complexity as needed
✅ **Team Diversity**: Support different preferences
✅ **Best of Both Worlds**: Combine TEA purity with Flux familiarity
✅ **Future-Proof**: Adapt as requirements change

### Disadvantages

⚠️ **Increased Complexity**: Two patterns to learn
⚠️ **Larger API Surface**: More concepts and abstractions
⚠️ **Higher Initial Cost**: More infrastructure to build
⚠️ **Potential Confusion**: When to use which pattern?
⚠️ **Maintenance Overhead**: Support both patterns forever

### Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| **Pattern confusion** | High | Clear decision guide, linting rules |
| **Over-engineering** | Medium | Start simple, add complexity gradually |
| **Performance overhead** | Medium | Benchmarks, optimization, zero-cost abstractions |
| **Maintenance burden** | High | Comprehensive tests, clear architecture |
| **Team fragmentation** | Medium | Documentation, code reviews, conventions |

---

## Success Metrics

### Technical Metrics
- [ ] Both patterns fully functional
- [ ] >90% test coverage
- [ ] <5% performance overhead vs direct implementation
- [ ] Zero-cost abstraction (unused pattern has no overhead)
- [ ] All components work with both patterns

### User Metrics
- [ ] 70% adoption within 6 months
- [ ] >4.5/5 developer satisfaction
- [ ] <5% bug rate
- [ ] 10+ production apps using framework

### Process Metrics
- [ ] Delivered in 15 weeks
- [ ] 5+ complete examples
- [ ] Comprehensive documentation
- [ ] Active community contributions

---

## Timeline Summary

| Phase | Duration | Key Deliverables |
|-------|----------|------------------|
| 0. Foundation | 2 weeks | Unified infrastructure |
| 1. TEA | 3 weeks | Complete TEA implementation |
| 2. Flux | 3 weeks | Complete Flux implementation |
| 3. Integration | 2 weeks | Pattern bridges, shared components |
| 4. Developer Experience | 2 weeks | DevTools, scaffolding, tooling |
| 5. Testing & Examples | 2 weeks | Tests, benchmarks, example apps |
| 6. Documentation | 1 week | Docs, guides, release prep |

**Total: 15 weeks**

---

## Next Steps

1. **Review & Approve**: Team review of hybrid plan
2. **Resource Allocation**: Assign 3-4 developers
3. **Kickoff**: Architecture review and Q&A
4. **Phase 0 Sprint**: Begin unified infrastructure
5. **Iterative Development**: Build, test, refine
6. **Community Preview**: Early access for feedback
7. **Release**: Launch hybrid framework

---

## Conclusion

The hybrid approach combines the best aspects of both TEA and Flux:

- **TEA** brings functional purity, testability, and elegant state management
- **Flux** brings familiarity, scalability, and rich ecosystem
- **Together** they create a flexible framework that adapts to your needs

This is the **most ambitious option** but also the **most future-proof**. It transforms purdah-ui from a component library into a **complete application framework** that rivals any web framework while leveraging Rust's strengths.

### Recommendation

Start with **Phase 0-1** to validate the unified infrastructure and TEA implementation. If successful, add Flux in Phase 2. This allows for:
- Early validation of architecture
- Incremental complexity
- Ability to pivot if needed
- Reduced initial risk

The hybrid approach is **ideal for** teams building the **next generation of GPUI applications** who want maximum flexibility and are willing to invest in a comprehensive framework.
