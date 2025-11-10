//! Command system for managing side effects in TEA.

use crate::unified::dispatcher::UnifiedDispatcher;
use std::sync::Arc;

/// A command represents a side effect to be executed.
///
/// Commands are returned from the `update` function and executed by the runtime.
pub enum Command<Msg> {
    /// No side effect.
    None,

    /// A single command to execute.
    Single(Box<dyn CommandExecutor<Msg>>),

    /// Multiple commands to execute in sequence.
    Batch(Vec<Command<Msg>>),
}

impl<Msg> Command<Msg> {
    /// Creates a command that does nothing.
    pub fn none() -> Self {
        Command::None
    }

    /// Creates a command from a single executor.
    pub fn single(executor: impl CommandExecutor<Msg> + 'static) -> Self {
        Command::Single(Box::new(executor))
    }

    /// Combines multiple commands into a batch.
    pub fn batch(commands: Vec<Command<Msg>>) -> Self {
        Command::Batch(commands)
    }

    /// Maps the message type of this command.
    pub fn map<NewMsg>(self, _f: impl Fn(Msg) -> NewMsg + 'static) -> Command<NewMsg>
    where
        Msg: 'static,
        NewMsg: 'static,
    {
        // TODO: Implement command mapping
        Command::None
    }
}

/// Trait for command executors.
///
/// Implementors define how to execute a side effect and produce messages.
pub trait CommandExecutor<Msg>: Send + 'static {
    /// Execute the command with access to the dispatcher.
    fn execute(self: Box<Self>, dispatcher: Arc<UnifiedDispatcher>);
}

/// Helper for creating commands from async functions.
pub struct AsyncCommand<Msg, F>
where
    F: FnOnce() -> Msg + Send + 'static,
    Msg: Send + 'static,
{
    func: Option<F>,
}

impl<Msg, F> AsyncCommand<Msg, F>
where
    F: FnOnce() -> Msg + Send + 'static,
    Msg: Send + 'static,
{
    /// Creates a new async command.
    pub fn new(func: F) -> Self {
        Self { func: Some(func) }
    }
}

impl<Msg, F> CommandExecutor<Msg> for AsyncCommand<Msg, F>
where
    F: FnOnce() -> Msg + Send + 'static,
    Msg: Send + 'static,
{
    fn execute(mut self: Box<Self>, _dispatcher: Arc<UnifiedDispatcher>) {
        if let Some(func) = self.func.take() {
            let _msg = func();
            // TODO: Dispatch the message through the dispatcher
        }
    }
}
