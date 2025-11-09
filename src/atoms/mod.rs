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
//! - [`Icon`]: SVG icon display with size and color variants
//! - [`Badge`]: Visual indicator and label component
//! - [`Avatar`]: User profile image with initials fallback
//! - [`Checkbox`]: Form checkbox with indeterminate state
//! - [`Radio`]: Radio button for mutually exclusive selections
//! - [`Switch`]: Toggle switch for binary state control
//! - [`Spinner`]: Loading indicator
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
//!     .variant(ButtonVariant::Primary)
//!     .on_click(|_, cx| { /* handler */ });
//!
//! // Create a badge
//! Badge::new("New")
//!     .variant(BadgeVariant::Success);
//!
//! // Create an avatar
//! Avatar::new("JD")
//!     .size(AvatarSize::Lg)
//!     .status(AvatarStatus::Online);
//! ```

pub mod avatar;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod icon;
pub mod icons; // Icon library constants
pub mod input;
pub mod label;
pub mod radio;
pub mod spinner;
pub mod switch;

pub use avatar::{Avatar, AvatarProps, AvatarSize, AvatarStatus};
pub use badge::{Badge, BadgeProps, BadgeVariant};
pub use button::{Button, ButtonProps, ButtonSize, ButtonVariant};
pub use checkbox::{Checkbox, CheckboxProps, CheckboxState};
pub use icon::{Icon, IconColor, IconSize};
pub use input::{Input, InputProps};
pub use label::{Label, LabelVariant};
pub use radio::{Radio, RadioProps};
pub use spinner::{Spinner, SpinnerColor, SpinnerProps, SpinnerSize};
pub use switch::{Switch, SwitchProps};
