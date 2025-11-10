//! Accessibility utilities and helpers.
//!
//! This module provides utilities for building accessible applications
//! that comply with WCAG 2.1 AA standards.
//!
//! ## Available Utilities
//!
//! - [`FocusTrap`]: Manages focus within a boundary (dialogs, modals)
//! - [`Announcer`]: Communicates updates to screen readers via live regions
//!
//! ## Example
//!
//! ```rust,ignore
//! use purdah_gpui_components::utils::*;
//!
//! // Trap focus within a dialog
//! FocusTrap::new()
//!     .boundary_element(dialog_element)
//!     .restore_on_unmount(true);
//!
//! // Announce a status update
//! Announcer::polite("Form saved successfully");
//! ```

pub mod focus_trap;
pub mod announcer;

pub use focus_trap::FocusTrap;
pub use announcer::{Announcer, AnnouncerPriority};
