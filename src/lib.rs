//! # Purdah GPUI Components
//!
//! High-level component library built on GPUI for improved developer experience.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use purdah_gpui_components::prelude::*;
//!
//! // Use the theme system
//! let theme = Theme::light();
//!
//! // Access design tokens
//! let primary_color = theme.alias.color_primary;
//! let base_spacing = theme.global.spacing_base;
//!
//! // Switch themes at runtime
//! let dark_theme = theme.with_mode(ThemeMode::Dark);
//! ```
//!
//! ## Features
//!
//! - **Atomic Design**: Atoms → Molecules → Organisms hierarchy
//! - **Design Tokens**: 3-layer token system (Global → Alias → Component)
//! - **Theming**: Light, Dark, and custom themes with runtime switching
//! - **Accessibility**: WCAG 2.1 AA compliance with built-in ARIA and keyboard navigation
//! - **Performance**: GPU-accelerated rendering via GPUI
//!
//! ## Module Organization
//!
//! - [`theme`]: Design token system and theming
//! - `atoms`: Primitive components (Button, Input, Icon, etc.) - *coming soon*
//! - `molecules`: Composite components (SearchBar, FormGroup, etc.) - *coming soon*
//! - `organisms`: Complex components (Dialog, Table, etc.) - *coming soon*
//! - `layout`: Layout primitives (VStack, HStack, Grid, etc.) - *coming soon*
//! - `utils`: Accessibility utilities and helpers - *coming soon*
//! - [`prelude`]: Convenient re-exports for common imports

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

pub mod theme;
pub mod atoms;

// Component modules (to be implemented)
// pub mod molecules;
// pub mod organisms;
// pub mod layout;
// pub mod utils;

pub mod prelude;
