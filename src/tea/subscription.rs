//! Subscription system for handling continuous event streams in TEA.

/// A subscription represents a stream of messages over time.
///
/// Subscriptions are used for things like:
/// - Timer ticks
/// - Keyboard events
/// - WebSocket messages
/// - File system watches
pub enum Subscription<Msg> {
    /// No subscription.
    None,

    /// A single subscription.
    Single(Box<dyn SubscriptionExecutor<Msg>>),

    /// Multiple subscriptions running concurrently.
    Batch(Vec<Subscription<Msg>>),
}

impl<Msg> Subscription<Msg> {
    /// Creates a subscription that does nothing.
    pub fn none() -> Self {
        Subscription::None
    }

    /// Creates a subscription from a single executor.
    pub fn single(executor: impl SubscriptionExecutor<Msg> + 'static) -> Self {
        Subscription::Single(Box::new(executor))
    }

    /// Combines multiple subscriptions.
    pub fn batch(subscriptions: Vec<Subscription<Msg>>) -> Self {
        Subscription::Batch(subscriptions)
    }
}

/// Trait for subscription executors.
///
/// Implementors define how to start and stop a subscription.
pub trait SubscriptionExecutor<Msg>: Send + 'static {
    /// Start the subscription.
    ///
    /// Returns a handle that can be used to stop the subscription.
    fn start(self: Box<Self>) -> Box<dyn SubscriptionHandle>;
}

/// Handle to a running subscription.
pub trait SubscriptionHandle: Send {
    /// Stop the subscription.
    fn stop(self: Box<Self>);
}
