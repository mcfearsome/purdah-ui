//! The Elm Architecture (TEA) implementation for Purdah.
//!
//! TEA provides a functional approach to state management with:
//! - Immutable state updates
//! - Pure update functions
//! - Effect management through Commands
//! - Type-safe message handling

pub mod model;
pub mod command;
pub mod subscription;

pub use model::{TeaModel, Message};
pub use command::{Command, CommandExecutor};
pub use subscription::Subscription;
