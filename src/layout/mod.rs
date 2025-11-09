//! Layout components for arranging UI elements.
//!
//! Layout components provide flexible containers for organizing atoms and molecules.
//!
//! ## Available Layout Components
//!
//! - [`VStack`]: Vertical stack layout
//! - [`HStack`]: Horizontal stack layout
//! - [`Spacer`]: Flexible spacing component
//! - [`Container`]: Max-width container with centering
//! - [`Divider`]: Horizontal or vertical divider line
//!
//! ## Example
//!
//! ```rust,ignore
//! use purdah_gpui_components::layout::*;
//!
//! // Vertical stack with gap
//! VStack::new()
//!     .gap(spacing_md)
//!     .children(vec![
//!         Button::new().label("First"),
//!         Button::new().label("Second"),
//!     ]);
//!
//! // Horizontal stack with alignment
//! HStack::new()
//!     .gap(spacing_sm)
//!     .align(Alignment::Center)
//!     .children(vec![
//!         Label::new("Name:"),
//!         Input::new(),
//!     ]);
//! ```

pub mod stack;
pub mod spacer;
pub mod container;
pub mod divider;

pub use stack::{HStack, VStack, Alignment, Justify};
pub use spacer::Spacer;
pub use container::Container;
pub use divider::{Divider, DividerOrientation};
