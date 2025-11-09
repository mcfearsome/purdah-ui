//! Molecule components - composite components built from atoms.
//!
//! Molecules are composed of multiple atoms to create more complex, reusable components.
//!
//! ## Available Molecules
//!
//! - [`SearchBar`]: Search input with icon and clear button
//! - [`FormGroup`]: Label + Input + Error message combination
//! - [`Card`]: Content card container with variants
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

pub use search_bar::{SearchBar, SearchBarProps};
pub use form_group::{FormGroup, FormGroupProps};
pub use card::{Card, CardProps, CardVariant};
