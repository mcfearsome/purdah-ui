//! Core TEA model and message traits.

use super::command::Command;
use std::fmt::Debug;

/// Trait for TEA messages.
///
/// Messages represent events or user actions that trigger state updates.
pub trait Message: Clone + Send + Sync + Debug + 'static {}

/// Trait for TEA models.
///
/// Models encapsulate application state and define how that state
/// changes in response to messages.
pub trait TeaModel: Clone + Send + Sync + 'static {
    /// The state type that this model exposes.
    type State: Clone + Send + Sync;

    /// The message type that this model handles.
    type Msg: Message;

    /// Initialize the model with its default state and any initial commands.
    fn init() -> (Self, Command<Self::Msg>);

    /// Update the model in response to a message.
    ///
    /// Returns a command that represents any side effects to be executed.
    fn update(&mut self, msg: Self::Msg) -> Command<Self::Msg>;

    /// Get a snapshot of the current state.
    fn state(&self) -> Self::State;
}

/// Macro for implementing the Message trait on an enum.
///
/// # Examples
///
/// ```rust,ignore
/// define_msg! {
///     pub enum CounterMsg {
///         Increment,
///         Decrement,
///         Reset,
///     }
/// }
/// ```
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
