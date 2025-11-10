//! Flux action trait and utilities.

use std::fmt::Debug;

/// Trait for Flux actions.
///
/// Actions represent events that trigger state changes in stores.
pub trait Action: Clone + Send + Sync + Debug + 'static {
    /// Returns the action type identifier.
    fn action_type(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Macro for defining Flux actions.
///
/// # Examples
///
/// ```rust,ignore
/// define_actions! {
///     pub enum CounterAction {
///         Increment,
///         Decrement,
///         Set { value: i32 },
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_actions {
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

        impl $crate::flux::Action for $name {}
    };
}
