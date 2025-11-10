//! Flux architecture implementation for Purdah.
//!
//! Flux provides a familiar, Redux-like approach to state management with:
//! - Centralized state stores
//! - Unidirectional data flow
//! - Action-based state updates
//! - Middleware support

pub mod action;
pub mod store;
pub mod middleware;

pub use action::Action;
pub use store::FluxStore;
pub use middleware::Middleware as FluxMiddleware;
