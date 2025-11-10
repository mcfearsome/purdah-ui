//! Hybrid Counter Example
//!
//! This example demonstrates the Hybrid TEA-Flux architecture by implementing
//! a simple counter application using the TEA pattern.
//!
//! Run with: cargo run --example hybrid_counter

use purdah_gpui_components::prelude::*;
use purdah_gpui_components::unified::{HybridRuntime};
use purdah_gpui_components::tea::{TeaModel, Command};
use purdah_gpui_components::define_msg;

// Define the counter model using TEA
#[derive(Clone, Debug)]
struct CounterModel {
    count: i32,
}

// Define messages for the counter
define_msg! {
    pub enum CounterMsg {
        Increment,
        Decrement,
        Reset,
        Set { value: i32 },
    }
}

// Implement the TEA model trait
impl TeaModel for CounterModel {
    type State = CounterState;
    type Msg = CounterMsg;

    fn init() -> (Self, Command<Self::Msg>) {
        (Self { count: 0 }, Command::none())
    }

    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg> {
        match msg {
            CounterMsg::Increment => {
                self.count += 1;
            }
            CounterMsg::Decrement => {
                self.count -= 1;
            }
            CounterMsg::Reset => {
                self.count = 0;
            }
            CounterMsg::Set { value } => {
                self.count = value;
            }
        }
        Command::none()
    }

    fn state(&self) -> Self::State {
        CounterState { count: self.count }
    }
}

// The state that will be exposed to the view
#[derive(Clone, Debug)]
struct CounterState {
    count: i32,
}

// Example usage (would be integrated with GPUI in a real app)
fn main() {
    println!("Hybrid Counter Example");
    println!("=====================");
    println!();
    println!("Creating hybrid runtime...");

    let runtime = HybridRuntime::new();

    println!("Adding TEA counter model...");
    let counter = runtime.container().add_tea(CounterModel::init().0);

    println!("Initial state: count = {}", counter.state().count);

    println!("\nDispatching Increment...");
    counter.dispatch(CounterMsg::Increment);
    println!("State: count = {}", counter.state().count);

    println!("\nDispatching Increment...");
    counter.dispatch(CounterMsg::Increment);
    println!("State: count = {}", counter.state().count);

    println!("\nDispatching Decrement...");
    counter.dispatch(CounterMsg::Decrement);
    println!("State: count = {}", counter.state().count);

    println!("\nDispatching Set {{ value: 42 }}...");
    counter.dispatch(CounterMsg::Set { value: 42 });
    println!("State: count = {}", counter.state().count);

    println!("\nDispatching Reset...");
    counter.dispatch(CounterMsg::Reset);
    println!("State: count = {}", counter.state().count);

    println!("\n✅ Hybrid architecture working successfully!");
}
