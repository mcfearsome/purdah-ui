//! Organism components - complex, feature-complete components.
//!
//! Organisms are composed of molecules and atoms to create sophisticated,
//! feature-rich components with complex interactions and state.
//!
//! ## Available Organisms
//!
//! - [`Dialog`]: Modal dialog with overlay and focus management
//! - [`Drawer`]: Side panel drawer with slide-in animation
//! - [`Table`]: Data table with sortable columns
//! - [`CommandPalette`]: Searchable command interface
//!
//! ## Example
//!
//! ```rust,ignore
//! use purdah_gpui_components::organisms::*;
//!
//! // Dialog
//! Dialog::new()
//!     .title("Confirm Action")
//!     .description("Are you sure?")
//!     .open(true);
//!
//! // Drawer
//! Drawer::new()
//!     .title("Settings")
//!     .position(DrawerPosition::Right)
//!     .open(true);
//!
//! // Table
//! Table::new()
//!     .columns(vec![
//!         TableColumn { header: "Name".into(), width: Some(px(200.0)) },
//!     ]);
//!
//! // Command Palette
//! CommandPalette::new()
//!     .commands(vec![Command { label: "Open".into(), description: None }])
//!     .open(true);
//! ```

pub mod dialog;
pub mod drawer;
pub mod table;
pub mod command_palette;

pub use dialog::{Dialog, DialogProps};
pub use drawer::{Drawer, DrawerPosition, DrawerProps};
pub use table::{Table, TableColumn, TableProps};
pub use command_palette::{Command, CommandPalette, CommandPaletteProps};
