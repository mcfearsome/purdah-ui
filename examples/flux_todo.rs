//! Flux Todo List Example
//!
//! This example demonstrates the Hybrid TEA-Flux architecture by implementing
//! a simple todo list application using the Flux pattern.
//!
//! Run with: cargo run --example flux_todo

use purdah_gpui_components::prelude::*;
use purdah_gpui_components::unified::HybridRuntime;
use purdah_gpui_components::flux::{FluxStore};
use purdah_gpui_components::define_actions;

// Define the todo item
#[derive(Clone, Debug, PartialEq)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

// Define the store state
#[derive(Clone, Debug)]
struct TodoState {
    todos: Vec<Todo>,
    next_id: usize,
}

// Define actions for the todo list
define_actions! {
    pub enum TodoAction {
        Add { text: String },
        Toggle { id: usize },
        Remove { id: usize },
        Clear,
    }
}

// Implement the Flux store
struct TodoStore {
    state: TodoState,
}

impl TodoStore {
    fn new() -> Self {
        Self {
            state: TodoState {
                todos: Vec::new(),
                next_id: 1,
            },
        }
    }
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
            TodoAction::Clear => {
                self.state.todos.clear();
            }
        }
    }
}

fn main() {
    println!("Flux Todo List Example");
    println!("======================");
    println!();
    println!("Creating hybrid runtime...");

    let runtime = HybridRuntime::new();

    println!("Adding Flux todo store...");
    let todo_store = runtime.container().add_flux(TodoStore::new());

    println!("Initial state: {} todos", todo_store.state().todos.len());

    println!("\nAdding 'Learn Rust'...");
    todo_store.dispatch(TodoAction::Add {
        text: "Learn Rust".to_string(),
    });
    println!("State: {} todos", todo_store.state().todos.len());
    for todo in &todo_store.state().todos {
        println!("  - [{}] {}", if todo.completed { "x" } else { " " }, todo.text);
    }

    println!("\nAdding 'Build GPUI app'...");
    todo_store.dispatch(TodoAction::Add {
        text: "Build GPUI app".to_string(),
    });
    println!("State: {} todos", todo_store.state().todos.len());
    for todo in &todo_store.state().todos {
        println!("  - [{}] {}", if todo.completed { "x" } else { " " }, todo.text);
    }

    println!("\nAdding 'Master Hybrid TEA-Flux'...");
    todo_store.dispatch(TodoAction::Add {
        text: "Master Hybrid TEA-Flux".to_string(),
    });
    println!("State: {} todos", todo_store.state().todos.len());
    for todo in &todo_store.state().todos {
        println!("  - [{}] {}", if todo.completed { "x" } else { " " }, todo.text);
    }

    println!("\nToggling todo #1...");
    todo_store.dispatch(TodoAction::Toggle { id: 1 });
    println!("State: {} todos", todo_store.state().todos.len());
    for todo in &todo_store.state().todos {
        println!("  - [{}] {}", if todo.completed { "x" } else { " " }, todo.text);
    }

    println!("\nRemoving todo #2...");
    todo_store.dispatch(TodoAction::Remove { id: 2 });
    println!("State: {} todos", todo_store.state().todos.len());
    for todo in &todo_store.state().todos {
        println!("  - [{}] {}", if todo.completed { "x" } else { " " }, todo.text);
    }

    println!("\n✅ Flux architecture working successfully!");
}
