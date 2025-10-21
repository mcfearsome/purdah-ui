//! Convenient re-exports for common imports.
//!
//! Use this module to quickly import commonly used types and traits:
//!
//! ```rust
//! use purdah_gpui_components::prelude::*;
//! ```

// Re-export theme types
pub use crate::theme::{
    AliasTokens, ButtonTokens, GlobalTokens, InputTokens, LabelTokens, Theme, ThemeMode,
};

// Re-export atom components
pub use crate::atoms::{
    Label, LabelVariant,
    Button, ButtonProps, ButtonSize, ButtonVariant,
    Input, InputProps,
};

// Re-export GPUI core types for convenience
pub use gpui::*;

// Future component re-exports (to be added as components are implemented)
// pub use crate::molecules::*;
// pub use crate::organisms::*;
// pub use crate::layout::*;
