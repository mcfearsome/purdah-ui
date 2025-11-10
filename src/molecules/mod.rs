//! Molecule components - composite components built from atoms.
//!
//! Molecules are composed of multiple atoms to create more complex, reusable components.
//!
//! ## Available Molecules
//!
//! - [`SearchBar`]: Search input with icon and clear button
//! - [`FormGroup`]: Label + Input + Error message combination
//! - [`Card`]: Content card container with variants
//! - [`TabGroup`]: Tabbed navigation with keyboard support
//! - [`Dropdown`]: Select menu with search and multi-select support
//! - [`Tooltip`]: Contextual information on hover/focus
//! - [`Popover`]: Click-triggered overlay with rich content
//!
//! ## Example
//!
//! ```rust,ignore
//! use purdah_gpui_components::molecules::*;
//!
//! // Search bar
//! SearchBar::new()
//!     .placeholder("Search...")
//!     .on_search(|query| { /* handle search */ });
//!
//! // Form group
//! FormGroup::new()
//!     .label("Email")
//!     .required(true)
//!     .input(Input::new().placeholder("you@example.com"));
//!
//! // Card
//! Card::new()
//!     .title("Profile")
//!     .variant(CardVariant::Elevated);
//! ```

pub mod search_bar;
pub mod form_group;
pub mod card;
pub mod tab_group;
pub mod dropdown;
pub mod tooltip;
pub mod popover;

pub use search_bar::{SearchBar, SearchBarProps};
pub use form_group::{FormGroup, FormGroupProps};
pub use card::{Card, CardProps, CardVariant};
pub use tab_group::{TabGroup, TabGroupProps, TabGroupVariant, Tab};
pub use dropdown::{Dropdown, DropdownProps, DropdownVariant, DropdownOption};
pub use tooltip::{Tooltip, TooltipProps, TooltipPosition};
pub use popover::{Popover, PopoverProps, PopoverPosition};
