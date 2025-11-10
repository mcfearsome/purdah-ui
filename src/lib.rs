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
//! - [`atoms`]: Primitive components (Button, Input, Icon, Badge, Avatar, Checkbox, Radio, Switch, Spinner)
//! - [`molecules`]: Composite components (SearchBar, FormGroup, Card)
//! - [`layout`]: Layout primitives (VStack, HStack, Spacer, Container, Divider)
//! - [`organisms`]: Complex components (Dialog, Drawer, Table, CommandPalette)
//! - [`utils`]: Accessibility utilities and helpers (FocusTrap, Announcer)
//! - [`prelude`]: Convenient re-exports for common imports

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![recursion_limit = "512"]

pub mod theme;
pub mod atoms;
pub mod layout;
pub mod molecules;
pub mod organisms;
pub mod utils;

pub mod prelude;
