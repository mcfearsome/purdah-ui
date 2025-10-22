//! Atomic components - the smallest reusable UI primitives.
//!
//! Atoms are the basic building blocks of the component library. They are
//! simple, focused components that do one thing well.
//!
//! ## Available Atoms
//!
//! - [`Label`]: Text display with typography variants
//! - [`Button`]: Interactive button with variants and states
//! - [`Input`]: Text input with validation states
//! - `Icon`: SVG icon display (coming soon: Lucide integration)
//!
//! ## Example
//!
//! ```rust,ignore
//! use purdah_gpui_components::atoms::*;
//!
//! // Create a label
//! Label::new("Hello, World!")
//!     .variant(LabelVariant::Heading1);
//!
//! // Create a button
//! Button::new()
//!     .label("Click me")
//!     .variant(ButtonVariant::Primary);
//!
//! // Create an input
//! Input::new()
//!     .placeholder("Enter your name...");
//! ```

pub mod label;
pub mod button;
pub mod input;
// pub mod icon;    // Coming soon: Lucide integration
// pub mod badge;   // Coming soon
// pub mod avatar;  // Coming soon

pub use label::{Label, LabelVariant};
pub use button::{Button, ButtonProps, ButtonSize, ButtonVariant};
pub use input::{Input, InputProps};
