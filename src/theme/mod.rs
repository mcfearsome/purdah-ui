//! Design token system and theming.
//!
//! The theme system follows a 3-layer architecture:
//!
//! 1. **Global Tokens** ([`GlobalTokens`]): Foundational values (colors, spacing, typography)
//! 2. **Alias Tokens** ([`AliasTokens`]): Semantic mappings (primary, danger, surface)
//! 3. **Component Tokens**: Component-specific tokens (ButtonTokens, InputTokens, etc.)
//!
//! ## Example
//!
//! ```rust,no_run
//! use purdah_gpui_components::theme::Theme;
//!
//! // Use built-in themes
//! let light_theme = Theme::light();
//! let dark_theme = Theme::dark();
//!
//! // Access token values
//! let primary_color = light_theme.alias.color_primary;
//! let base_spacing = light_theme.global.spacing_base;
//! ```

mod tokens;
mod themes;

pub use tokens::{AliasTokens, ButtonTokens, GlobalTokens, IconTokens, InputTokens, LabelTokens};
pub use themes::{Theme, ThemeMode};
