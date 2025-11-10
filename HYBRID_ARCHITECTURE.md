# Hybrid TEA-Flux Architecture

## Overview

Purdah-UI now includes a **Hybrid TEA-Flux architecture** that combines the best of both The Elm Architecture (TEA) and Flux patterns. This flexible framework allows developers to choose the pattern that best fits their needs while sharing common infrastructure.

## Quick Start

### Using TEA (The Elm Architecture)

TEA is perfect for local component state, functional programming fans, and simpler state management needs.

```rust
use purdah_gpui_components::prelude::*;
use purdah_gpui_components::unified::HybridRuntime;
use purdah_gpui_components::tea::{TeaModel, Command};
use purdah_gpui_components::define_msg;

// Define your model
#[derive(Clone, Debug)]
struct CounterModel {
    count: i32,
}

// Define messages
define_msg! {
    pub enum CounterMsg {
        Increment,
        Decrement,
        Reset,
    }
}

// Implement TEA model
impl TeaModel for CounterModel {
    type State = CounterState;
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

    fn state(&self) -> Self::State {
        CounterState { count: self.count }
    }
}

#[derive(Clone, Debug)]
struct CounterState {
    count: i32,
}

// Use it in your app
fn main() {
    let runtime = HybridRuntime::new();
    let counter = runtime.container().add_tea(CounterModel::init().0);

    // Dispatch messages
    counter.dispatch(CounterMsg::Increment);
    println!("Count: {}", counter.state().count); // Output: Count: 1
}
```

### Using Flux

Flux is perfect for global app state, teams familiar with Redux, and complex state interactions.

```rust
use purdah_gpui_components::prelude::*;
use purdah_gpui_components::unified::HybridRuntime;
use purdah_gpui_components::flux::FluxStore;
use purdah_gpui_components::define_actions;

// Define actions
define_actions! {
    pub enum TodoAction {
        Add { text: String },
        Toggle { id: usize },
        Remove { id: usize },
    }
}

// Define state
#[derive(Clone, Debug)]
struct TodoState {
    todos: Vec<Todo>,
    next_id: usize,
}

#[derive(Clone, Debug)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

// Implement Flux store
struct TodoStore {
    state: TodoState,
}

impl FluxStore for TodoStore {
    type State = TodoState;
    type Action = TodoAction;

    fn state(&self) -> Self::State {
        self.state.clone()
    }

    fn reduce(&mut self, action: &Self::Action) {
        match action {
            TodoAction::Add { text } => {
                self.state.todos.push(Todo {
                    id: self.state.next_id,
                    text: text.clone(),
                    completed: false,
                });
                self.state.next_id += 1;
            }
            TodoAction::Toggle { id } => {
                if let Some(todo) = self.state.todos.iter_mut().find(|t| t.id == *id) {
                    todo.completed = !todo.completed;
                }
            }
            TodoAction::Remove { id } => {
                self.state.todos.retain(|t| t.id != *id);
            }
        }
    }
}

// Use it in your app
fn main() {
    let runtime = HybridRuntime::new();
    let todo_store = runtime.container().add_flux(TodoStore::new());

    // Dispatch actions
    todo_store.dispatch(TodoAction::Add { text: "Learn Rust".to_string() });
    println!("Todos: {}", todo_store.state().todos.len()); // Output: Todos: 1
}
```

### Mixing Both Patterns

The real power comes from using both patterns together in a single application:

```rust
fn main() {
    let runtime = HybridRuntime::new();

    // Use TEA for local UI state
    let ui_state = runtime.container().add_tea(UIModel::init().0);

    // Use Flux for global data
    let user_store = runtime.container().add_flux(UserStore::new());

    // Both work seamlessly together!
    ui_state.dispatch(UIMsg::ToggleSidebar);
    user_store.dispatch(UserAction::Login { username: "alice".to_string() });
}
```

## Architecture Components

### Unified Infrastructure

The hybrid architecture is built on a unified foundation:

- **`UnifiedDispatcher`**: Routes events to both TEA and Flux handlers
- **`StateContainer`**: Manages both TEA models and Flux stores
- **`HybridRuntime`**: Coordinates the entire system and integrates with GPUI
- **`Event` trait**: Common abstraction that can convert to TEA messages or Flux actions

### TEA Components

- **`TeaModel`**: Trait for defining models with pure update functions
- **`Message`**: Trait for TEA messages
- **`Command`**: System for managing side effects
- **`Subscription`**: System for continuous event streams

### Flux Components

- **`FluxStore`**: Trait for defining stores with reduce functions
- **`Action`**: Trait for Flux actions
- **`Middleware`**: System for intercepting actions

## When to Use Which Pattern

### Use TEA When:

- ✅ Managing local component state
- ✅ Building forms with validation
- ✅ You prefer functional programming
- ✅ You want pure, testable functions
- ✅ State is scoped to a single feature

**Examples:** Accordion, modal, multi-step wizard, local filters

### Use Flux When:

- ✅ Managing global application state
- ✅ Multiple components need the same state
- ✅ Your team is familiar with Redux
- ✅ You need complex middleware
- ✅ State is shared across features

**Examples:** User authentication, shopping cart, cached API data, app settings

### Mix Both When:

- ✅ Building large applications
- ✅ You want to optimize per-feature
- ✅ Your team has varied preferences
- ✅ You need progressive complexity

**Examples:** Dashboard with global data (Flux) + local UI state (TEA), E-commerce with cart (Flux) + product filters (TEA)

## Features

### ✨ **Pattern Independence**
TEA and Flux don't depend on each other - use one, the other, or both!

### 🔄 **Interoperability**
Components from both patterns can coexist and communicate seamlessly.

### 📈 **Progressive Complexity**
Start simple with props, add TEA for local state, add Flux for global state.

### 🛡️ **Type Safety**
Full Rust type checking ensures correctness at compile time.

### ⚡ **Zero-Cost Abstraction**
Unused patterns add no runtime overhead.

### 🔧 **Developer Experience**
Helpful macros (`define_msg!`, `define_actions!`, `define_event!`) reduce boilerplate.

## Examples

The repository includes several examples demonstrating the hybrid architecture:

- **`hybrid_counter`**: Simple counter using TEA
- **`flux_todo`**: Todo list using Flux
- More examples coming soon!

Run examples with:
```bash
cargo run --example hybrid_counter
cargo run --example flux_todo
```

## Implementation Status

### ✅ Phase 0: Foundation (Complete)
- Unified event system
- State container
- Runtime integration
- Component abstractions

### 🚧 Phase 1-2: TEA & Flux Core (In Progress)
- Basic TEA and Flux traits implemented
- Command and subscription systems in development
- Middleware system in development

### 📋 Phase 3-6: Coming Soon
- Cross-pattern bridges
- DevTools (unified logger, time-travel debugger)
- Comprehensive examples
- Full documentation

## Contributing

We're actively developing the hybrid architecture! Contributions are welcome:

1. Check the `docs/architecture/HYBRID_TEA_FLUX_PLAN.md` for the full implementation plan
2. Pick a phase or feature to work on
3. Submit a PR with tests and documentation

## Resources

- [Full Implementation Plan](docs/architecture/HYBRID_TEA_FLUX_PLAN.md)
- [TEA Documentation](docs/architecture/TEA_IMPLEMENTATION_PLAN.md)
- [Flux Documentation](docs/architecture/FLUX_IMPLEMENTATION_PLAN.md)

## License

MIT OR Apache-2.0
