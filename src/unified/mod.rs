//! Unified infrastructure for the Hybrid TEA-Flux architecture.
//!
//! This module provides the core abstractions that enable both The Elm Architecture (TEA)
//! and Flux patterns to coexist and interoperate seamlessly.

pub mod event;
pub mod dispatcher;
pub mod container;
pub mod runtime;

pub use event::Event;
pub use dispatcher::{UnifiedDispatcher, Middleware, HandlerId};
pub use container::{StateContainer, TeaHandle, FluxHandle};
pub use runtime::HybridRuntime;
