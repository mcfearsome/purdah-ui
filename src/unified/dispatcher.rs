//! Unified dispatcher for handling both TEA messages and Flux actions.
//!
//! The dispatcher manages event handlers for both patterns and routes events
//! to the appropriate handlers based on their type.

use super::event::Event;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::collections::VecDeque;

/// Function type for TEA message handlers.
pub type TeaHandlerFn = Arc<dyn Fn(&dyn Any) + Send + Sync>;

/// Function type for Flux action handlers.
pub type FluxHandlerFn = Arc<dyn Fn(&dyn Any) + Send + Sync>;

/// Handler identifier for unregistering handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandlerId {
    /// TEA handler with type ID and index.
    Tea(TypeId, usize),
    /// Flux handler with type ID and index.
    Flux(TypeId, usize),
}

/// Middleware trait for intercepting events before and after dispatch.
pub trait Middleware: Send + Sync {
    /// Called before an event is dispatched to handlers.
    fn before_dispatch(&self, event: &dyn Any);

    /// Called after an event has been dispatched to all handlers.
    fn after_dispatch(&self, event: &dyn Any);
}

/// Internal state of the unified dispatcher.
struct DispatcherInner {
    /// TEA message handlers, organized by message type.
    tea_handlers: RwLock<HashMap<TypeId, Vec<TeaHandlerFn>>>,

    /// Flux action handlers, organized by action type.
    flux_handlers: RwLock<HashMap<TypeId, Vec<FluxHandlerFn>>>,

    /// Middleware chain for intercepting events.
    middleware: RwLock<Vec<Box<dyn Middleware>>>,

    /// Queue for events that need to be dispatched.
    event_queue: Mutex<VecDeque<Box<dyn Any + Send>>>,
}

/// Unified dispatcher that handles both TEA messages and Flux actions.
///
/// The dispatcher provides a single point for routing events to their appropriate
/// handlers, supporting both TEA and Flux patterns simultaneously.
#[derive(Clone)]
pub struct UnifiedDispatcher {
    inner: Arc<DispatcherInner>,
}

impl UnifiedDispatcher {
    /// Creates a new unified dispatcher.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DispatcherInner {
                tea_handlers: RwLock::new(HashMap::new()),
                flux_handlers: RwLock::new(HashMap::new()),
                middleware: RwLock::new(Vec::new()),
                event_queue: Mutex::new(VecDeque::new()),
            }),
        }
    }

    /// Registers a TEA message handler.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let dispatcher = UnifiedDispatcher::new();
    /// dispatcher.register_tea(|msg: &CounterMsg| {
    ///     println!("Received message: {:?}", msg);
    /// });
    /// ```
    pub fn register_tea<M>(&self, handler: impl Fn(&M) + Send + Sync + 'static) -> HandlerId
    where
        M: 'static,
    {
        let type_id = TypeId::of::<M>();
        let handler: TeaHandlerFn = Arc::new(move |msg| {
            if let Some(typed_msg) = msg.downcast_ref::<M>() {
                handler(typed_msg);
            }
        });

        let mut handlers = self.inner.tea_handlers.write().unwrap();
        let type_handlers = handlers.entry(type_id).or_insert_with(Vec::new);
        type_handlers.push(handler);

        HandlerId::Tea(type_id, type_handlers.len() - 1)
    }

    /// Registers a Flux action handler.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let dispatcher = UnifiedDispatcher::new();
    /// dispatcher.register_flux(|action: &CounterAction| {
    ///     println!("Received action: {:?}", action);
    /// });
    /// ```
    pub fn register_flux<A>(&self, handler: impl Fn(&A) + Send + Sync + 'static) -> HandlerId
    where
        A: 'static,
    {
        let type_id = TypeId::of::<A>();
        let handler: FluxHandlerFn = Arc::new(move |action| {
            if let Some(typed_action) = action.downcast_ref::<A>() {
                handler(typed_action);
            }
        });

        let mut handlers = self.inner.flux_handlers.write().unwrap();
        let type_handlers = handlers.entry(type_id).or_insert_with(Vec::new);
        type_handlers.push(handler);

        HandlerId::Flux(type_id, type_handlers.len() - 1)
    }

    /// Dispatches an event to all registered handlers.
    ///
    /// The event is converted to both TEA messages and Flux actions (if applicable)
    /// and routed to the appropriate handlers.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// dispatcher.dispatch(UserEvent::Login {
    ///     username: "alice".to_string(),
    ///     password: "secret".to_string(),
    /// });
    /// ```
    pub fn dispatch<E: Event>(&self, event: E) {
        // Run middleware before dispatch
        let event_any: Box<dyn Any> = Box::new(event.clone());
        for middleware in self.inner.middleware.read().unwrap().iter() {
            middleware.before_dispatch(event_any.as_ref());
        }

        // Dispatch to TEA handlers
        if let Some(msg) = event.as_message() {
            let type_id = (*msg).type_id();
            if let Some(handlers) = self.inner.tea_handlers.read().unwrap().get(&type_id) {
                for handler in handlers {
                    handler(msg.as_ref());
                }
            }
        }

        // Dispatch to Flux handlers
        if let Some(action) = event.as_action() {
            let type_id = (*action).type_id();
            if let Some(handlers) = self.inner.flux_handlers.read().unwrap().get(&type_id) {
                for handler in handlers {
                    handler(action.as_ref());
                }
            }
        }

        // Run middleware after dispatch
        for middleware in self.inner.middleware.read().unwrap().iter() {
            middleware.after_dispatch(event_any.as_ref());
        }
    }

    /// Adds middleware to the dispatcher.
    ///
    /// Middleware is called before and after every event dispatch.
    pub fn add_middleware(&self, middleware: Box<dyn Middleware>) {
        self.inner.middleware.write().unwrap().push(middleware);
    }

    /// Queues an event for later processing.
    ///
    /// This is useful when events need to be dispatched from contexts where
    /// immediate dispatch is not appropriate (e.g., during rendering).
    pub fn queue_event<E: Event>(&self, event: E) {
        self.inner
            .event_queue
            .lock()
            .unwrap()
            .push_back(Box::new(event));
    }

    /// Processes all queued events.
    ///
    /// This should be called once per frame to ensure all queued events are handled.
    pub fn process_queue(&self) {
        let mut queue = self.inner.event_queue.lock().unwrap();
        while let Some(event_any) = queue.pop_front() {
            // Try to dispatch as a TEA message
            let type_id = (*event_any).type_id();

            if let Some(handlers) = self.inner.tea_handlers.read().unwrap().get(&type_id) {
                for handler in handlers {
                    handler(event_any.as_ref());
                }
            }

            if let Some(handlers) = self.inner.flux_handlers.read().unwrap().get(&type_id) {
                for handler in handlers {
                    handler(event_any.as_ref());
                }
            }
        }
    }

    /// Unregisters a handler by its ID.
    ///
    /// Note: This creates a "hole" in the handler list but doesn't reindex.
    pub fn unregister(&self, handler_id: HandlerId) {
        match handler_id {
            HandlerId::Tea(type_id, index) => {
                if let Some(handlers) = self.inner.tea_handlers.write().unwrap().get_mut(&type_id) {
                    if index < handlers.len() {
                        // Replace with a no-op handler instead of removing
                        handlers[index] = Arc::new(|_| {});
                    }
                }
            }
            HandlerId::Flux(type_id, index) => {
                if let Some(handlers) = self.inner.flux_handlers.write().unwrap().get_mut(&type_id) {
                    if index < handlers.len() {
                        // Replace with a no-op handler instead of removing
                        handlers[index] = Arc::new(|_| {});
                    }
                }
            }
        }
    }
}

impl Default for UnifiedDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Debug, Clone, PartialEq)]
    struct TestMsg {
        value: i32,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestAction {
        value: i32,
    }

    #[test]
    fn test_register_tea_handler() {
        let dispatcher = UnifiedDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        dispatcher.register_tea(move |_msg: &TestMsg| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Manually call the handler
        let msg = Box::new(TestMsg { value: 42 });
        let type_id = (*msg).type_id();
        if let Some(handlers) = dispatcher.inner.tea_handlers.read().unwrap().get(&type_id) {
            for handler in handlers {
                handler(msg.as_ref());
            }
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_register_flux_handler() {
        let dispatcher = UnifiedDispatcher::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        dispatcher.register_flux(move |_action: &TestAction| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Manually call the handler
        let action = Box::new(TestAction { value: 42 });
        let type_id = (*action).type_id();
        if let Some(handlers) = dispatcher.inner.flux_handlers.read().unwrap().get(&type_id) {
            for handler in handlers {
                handler(action.as_ref());
            }
        }

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    struct TestMiddleware {
        before_count: Arc<AtomicUsize>,
        after_count: Arc<AtomicUsize>,
    }

    impl Middleware for TestMiddleware {
        fn before_dispatch(&self, _event: &dyn Any) {
            self.before_count.fetch_add(1, Ordering::SeqCst);
        }

        fn after_dispatch(&self, _event: &dyn Any) {
            self.after_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_middleware() {
        let before_count = Arc::new(AtomicUsize::new(0));
        let after_count = Arc::new(AtomicUsize::new(0));

        let middleware = TestMiddleware {
            before_count: Arc::clone(&before_count),
            after_count: Arc::clone(&after_count),
        };

        let dispatcher = UnifiedDispatcher::new();
        dispatcher.add_middleware(Box::new(middleware));

        // Create a simple event and dispatch it
        #[derive(Clone, Debug)]
        struct SimpleEvent;

        impl Event for SimpleEvent {
            fn event_type(&self) -> &'static str {
                "SimpleEvent"
            }
        }

        dispatcher.dispatch(SimpleEvent);

        assert_eq!(before_count.load(Ordering::SeqCst), 1);
        assert_eq!(after_count.load(Ordering::SeqCst), 1);
    }
}
