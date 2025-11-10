//! Unified event system that works with both TEA and Flux patterns.
//!
//! This module provides a common event abstraction that can be converted to
//! either TEA messages or Flux actions, enabling seamless interoperability.

use std::any::Any;
use std::fmt::Debug;

/// Unified event trait that works with both TEA and Flux patterns.
///
/// Events can be converted to TEA messages or Flux actions, allowing
/// components to work with either pattern without modification.
pub trait Event: Clone + Send + Sync + Debug + 'static {
    /// Returns a unique type identifier for this event.
    fn event_type(&self) -> &'static str;

    /// Convert to TEA message (if applicable).
    ///
    /// Returns `Some(Box<dyn Any>)` if this event can be converted to a TEA message,
    /// or `None` if the conversion is not supported.
    fn as_message(&self) -> Option<Box<dyn Any>> {
        None
    }

    /// Convert to Flux action (if applicable).
    ///
    /// Returns `Some(Box<dyn Any>)` if this event can be converted to a Flux action,
    /// or `None` if the conversion is not supported.
    fn as_action(&self) -> Option<Box<dyn Any>> {
        None
    }
}

/// Macro for defining unified events that can work with both TEA and Flux.
///
/// # Examples
///
/// ```rust,ignore
/// define_event! {
///     pub enum UserEvent {
///         Login { username: String, password: String },
///         Logout,
///         UpdateProfile { name: String },
///     }
///
///     // Convert to TEA message
///     tea: |event| -> UserMsg {
///         match event {
///             UserEvent::Login { username, password } =>
///                 UserMsg::AttemptLogin { username: username.clone(), password: password.clone() },
///             UserEvent::Logout => UserMsg::Logout,
///             UserEvent::UpdateProfile { name } => UserMsg::UpdateProfile { name: name.clone() },
///         }
///     }
///
///     // Convert to Flux action
///     flux: |event| -> UserAction {
///         match event {
///             UserEvent::Login { username, password } =>
///                 UserAction::Login { username: username.clone(), password: password.clone() },
///             UserEvent::Logout => UserAction::Logout,
///             UserEvent::UpdateProfile { name } => UserAction::UpdateProfile { name: name.clone() },
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_event {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident $({ $($field:ident: $ty:ty),* $(,)? })?
            ),* $(,)?
        }

        $(
            // Optional TEA implementation
            tea: |$tea_self:ident| -> $tea_msg:ty $tea_body:block
        )?

        $(
            // Optional Flux implementation
            flux: |$flux_self:ident| -> $flux_action:ty $flux_body:block
        )?
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant $({ $($field: $ty),* })?
            ),*
        }

        impl $crate::unified::event::Event for $name {
            fn event_type(&self) -> &'static str {
                stringify!($name)
            }

            $(
                fn as_message(&self) -> Option<Box<dyn std::any::Any>> {
                    let $tea_self = self;
                    let msg: $tea_msg = $tea_body;
                    Some(Box::new(msg))
                }
            )?

            $(
                fn as_action(&self) -> Option<Box<dyn std::any::Any>> {
                    let $flux_self = self;
                    let action: $flux_action = $flux_body;
                    Some(Box::new(action))
                }
            )?
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestMsg {
        Increment,
        Decrement,
    }

    #[derive(Debug, Clone, PartialEq)]
    enum TestAction {
        Increment,
        Decrement,
    }

    define_event! {
        pub enum TestEvent {
            Increment,
            Decrement,
        }

        tea: |event| -> TestMsg {
            match event {
                TestEvent::Increment => TestMsg::Increment,
                TestEvent::Decrement => TestMsg::Decrement,
            }
        }

        flux: |event| -> TestAction {
            match event {
                TestEvent::Increment => TestAction::Increment,
                TestEvent::Decrement => TestAction::Decrement,
            }
        }
    }

    #[test]
    fn test_event_type() {
        let event = TestEvent::Increment;
        assert_eq!(event.event_type(), "TestEvent");
    }

    #[test]
    fn test_as_message() {
        let event = TestEvent::Increment;
        let msg = event.as_message();
        assert!(msg.is_some());
        let msg = msg.unwrap().downcast::<TestMsg>().unwrap();
        assert_eq!(*msg, TestMsg::Increment);
    }

    #[test]
    fn test_as_action() {
        let event = TestEvent::Decrement;
        let action = event.as_action();
        assert!(action.is_some());
        let action = action.unwrap().downcast::<TestAction>().unwrap();
        assert_eq!(*action, TestAction::Decrement);
    }
}
