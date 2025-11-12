//! Flux middleware system.

use super::action::Action;
use std::any::Any;

/// Middleware for intercepting Flux actions.
///
/// Middleware can be used for:
/// - Logging actions
/// - Analytics
/// - Async action handling
/// - State persistence
pub trait Middleware: Send + Sync {
    /// Called before an action is dispatched to stores.
    ///
    /// Return `true` to continue dispatch, or `false` to stop it.
    fn before_action(&self, action: &dyn Any) -> bool {
        let _ = action;
        true
    }

    /// Called after an action has been dispatched to all stores.
    fn after_action(&self, action: &dyn Any) {
        let _ = action;
    }
}

/// Logger middleware that prints actions to the console.
pub struct LoggerMiddleware;

impl Middleware for LoggerMiddleware {
    fn before_action(&self, action: &dyn Any) -> bool {
        println!("[Flux Action] {:?}", action);
        true
    }
}

/// Thunk middleware for handling async actions.
///
/// Thunks are functions that can dispatch multiple actions asynchronously.
pub struct ThunkMiddleware;

impl Middleware for ThunkMiddleware {
    fn before_action(&self, _action: &dyn Any) -> bool {
        // TODO: Implement thunk handling
        true
    }
}
