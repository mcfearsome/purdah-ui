//! Flux store trait.

use super::action::Action;

/// Trait for Flux stores.
///
/// Stores hold application state and define how that state
/// changes in response to actions.
pub trait FluxStore: Send + Sync + 'static {
    /// The state type that this store manages.
    type State: Clone + Send + Sync;

    /// The action type that this store handles.
    type Action: Action;

    /// Get a snapshot of the current state.
    fn state(&self) -> Self::State;

    /// Reduce the state in response to an action.
    ///
    /// This is the only way to modify the store's state.
    fn reduce(&mut self, action: &Self::Action);
}
