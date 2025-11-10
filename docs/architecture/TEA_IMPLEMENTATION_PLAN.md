# The Elm Architecture (TEA) Implementation Plan for Purdah-UI

## Executive Summary

This document outlines a comprehensive plan to implement The Elm Architecture (TEA) pattern in purdah-ui. TEA provides a functional, predictable state management approach with clear unidirectional data flow.

**Timeline**: 8-10 weeks
**Complexity**: Medium-High
**Team Size**: 2-3 developers
**Risk Level**: Medium

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

### TEA Pattern Structure

```
┌─────────────────────────────────────────────────────────┐
│                    Application Loop                      │
│                                                           │
│  ┌──────┐    Msg    ┌────────┐   Model   ┌──────┐      │
│  │      │ ────────> │        │ ────────> │      │      │
│  │ View │           │ Update │           │ View │      │
│  │      │ <──────── │        │ <──────── │      │      │
│  └──────┘   render  └────────┘   state   └──────┘      │
│      │                   │                               │
│      │                   │                               │
│      │                   ▼                               │
│      │              ┌─────────┐                         │
│      └─────────────>│ Command │──> Side Effects         │
│                      └─────────┘    (HTTP, timers, etc) │
└─────────────────────────────────────────────────────────┘
```

### Key Principles

1. **Model**: Single source of truth (immutable state)
2. **Message**: Tagged union of all possible events
3. **Update**: Pure function `(Msg, Model) -> (Model, Command)`
4. **View**: Pure function `Model -> Html`
5. **Command**: Declarative side effects
6. **Subscription**: External event streams

---

## Core Components

### 1. Model System

```rust
// src/tea/model.rs

/// Trait for application state
pub trait Model: Clone + Send + Sync + 'static {
    type Msg: Message;

    /// Initial state
    fn init() -> (Self, Command<Self::Msg>);

    /// State transition
    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;

    /// External event subscriptions
    fn subscriptions(&self) -> Subscription<Self::Msg> {
        Subscription::none()
    }
}
```

### 2. Message System

```rust
// src/tea/message.rs

/// Trait for application messages
pub trait Message: Clone + Send + Sync + Debug + 'static {
    /// Optional message metadata for debugging/logging
    fn metadata(&self) -> MessageMetadata {
        MessageMetadata::default()
    }
}

#[derive(Debug, Clone)]
pub struct MessageMetadata {
    pub timestamp: std::time::Instant,
    pub source: Option<String>,
}
```

### 3. Command System (Side Effects)

```rust
// src/tea/command.rs

/// Declarative side effects
pub enum Command<Msg> {
    None,
    Single(Box<dyn CommandExecutor<Msg>>),
    Batch(Vec<Command<Msg>>),
}

pub trait CommandExecutor<Msg>: Send + 'static {
    fn execute(self: Box<Self>, dispatch: Sender<Msg>);
}

impl<Msg> Command<Msg> {
    pub fn none() -> Self { Command::None }

    pub fn perform<F, Fut>(future: F) -> Self
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Msg> + Send + 'static,
    {
        // Wraps async operations
    }

    pub fn batch(commands: Vec<Command<Msg>>) -> Self {
        Command::Batch(commands)
    }
}
```

### 4. Subscription System

```rust
// src/tea/subscription.rs

/// External event streams
pub enum Subscription<Msg> {
    None,
    Single(Box<dyn SubscriptionHandler<Msg>>),
    Batch(Vec<Subscription<Msg>>),
}

pub trait SubscriptionHandler<Msg>: Send + 'static {
    fn subscribe(&self, dispatch: Sender<Msg>) -> SubscriptionId;
    fn unsubscribe(&self, id: SubscriptionId);
}

impl<Msg> Subscription<Msg> {
    pub fn none() -> Self { Subscription::None }

    pub fn interval(duration: Duration, msg: impl Fn() -> Msg + Send + 'static) -> Self {
        // Timer-based subscriptions
    }

    pub fn keyboard(handler: impl Fn(KeyEvent) -> Option<Msg> + Send + 'static) -> Self {
        // Keyboard event subscriptions
    }
}
```

### 5. Runtime Integration with GPUI

```rust
// src/tea/runtime.rs

/// TEA runtime that integrates with GPUI
pub struct TeaRuntime<M: Model> {
    model: M,
    pending_commands: VecDeque<Command<M::Msg>>,
    subscriptions: Vec<SubscriptionId>,
    message_queue: Receiver<M::Msg>,
    message_sender: Sender<M::Msg>,
}

impl<M: Model> TeaRuntime<M> {
    pub fn new(cx: &mut WindowContext) -> Self {
        let (model, init_command) = M::init();
        let (tx, rx) = channel();

        let mut runtime = Self {
            model,
            pending_commands: VecDeque::new(),
            subscriptions: Vec::new(),
            message_queue: rx,
            message_sender: tx,
        };

        runtime.execute_command(init_command, cx);
        runtime
    }

    pub fn dispatch(&self, msg: M::Msg) {
        self.message_sender.send(msg).ok();
    }

    pub fn process_messages(&mut self, cx: &mut WindowContext) {
        while let Ok(msg) = self.message_queue.try_recv() {
            let command = self.model.update(msg);
            self.execute_command(command, cx);
        }
    }

    fn execute_command(&mut self, command: Command<M::Msg>, cx: &mut WindowContext) {
        match command {
            Command::None => {}
            Command::Single(executor) => {
                executor.execute(self.message_sender.clone());
            }
            Command::Batch(commands) => {
                for cmd in commands {
                    self.execute_command(cmd, cx);
                }
            }
        }
    }
}
```

---

## Implementation Phases

### Phase 1: Foundation (Weeks 1-2)
- Core TEA abstractions (Model, Message, Command, Subscription)
- GPUI integration layer
- Basic runtime implementation
- Unit tests for core types

### Phase 2: Component Integration (Weeks 3-4)
- Event handler traits for components
- Callback system with Message dispatch
- Update all atoms with event support
- Integration tests

### Phase 3: Advanced Features (Weeks 5-6)
- Command executor implementations
- Subscription handlers
- Async effect system
- Performance optimizations

### Phase 4: Developer Experience (Weeks 7-8)
- Debugging tools (message logger, time-travel)
- Documentation and examples
- Migration guides
- Error handling improvements

### Phase 5: Testing & Refinement (Weeks 9-10)
- Comprehensive test suite
- Performance benchmarks
- Real-world example applications
- Community feedback integration

---

## Detailed Phase Breakdown

### Phase 1: Foundation (Weeks 1-2)

#### Week 1: Core Types

**Day 1-2: Message System**
```rust
// src/tea/message.rs

pub trait Message: Clone + Send + Sync + Debug + 'static {}

/// Helper for creating message types
#[macro_export]
macro_rules! define_msg {
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

        impl $crate::tea::Message for $name {}
    };
}
```

**Tasks:**
- [ ] Define `Message` trait
- [ ] Create `define_msg!` macro for ergonomic message definitions
- [ ] Add message metadata support (timestamp, source)
- [ ] Write unit tests for message types

**Day 3-4: Model System**
```rust
// src/tea/model.rs

pub trait Model: Clone + Send + Sync + 'static {
    type Msg: Message;

    fn init() -> (Self, Command<Self::Msg>);
    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;
    fn subscriptions(&self) -> Subscription<Self::Msg> {
        Subscription::none()
    }
}

/// Helper for defining models
#[macro_export]
macro_rules! define_model {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $(pub $field:ident: $ty:ty),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $name {
            $(pub $field: $ty),*
        }
    };
}
```

**Tasks:**
- [ ] Define `Model` trait
- [ ] Create `define_model!` macro
- [ ] Add model validation hooks
- [ ] Write unit tests

**Day 5: Command System Foundation**
```rust
// src/tea/command.rs

pub enum Command<Msg> {
    None,
    Single(Box<dyn CommandExecutor<Msg>>),
    Batch(Vec<Command<Msg>>),
}

pub trait CommandExecutor<Msg>: Send + 'static {
    fn execute(self: Box<Self>, dispatch: Sender<Msg>);
}

impl<Msg: 'static> Command<Msg> {
    pub fn none() -> Self {
        Command::None
    }

    pub fn batch(commands: Vec<Command<Msg>>) -> Self {
        if commands.is_empty() {
            Command::None
        } else {
            Command::Batch(commands)
        }
    }

    pub fn map<NewMsg, F>(self, f: F) -> Command<NewMsg>
    where
        F: Fn(Msg) -> NewMsg + Send + 'static,
        NewMsg: 'static,
    {
        // Command functor for message mapping
    }
}
```

**Tasks:**
- [ ] Define `Command` enum and `CommandExecutor` trait
- [ ] Implement command combinators (none, batch, map)
- [ ] Add error handling for command execution
- [ ] Write unit tests

#### Week 2: Runtime Integration

**Day 1-3: GPUI Runtime**
```rust
// src/tea/runtime.rs

pub struct TeaRuntime<M: Model> {
    model: M,
    message_tx: Sender<M::Msg>,
    message_rx: Receiver<M::Msg>,
    subscriptions: HashMap<SubscriptionId, Box<dyn Any>>,
}

impl<M: Model> TeaRuntime<M> {
    pub fn new() -> Self {
        let (model, init_cmd) = M::init();
        let (tx, rx) = crossbeam_channel::unbounded();

        let mut runtime = Self {
            model,
            message_tx: tx,
            message_rx: rx,
            subscriptions: HashMap::new(),
        };

        runtime.execute_command(init_cmd);
        runtime
    }

    pub fn dispatch(&self, msg: M::Msg) {
        self.message_tx.send(msg).ok();
    }

    pub fn process_messages(&mut self) -> bool {
        let mut updated = false;

        while let Ok(msg) = self.message_rx.try_recv() {
            let command = self.model.update(msg);
            self.execute_command(command);
            updated = true;
        }

        updated
    }

    fn execute_command(&mut self, command: Command<M::Msg>) {
        match command {
            Command::None => {}
            Command::Single(executor) => {
                executor.execute(self.message_tx.clone());
            }
            Command::Batch(commands) => {
                for cmd in commands {
                    self.execute_command(cmd);
                }
            }
        }
    }
}
```

**Tasks:**
- [ ] Implement `TeaRuntime` core structure
- [ ] Add message queue processing
- [ ] Integrate with GPUI's render loop
- [ ] Add runtime lifecycle hooks (start, stop, pause)
- [ ] Write integration tests

**Day 4-5: GPUI View Integration**
```rust
// src/tea/view.rs

/// GPUI view that renders TEA model
pub struct TeaView<M: Model> {
    runtime: Arc<Mutex<TeaRuntime<M>>>,
}

impl<M: Model> TeaView<M> {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        let runtime = Arc::new(Mutex::new(TeaRuntime::new()));

        // Setup render loop integration
        cx.observe_global::<TeaRenderTrigger>(move |_view, cx| {
            // Process messages on each frame
        }).detach();

        cx.new_view(|_cx| Self { runtime })
    }

    pub fn dispatch(&self, msg: M::Msg) {
        self.runtime.lock().unwrap().dispatch(msg);
    }
}

impl<M: Model + TeaRenderable> Render for TeaView<M> {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Process pending messages
        let mut runtime = self.runtime.lock().unwrap();
        if runtime.process_messages() {
            cx.notify(); // Request re-render
        }

        // Render current model state
        runtime.model.view(Dispatcher::new(self.runtime.clone()))
    }
}

/// Trait for models that can render themselves
pub trait TeaRenderable: Model {
    fn view(&self, dispatch: Dispatcher<Self::Msg>) -> impl IntoElement;
}
```

**Tasks:**
- [ ] Create `TeaView` GPUI component
- [ ] Implement `TeaRenderable` trait
- [ ] Add message processing in render loop
- [ ] Test view updates and re-renders
- [ ] Document view lifecycle

---

### Phase 2: Component Integration (Weeks 3-4)

#### Week 3: Event Handler Infrastructure

**Day 1-2: Event Handler Traits**
```rust
// src/tea/events.rs

/// Dispatcher for sending messages from components
#[derive(Clone)]
pub struct Dispatcher<Msg> {
    sender: Sender<Msg>,
}

impl<Msg> Dispatcher<Msg> {
    pub fn new(sender: Sender<Msg>) -> Self {
        Self { sender }
    }

    pub fn dispatch(&self, msg: Msg) {
        self.sender.send(msg).ok();
    }

    pub fn callback<E>(&self, f: impl Fn(E) -> Msg + 'static) -> impl Fn(E) + 'static {
        let sender = self.sender.clone();
        move |event| {
            sender.send(f(event)).ok();
        }
    }
}

/// Event handler builders for components
pub trait EventHandlers {
    type Msg: Message;

    fn on_click(self, dispatch: Dispatcher<Self::Msg>, msg: Self::Msg) -> Self;
    fn on_change(self, dispatch: Dispatcher<Self::Msg>, f: impl Fn(String) -> Self::Msg + 'static) -> Self;
    fn on_submit(self, dispatch: Dispatcher<Self::Msg>, msg: Self::Msg) -> Self;
}
```

**Tasks:**
- [ ] Create `Dispatcher` type for message sending
- [ ] Define `EventHandlers` trait
- [ ] Add callback builders (on_click, on_change, etc.)
- [ ] Write unit tests for dispatching
- [ ] Document event handler patterns

**Day 3-5: Atom Component Updates**

Update each atom component to support event handlers:

```rust
// src/atoms/button.rs (updated)

use crate::tea::{Dispatcher, Message};

pub struct Button<Msg: Message> {
    props: ButtonProps,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    _phantom: PhantomData<Msg>,
}

impl<Msg: Message> Button<Msg> {
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

    pub fn on_click(mut self, dispatch: Dispatcher<Msg>, msg: Msg) -> Self {
        let callback = move || {
            dispatch.dispatch(msg.clone());
        };
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl<Msg: Message> IntoElement for Button<Msg> {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();

        div()
            .id("button")
            .on_click(move |_event, cx| {
                if let Some(handler) = &self.on_click {
                    handler();
                    cx.notify();
                }
            })
            // ... rest of styling
    }
}
```

**Components to update:**
- [ ] Button (on_click)
- [ ] Input (on_change, on_focus, on_blur)
- [ ] Checkbox (on_change)
- [ ] Radio (on_change)
- [ ] Switch (on_toggle)
- [ ] Select (on_change) - if exists

**Tasks per component:**
- [ ] Add generic `Msg` parameter
- [ ] Add event handler fields
- [ ] Implement event builder methods
- [ ] Wire up GPUI event listeners
- [ ] Write integration tests
- [ ] Update documentation

#### Week 4: Molecule & Organism Updates

**Day 1-2: Molecule Updates**

```rust
// src/molecules/form_group.rs (updated)

pub struct FormGroup<Msg: Message> {
    label: Option<SharedString>,
    input: Input<Msg>,
    error_message: Option<SharedString>,
}

impl<Msg: Message> FormGroup<Msg> {
    pub fn new(input: Input<Msg>) -> Self {
        Self {
            label: None,
            input,
            error_message: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}
```

**Tasks:**
- [ ] Update FormGroup to support generic messages
- [ ] Update SearchBar with on_search handler
- [ ] Update Card (no events needed)
- [ ] Write integration tests

**Day 3-5: Organism Updates**

```rust
// src/organisms/dialog.rs (updated)

pub struct Dialog<Msg: Message> {
    title: SharedString,
    content: Box<dyn IntoElement>,
    on_close: Option<Box<dyn Fn() + Send + Sync>>,
    _phantom: PhantomData<Msg>,
}

impl<Msg: Message> Dialog<Msg> {
    pub fn on_close(mut self, dispatch: Dispatcher<Msg>, msg: Msg) -> Self {
        let callback = move || {
            dispatch.dispatch(msg.clone());
        };
        self.on_close = Some(Box::new(callback));
        self
    }
}
```

**Tasks:**
- [ ] Update Dialog with on_close, on_confirm handlers
- [ ] Update Drawer with on_close handler
- [ ] Update Table with on_row_click handler
- [ ] Update CommandPalette with on_select handler
- [ ] Write comprehensive integration tests

---

### Phase 3: Advanced Features (Weeks 5-6)

#### Week 5: Command Executors

**Day 1-2: HTTP Command**
```rust
// src/tea/commands/http.rs

pub struct HttpCommand<Msg> {
    request: HttpRequest,
    on_success: Box<dyn Fn(HttpResponse) -> Msg + Send>,
    on_error: Box<dyn Fn(HttpError) -> Msg + Send>,
}

impl<Msg: Message> HttpCommand<Msg> {
    pub fn get(url: impl Into<String>) -> HttpCommandBuilder<Msg> {
        HttpCommandBuilder::new(Method::GET, url.into())
    }

    pub fn post(url: impl Into<String>) -> HttpCommandBuilder<Msg> {
        HttpCommandBuilder::new(Method::POST, url.into())
    }
}

impl<Msg: Message> CommandExecutor<Msg> for HttpCommand<Msg> {
    fn execute(self: Box<Self>, dispatch: Sender<Msg>) {
        tokio::spawn(async move {
            match self.request.send().await {
                Ok(response) => {
                    dispatch.send((self.on_success)(response)).ok();
                }
                Err(error) => {
                    dispatch.send((self.on_error)(error)).ok();
                }
            }
        });
    }
}
```

**Tasks:**
- [ ] Implement `HttpCommand` executor
- [ ] Add request builder pattern
- [ ] Support GET, POST, PUT, DELETE methods
- [ ] Add timeout and retry logic
- [ ] Write integration tests with mock server

**Day 3-4: Timer & Delay Commands**
```rust
// src/tea/commands/time.rs

pub struct DelayCommand<Msg> {
    duration: Duration,
    message: Msg,
}

impl<Msg: Message> DelayCommand<Msg> {
    pub fn new(duration: Duration, message: Msg) -> Command<Msg> {
        Command::Single(Box::new(Self { duration, message }))
    }
}

impl<Msg: Message> CommandExecutor<Msg> for DelayCommand<Msg> {
    fn execute(self: Box<Self>, dispatch: Sender<Msg>) {
        tokio::spawn(async move {
            tokio::time::sleep(self.duration).await;
            dispatch.send(self.message).ok();
        });
    }
}
```

**Tasks:**
- [ ] Implement `DelayCommand`
- [ ] Add debounce utility
- [ ] Add throttle utility
- [ ] Write unit tests

**Day 5: Local Storage Command**
```rust
// src/tea/commands/storage.rs

pub struct SaveToStorageCommand<Msg> {
    key: String,
    value: serde_json::Value,
    on_success: Box<dyn Fn() -> Msg + Send>,
    on_error: Box<dyn Fn(String) -> Msg + Send>,
}

// Similar implementation for LoadFromStorageCommand
```

**Tasks:**
- [ ] Implement storage commands
- [ ] Add serialization/deserialization
- [ ] Handle storage errors gracefully
- [ ] Write integration tests

#### Week 6: Subscription Handlers

**Day 1-2: Keyboard Subscription**
```rust
// src/tea/subscriptions/keyboard.rs

pub struct KeyboardSubscription<Msg> {
    handler: Arc<dyn Fn(KeyEvent) -> Option<Msg> + Send + Sync>,
}

impl<Msg: Message> KeyboardSubscription<Msg> {
    pub fn new(handler: impl Fn(KeyEvent) -> Option<Msg> + Send + Sync + 'static) -> Subscription<Msg> {
        Subscription::Single(Box::new(Self {
            handler: Arc::new(handler),
        }))
    }
}

impl<Msg: Message> SubscriptionHandler<Msg> for KeyboardSubscription<Msg> {
    fn subscribe(&self, dispatch: Sender<Msg>) -> SubscriptionId {
        // Register GPUI keyboard listener
        // Return subscription ID for cleanup
    }

    fn unsubscribe(&self, id: SubscriptionId) {
        // Remove GPUI keyboard listener
    }
}
```

**Tasks:**
- [ ] Implement keyboard subscription
- [ ] Add key combo matching (Ctrl+S, etc.)
- [ ] Handle platform differences (Cmd vs Ctrl)
- [ ] Write integration tests

**Day 3-4: Mouse & Window Subscriptions**
```rust
// src/tea/subscriptions/mouse.rs
pub struct MouseSubscription<Msg> { /* ... */ }

// src/tea/subscriptions/window.rs
pub struct WindowResizeSubscription<Msg> { /* ... */ }
pub struct WindowFocusSubscription<Msg> { /* ... */ }
```

**Tasks:**
- [ ] Implement mouse position subscription
- [ ] Implement window resize subscription
- [ ] Implement window focus subscription
- [ ] Add subscription combinators (merge, filter)
- [ ] Write integration tests

**Day 5: Interval Subscription**
```rust
// src/tea/subscriptions/time.rs

pub struct IntervalSubscription<Msg> {
    interval: Duration,
    message_fn: Arc<dyn Fn() -> Msg + Send + Sync>,
}

impl<Msg: Message> SubscriptionHandler<Msg> for IntervalSubscription<Msg> {
    fn subscribe(&self, dispatch: Sender<Msg>) -> SubscriptionId {
        let interval = self.interval;
        let msg_fn = Arc::clone(&self.message_fn);

        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            loop {
                interval_timer.tick().await;
                dispatch.send(msg_fn()).ok();
            }
        });

        SubscriptionId::from_handle(handle)
    }
}
```

**Tasks:**
- [ ] Implement interval subscription
- [ ] Add subscription lifecycle management
- [ ] Ensure proper cleanup on unsubscribe
- [ ] Write unit tests

---

### Phase 4: Developer Experience (Weeks 7-8)

#### Week 7: Debugging Tools

**Day 1-3: Message Logger**
```rust
// src/tea/debug/logger.rs

pub struct MessageLogger<M: Model> {
    messages: VecDeque<LogEntry<M::Msg>>,
    max_size: usize,
    enabled: bool,
}

#[derive(Clone, Debug)]
pub struct LogEntry<Msg> {
    pub message: Msg,
    pub timestamp: Instant,
    pub model_snapshot: String, // JSON serialized
}

impl<M: Model> MessageLogger<M> {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_size),
            max_size,
            enabled: cfg!(debug_assertions),
        }
    }

    pub fn log(&mut self, msg: &M::Msg, model: &M)
    where
        M: Serialize,
        M::Msg: Clone,
    {
        if !self.enabled {
            return;
        }

        let entry = LogEntry {
            message: msg.clone(),
            timestamp: Instant::now(),
            model_snapshot: serde_json::to_string_pretty(model).unwrap_or_default(),
        };

        if self.messages.len() >= self.max_size {
            self.messages.pop_front();
        }

        self.messages.push_back(entry);
    }

    pub fn export(&self) -> String {
        // Export as JSON for debugging
    }
}
```

**Tasks:**
- [ ] Implement message logger
- [ ] Add pretty-printing for messages
- [ ] Add filtering by message type
- [ ] Create debug UI panel (optional)
- [ ] Write unit tests

**Day 4-5: Time-Travel Debugging**
```rust
// src/tea/debug/time_travel.rs

pub struct TimeTravelDebugger<M: Model> {
    history: Vec<HistoryEntry<M>>,
    current_index: usize,
}

#[derive(Clone)]
struct HistoryEntry<M: Model> {
    model: M,
    message: M::Msg,
    timestamp: Instant,
}

impl<M: Model> TimeTravelDebugger<M> {
    pub fn record(&mut self, model: M, msg: M::Msg) {
        // Truncate future if we're not at the end
        self.history.truncate(self.current_index + 1);

        self.history.push(HistoryEntry {
            model,
            message: msg,
            timestamp: Instant::now(),
        });

        self.current_index = self.history.len() - 1;
    }

    pub fn step_back(&mut self) -> Option<&M> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.history[self.current_index].model)
        } else {
            None
        }
    }

    pub fn step_forward(&mut self) -> Option<&M> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            Some(&self.history[self.current_index].model)
        } else {
            None
        }
    }

    pub fn jump_to(&mut self, index: usize) -> Option<&M> {
        if index < self.history.len() {
            self.current_index = index;
            Some(&self.history[index].model)
        } else {
            None
        }
    }
}
```

**Tasks:**
- [ ] Implement time-travel debugger
- [ ] Add snapshot/restore functionality
- [ ] Create debugger UI component
- [ ] Add export/import for debugging sessions
- [ ] Write integration tests

#### Week 8: Documentation & Examples

**Day 1-2: API Documentation**
- [ ] Write comprehensive rustdoc for all public APIs
- [ ] Add usage examples to each type
- [ ] Create "Getting Started" guide
- [ ] Document best practices
- [ ] Add troubleshooting section

**Day 3-4: Example Applications**

Create example apps demonstrating TEA patterns:

```rust
// examples/counter.rs - Simple counter

define_msg! {
    pub enum CounterMsg {
        Increment,
        Decrement,
        Reset,
    }
}

define_model! {
    pub struct CounterModel {
        pub count: i32,
    }
}

impl Model for CounterModel {
    type Msg = CounterMsg;

    fn init() -> (Self, Command<Self::Msg>) {
        (Self { count: 0 }, Command::none())
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            CounterMsg::Increment => self.count += 1,
            CounterMsg::Decrement => self.count -= 1,
            CounterMsg::Reset => self.count = 0,
        }
        Command::none()
    }
}

impl TeaRenderable for CounterModel {
    fn view(&self, dispatch: Dispatcher<Self::Msg>) -> impl IntoElement {
        VStack::new()
            .child(Label::new(format!("Count: {}", self.count)))
            .child(
                HStack::new()
                    .child(Button::new().label("-").on_click(dispatch.clone(), CounterMsg::Decrement))
                    .child(Button::new().label("Reset").on_click(dispatch.clone(), CounterMsg::Reset))
                    .child(Button::new().label("+").on_click(dispatch, CounterMsg::Increment))
            )
    }
}
```

**Example applications to create:**
- [ ] Counter (basic state management)
- [ ] Todo List (list management, commands)
- [ ] HTTP Client (async commands, loading states)
- [ ] Form Validation (complex state, validation)
- [ ] Real-time Chat (subscriptions, WebSocket)
- [ ] Timer/Stopwatch (time subscriptions)

**Day 5: Migration Guide**
```markdown
# Migrating to TEA Architecture

## From Props-Based Components

Before (stateless):
...

After (TEA):
...

## Pattern Examples
...

## Common Pitfalls
...
```

**Tasks:**
- [ ] Write migration guide from current pattern
- [ ] Document common migration patterns
- [ ] Add comparison with other architectures
- [ ] Create troubleshooting FAQ
- [ ] Add performance optimization tips

---

### Phase 5: Testing & Refinement (Weeks 9-10)

#### Week 9: Comprehensive Testing

**Day 1-2: Unit Tests**
```rust
// tests/tea/model_tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let (mut model, _) = CounterModel::init();
        model.update(CounterMsg::Increment);
        assert_eq!(model.count, 1);
    }

    #[test]
    fn test_counter_reset() {
        let (mut model, _) = CounterModel::init();
        model.update(CounterMsg::Increment);
        model.update(CounterMsg::Increment);
        model.update(CounterMsg::Reset);
        assert_eq!(model.count, 0);
    }
}
```

**Testing categories:**
- [ ] Model update functions (pure logic)
- [ ] Message dispatching
- [ ] Command execution
- [ ] Subscription lifecycle
- [ ] Runtime message processing

**Day 3-4: Integration Tests**
```rust
// tests/integration/tea_integration.rs

#[gpui::test]
async fn test_button_click_updates_model(cx: &mut TestAppContext) {
    let view = cx.add_window(|cx| TeaView::<CounterModel>::new(cx));

    // Simulate button click
    view.update(cx, |view, cx| {
        view.dispatch(CounterMsg::Increment);
        cx.notify();
    });

    // Assert model updated
    view.read_with(cx, |view, _| {
        assert_eq!(view.runtime.lock().unwrap().model.count, 1);
    });
}
```

**Integration tests:**
- [ ] Button clicks trigger messages
- [ ] Input changes update model
- [ ] Form submissions work correctly
- [ ] Dialog close handlers execute
- [ ] Async commands complete
- [ ] Subscriptions receive events

**Day 5: Performance Benchmarks**
```rust
// benches/tea_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_message_dispatch(c: &mut Criterion) {
    c.bench_function("dispatch 1000 messages", |b| {
        let runtime = TeaRuntime::<CounterModel>::new();

        b.iter(|| {
            for _ in 0..1000 {
                runtime.dispatch(black_box(CounterMsg::Increment));
            }
            runtime.process_messages();
        });
    });
}

criterion_group!(benches, bench_message_dispatch);
criterion_main!(benches);
```

**Benchmarks:**
- [ ] Message dispatch throughput
- [ ] Update function performance
- [ ] Rendering performance
- [ ] Memory usage
- [ ] Command execution overhead

#### Week 10: Real-World Testing & Polish

**Day 1-3: Example Application Development**

Build a complete example application:

```rust
// examples/todo_app/main.rs

// Complex todo app with:
// - Add/edit/delete todos
// - Filter by status
// - Local storage persistence
// - Undo/redo functionality
// - Keyboard shortcuts
```

**Features to implement:**
- [ ] Full CRUD operations
- [ ] State persistence
- [ ] Complex interactions
- [ ] Multiple subscriptions
- [ ] Error handling
- [ ] Loading states

**Day 4: Community Feedback**
- [ ] Share with early adopters
- [ ] Collect feedback on ergonomics
- [ ] Identify pain points
- [ ] Document common questions
- [ ] Create FAQ section

**Day 5: Final Polish**
- [ ] Fix discovered bugs
- [ ] Improve error messages
- [ ] Optimize hot paths
- [ ] Update documentation
- [ ] Prepare release notes

---

## Code Examples

### Complete Counter Example

```rust
// examples/counter_app.rs

use purdah_gpui_components::prelude::*;
use purdah_gpui_components::tea::*;

// 1. Define messages
define_msg! {
    pub enum Msg {
        Increment,
        Decrement,
        Reset,
        IncrementBy { amount: i32 },
    }
}

// 2. Define model
#[derive(Clone)]
pub struct CounterModel {
    count: i32,
}

impl Model for CounterModel {
    type Msg = Msg;

    fn init() -> (Self, Command<Self::Msg>) {
        (Self { count: 0 }, Command::none())
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::Increment => {
                self.count += 1;
            }
            Msg::Decrement => {
                self.count -= 1;
            }
            Msg::Reset => {
                self.count = 0;
            }
            Msg::IncrementBy { amount } => {
                self.count += amount;
            }
        }
        Command::none()
    }
}

// 3. Define view
impl TeaRenderable for CounterModel {
    fn view(&self, dispatch: Dispatcher<Self::Msg>) -> impl IntoElement {
        VStack::new()
            .gap(Spacing::Lg)
            .child(
                Label::new(format!("Count: {}", self.count))
                    .size(LabelSize::Lg)
            )
            .child(
                HStack::new()
                    .gap(Spacing::Md)
                    .child(
                        Button::new()
                            .label("-")
                            .variant(ButtonVariant::Outline)
                            .on_click(dispatch.clone(), Msg::Decrement)
                    )
                    .child(
                        Button::new()
                            .label("Reset")
                            .variant(ButtonVariant::Secondary)
                            .on_click(dispatch.clone(), Msg::Reset)
                    )
                    .child(
                        Button::new()
                            .label("+")
                            .variant(ButtonVariant::Primary)
                            .on_click(dispatch, Msg::Increment)
                    )
            )
    }
}

// 4. Main app
fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            TeaView::<CounterModel>::new(cx)
        });
    });
}
```

### HTTP Request Example

```rust
// examples/http_request.rs

use purdah_gpui_components::tea::*;
use purdah_gpui_components::tea::commands::http::*;

define_msg! {
    pub enum Msg {
        FetchUser,
        UserFetched { user: User },
        FetchError { error: String },
    }
}

#[derive(Clone)]
pub struct UserModel {
    user: Option<User>,
    loading: bool,
    error: Option<String>,
}

impl Model for UserModel {
    type Msg = Msg;

    fn init() -> (Self, Command<Self::Msg>) {
        (
            Self {
                user: None,
                loading: false,
                error: None,
            },
            Command::none()
        )
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::FetchUser => {
                self.loading = true;
                self.error = None;

                // Return HTTP command
                HttpCommand::get("https://api.example.com/user/1")
                    .expect_json::<User>()
                    .on_success(|user| Msg::UserFetched { user })
                    .on_error(|err| Msg::FetchError { error: err.to_string() })
                    .build()
            }
            Msg::UserFetched { user } => {
                self.loading = false;
                self.user = Some(user);
                Command::none()
            }
            Msg::FetchError { error } => {
                self.loading = false;
                self.error = Some(error);
                Command::none()
            }
        }
    }
}

impl TeaRenderable for UserModel {
    fn view(&self, dispatch: Dispatcher<Self::Msg>) -> impl IntoElement {
        VStack::new()
            .child(
                Button::new()
                    .label("Fetch User")
                    .loading(self.loading)
                    .disabled(self.loading)
                    .on_click(dispatch, Msg::FetchUser)
            )
            .when_some(self.user.as_ref(), |stack, user| {
                stack.child(Label::new(format!("Name: {}", user.name)))
            })
            .when_some(self.error.as_ref(), |stack, error| {
                stack.child(
                    Label::new(error.clone())
                        .color(LabelColor::Danger)
                )
            })
    }
}
```

### Form with Validation Example

```rust
// examples/form_validation.rs

define_msg! {
    pub enum Msg {
        EmailChanged { value: String },
        PasswordChanged { value: String },
        Submit,
        SubmitSuccess,
        SubmitError { error: String },
    }
}

#[derive(Clone)]
pub struct FormModel {
    email: String,
    password: String,
    errors: HashMap<String, String>,
    submitting: bool,
}

impl Model for FormModel {
    type Msg = Msg;

    fn init() -> (Self, Command<Self::Msg>) {
        (
            Self {
                email: String::new(),
                password: String::new(),
                errors: HashMap::new(),
                submitting: false,
            },
            Command::none()
        )
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            Msg::EmailChanged { value } => {
                self.email = value;
                self.validate_email();
                Command::none()
            }
            Msg::PasswordChanged { value } => {
                self.password = value;
                self.validate_password();
                Command::none()
            }
            Msg::Submit => {
                if self.validate() {
                    self.submitting = true;

                    HttpCommand::post("https://api.example.com/login")
                        .json(json!({
                            "email": self.email,
                            "password": self.password,
                        }))
                        .on_success(|_| Msg::SubmitSuccess)
                        .on_error(|e| Msg::SubmitError { error: e.to_string() })
                        .build()
                } else {
                    Command::none()
                }
            }
            Msg::SubmitSuccess => {
                self.submitting = false;
                // Navigate to home page
                Command::none()
            }
            Msg::SubmitError { error } => {
                self.submitting = false;
                self.errors.insert("form".into(), error);
                Command::none()
            }
        }
    }
}

impl FormModel {
    fn validate_email(&mut self) -> bool {
        if self.email.is_empty() {
            self.errors.insert("email".into(), "Email is required".into());
            false
        } else if !self.email.contains('@') {
            self.errors.insert("email".into(), "Invalid email format".into());
            false
        } else {
            self.errors.remove("email");
            true
        }
    }

    fn validate_password(&mut self) -> bool {
        if self.password.len() < 8 {
            self.errors.insert("password".into(), "Password must be at least 8 characters".into());
            false
        } else {
            self.errors.remove("password");
            true
        }
    }

    fn validate(&mut self) -> bool {
        let email_valid = self.validate_email();
        let password_valid = self.validate_password();
        email_valid && password_valid
    }
}

impl TeaRenderable for FormModel {
    fn view(&self, dispatch: Dispatcher<Self::Msg>) -> impl IntoElement {
        VStack::new()
            .gap(Spacing::Lg)
            .child(
                FormGroup::new(
                    Input::new()
                        .value(self.email.clone())
                        .placeholder("Email")
                        .error(self.errors.contains_key("email"))
                        .on_change(dispatch.clone(), |value| Msg::EmailChanged { value })
                )
                .label("Email")
                .when_some(self.errors.get("email"), |group, error| {
                    group.error_message(error.clone())
                })
            )
            .child(
                FormGroup::new(
                    Input::new()
                        .value(self.password.clone())
                        .placeholder("Password")
                        .input_type(InputType::Password)
                        .error(self.errors.contains_key("password"))
                        .on_change(dispatch.clone(), |value| Msg::PasswordChanged { value })
                )
                .label("Password")
                .when_some(self.errors.get("password"), |group, error| {
                    group.error_message(error.clone())
                })
            )
            .child(
                Button::new()
                    .label("Submit")
                    .variant(ButtonVariant::Primary)
                    .loading(self.submitting)
                    .disabled(self.submitting || !self.errors.is_empty())
                    .on_click(dispatch, Msg::Submit)
            )
    }
}
```

---

## Testing Strategy

### Unit Testing

**Test Categories:**
1. **Model Updates**: Pure update functions
2. **Message Creation**: Message type correctness
3. **Command Building**: Command construction
4. **Validation Logic**: Business logic

**Example:**
```rust
#[test]
fn test_email_validation() {
    let (mut model, _) = FormModel::init();
    model.update(Msg::EmailChanged { value: "invalid".into() });
    assert!(model.errors.contains_key("email"));

    model.update(Msg::EmailChanged { value: "valid@example.com".into() });
    assert!(!model.errors.contains_key("email"));
}
```

### Integration Testing

**Test Categories:**
1. **Event Handling**: Button clicks, input changes
2. **Command Execution**: Async operations complete
3. **Subscriptions**: External events trigger messages
4. **Rendering**: View updates on state changes

**Example:**
```rust
#[gpui::test]
async fn test_form_submission(cx: &mut TestAppContext) {
    let view = cx.add_window(|cx| TeaView::<FormModel>::new(cx));

    // Fill form
    view.update(cx, |view, _| {
        view.dispatch(Msg::EmailChanged { value: "test@example.com".into() });
        view.dispatch(Msg::PasswordChanged { value: "password123".into() });
    });

    // Submit
    view.update(cx, |view, _| {
        view.dispatch(Msg::Submit);
    });

    // Wait for async command
    cx.run_until_parked();

    // Verify state
    view.read_with(cx, |view, _| {
        let model = &view.runtime.lock().unwrap().model;
        assert!(!model.submitting);
    });
}
```

### Property-Based Testing

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_counter_increments_correctly(increments in 0..1000) {
        let (mut model, _) = CounterModel::init();

        for _ in 0..increments {
            model.update(Msg::Increment);
        }

        assert_eq!(model.count, increments as i32);
    }
}
```

---

## Documentation Plan

### API Documentation

**Structure:**
```
docs/
├── tea/
│   ├── getting-started.md
│   ├── core-concepts.md
│   ├── model.md
│   ├── message.md
│   ├── command.md
│   ├── subscription.md
│   ├── best-practices.md
│   └── examples/
│       ├── counter.md
│       ├── todo-list.md
│       ├── http-requests.md
│       └── form-validation.md
├── api/
│   ├── model-trait.md
│   ├── command-api.md
│   └── subscription-api.md
└── migration/
    ├── from-props-based.md
    └── from-other-frameworks.md
```

### Getting Started Guide

```markdown
# Getting Started with TEA in Purdah-UI

## Your First TEA Application

1. Define your messages
2. Create your model
3. Implement the update function
4. Render your view

[Step-by-step tutorial...]
```

### API Reference

- Full rustdoc for all public types
- Usage examples for each function
- Common patterns and idioms
- Performance considerations

---

## Migration Path

### From Props-Based to TEA

**Step 1: Identify State**
```rust
// Before: State in parent component
struct AppView {
    count: i32,
}

// After: State in TEA model
#[derive(Clone)]
struct AppModel {
    count: i32,
}
```

**Step 2: Convert Events to Messages**
```rust
// Before: Callbacks
Button::new()
    .label("+")
    .on_click(|_| {
        // Direct state mutation
    })

// After: Messages
Button::new()
    .label("+")
    .on_click(dispatch, AppMsg::Increment)
```

**Step 3: Implement Update Logic**
```rust
impl Model for AppModel {
    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            AppMsg::Increment => self.count += 1,
            // ...
        }
        Command::none()
    }
}
```

### Gradual Migration Strategy

1. **Start small**: Migrate one feature at a time
2. **Coexist**: TEA and props-based can coexist
3. **Test thoroughly**: Ensure behavior unchanged
4. **Document changes**: Update team knowledge

---

## Trade-offs & Risks

### Advantages

✅ **Predictability**: All state changes through update function
✅ **Testability**: Pure functions easy to test
✅ **Time-travel debugging**: State history tracking
✅ **Clear data flow**: Unidirectional architecture
✅ **Type safety**: Compiler-checked messages

### Disadvantages

⚠️ **Boilerplate**: Message enums can be verbose
⚠️ **Learning curve**: New mental model for developers
⚠️ **Performance overhead**: Message dispatch and cloning
⚠️ **Async complexity**: Command system adds indirection

### Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| GPUI async limitations | High | Implement custom runtime with tokio |
| Performance regression | Medium | Benchmark and optimize hot paths |
| Breaking API changes | Medium | Version carefully, provide migration tools |
| Adoption resistance | Medium | Excellent docs and examples |
| Maintenance burden | Low | Comprehensive test coverage |

---

## Success Metrics

### Technical Metrics

- [ ] **Test coverage**: >90% for TEA core
- [ ] **Performance**: <5% overhead vs direct mutation
- [ ] **Build time**: <10s increase for TEA dependencies
- [ ] **Documentation**: 100% API coverage

### User Metrics

- [ ] **Adoption rate**: 50% of new projects use TEA in 3 months
- [ ] **Developer satisfaction**: >4/5 rating in surveys
- [ ] **Bug reports**: <10 critical bugs in first 6 months
- [ ] **Community engagement**: 100+ stars on GitHub

### Process Metrics

- [ ] **On-time delivery**: Complete in 10 weeks
- [ ] **Code review velocity**: <2 days average PR review time
- [ ] **Example quality**: 5+ production-quality examples
- [ ] **Migration success**: 3+ existing projects migrated

---

## Timeline Summary

| Phase | Duration | Deliverables |
|-------|----------|--------------|
| 1. Foundation | 2 weeks | Core types, runtime, integration |
| 2. Components | 2 weeks | Event handlers on all components |
| 3. Advanced | 2 weeks | Commands, subscriptions |
| 4. Developer Experience | 2 weeks | Debugging tools, docs, examples |
| 5. Testing & Refinement | 2 weeks | Tests, benchmarks, polish |

**Total: 10 weeks**

---

## Next Steps

1. **Approval**: Review and approve this plan
2. **Team assignment**: Assign 2-3 developers
3. **Kick-off meeting**: Align on goals and timeline
4. **Sprint planning**: Break down Phase 1 into tasks
5. **Begin implementation**: Start with foundation

---

## Appendix A: Alternative Approaches

### Option 1: Minimal TEA (Faster, Less Features)

- Skip subscriptions (use GPUI directly)
- Skip time-travel debugging
- Basic command system only
- **Timeline: 6 weeks**

### Option 2: Full TEA with Effects System (Slower, More Features)

- Full algebraic effects system
- Advanced composition operators
- Plugin architecture for commands
- **Timeline: 14 weeks**

### Option 3: TEA-Lite (Hybrid Approach)

- Message and update only
- No command/subscription abstraction
- Direct async/await in components
- **Timeline: 4 weeks**

---

## Appendix B: Resources

- **Elm Guide**: https://guide.elm-lang.org/architecture/
- **TEA in Rust**: https://github.com/rust-adventure/tea
- **GPUI Docs**: [internal docs]
- **Functional UI Patterns**: Various papers and articles

---

## Questions?

Contact the architecture team or open a discussion in #purdah-ui-architecture.
