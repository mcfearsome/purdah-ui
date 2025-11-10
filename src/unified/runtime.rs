//! Hybrid runtime that manages both TEA and Flux patterns.
//!
//! The runtime provides integration with GPUI's event loop and manages
//! the lifecycle of all state objects.

use super::{StateContainer, UnifiedDispatcher};
use std::sync::Arc;

/// Hybrid runtime that manages both TEA and Flux patterns.
///
/// The runtime coordinates event processing, state updates, and GPUI integration
/// for both architectural patterns.
pub struct HybridRuntime {
    /// The state container that holds all models and stores.
    container: StateContainer,

    /// The unified dispatcher for routing events.
    dispatcher: Arc<UnifiedDispatcher>,
}

impl HybridRuntime {
    /// Creates a new hybrid runtime.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let runtime = HybridRuntime::new();
    /// ```
    pub fn new() -> Arc<Self> {
        let dispatcher = Arc::new(UnifiedDispatcher::new());
        let container = StateContainer::new(Arc::clone(&dispatcher));

        Arc::new(Self {
            container,
            dispatcher,
        })
    }

    /// Gets a reference to the state container.
    pub fn container(&self) -> &StateContainer {
        &self.container
    }

    /// Gets a reference to the unified dispatcher.
    pub fn dispatcher(&self) -> Arc<UnifiedDispatcher> {
        Arc::clone(&self.dispatcher)
    }

    /// Processes all queued events.
    ///
    /// This should be called once per frame to ensure all queued events are handled.
    pub fn process_events(&self) {
        self.dispatcher.process_queue();
    }
}

impl Default for HybridRuntime {
    fn default() -> Arc<Self> {
        Self::new()
    }
}

// Note: GPUI integration will be added in the next phase.
// The setup method will look like:
//
// ```rust,ignore
// impl HybridRuntime {
//     pub fn setup(self: Arc<Self>, cx: &mut WindowContext) {
//         // Store runtime globally
//         cx.set_global(Arc::clone(&self));
//
//         // Setup frame callback for processing events
//         let runtime = Arc::clone(&self);
//         cx.on_next_frame(move |_cx| {
//             runtime.process_events();
//         });
//     }
// }
// ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_runtime() {
        let runtime = HybridRuntime::new();
        assert!(Arc::strong_count(&runtime) >= 1);
    }

    #[test]
    fn test_get_container() {
        let runtime = HybridRuntime::new();
        let _container = runtime.container();
    }

    #[test]
    fn test_get_dispatcher() {
        let runtime = HybridRuntime::new();
        let dispatcher = runtime.dispatcher();
        assert!(Arc::strong_count(&dispatcher) >= 2); // Runtime + our reference
    }
}
