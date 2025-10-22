//! Theme definitions and theming system.

use super::{AliasTokens, GlobalTokens};

/// Theme mode variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Light theme mode
    Light,
    /// Dark theme mode
    Dark,
    /// Follow system theme preference
    System,
}

/// Complete theme containing all token layers
///
/// A theme bundles together global tokens, alias tokens, and component-specific tokens
/// to provide a complete styling system.
///
/// ## Example
///
/// ```rust,no_run
/// use purdah_gpui_components::theme::Theme;
///
/// // Use built-in themes
/// let light = Theme::light();
/// let dark = Theme::dark();
///
/// // Access token values
/// let primary_color = light.alias.color_primary;
/// let spacing = light.global.spacing_base;
/// ```
#[derive(Debug, Clone)]
pub struct Theme {
    /// Global foundational tokens
    pub global: GlobalTokens,
    /// Semantic alias tokens
    pub alias: AliasTokens,
    /// Theme mode
    pub mode: ThemeMode,
}

impl Theme {
    /// Create a new light theme with default tokens
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::Theme;
    ///
    /// let theme = Theme::light();
    /// ```
    pub fn light() -> Self {
        let global = GlobalTokens::default();
        let alias = AliasTokens::from_global(&global, false);

        Self {
            global,
            alias,
            mode: ThemeMode::Light,
        }
    }

    /// Create a new dark theme with default tokens
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::Theme;
    ///
    /// let theme = Theme::dark();
    /// ```
    pub fn dark() -> Self {
        let global = GlobalTokens::default();
        let alias = AliasTokens::from_global(&global, true);

        Self {
            global,
            alias,
            mode: ThemeMode::Dark,
        }
    }

    /// Create a theme based on the specified mode
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::{Theme, ThemeMode};
    ///
    /// let theme = Theme::from_mode(ThemeMode::Dark);
    /// ```
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
            ThemeMode::System => {
                // TODO: Detect system theme preference
                // For now, default to light mode
                Self::light()
            }
        }
    }

    /// Switch to a different theme mode
    ///
    /// This creates a new theme with the specified mode while preserving
    /// custom global token overrides if any.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::{Theme, ThemeMode};
    ///
    /// let mut theme = Theme::light();
    /// theme = theme.with_mode(ThemeMode::Dark);
    /// ```
    pub fn with_mode(self, mode: ThemeMode) -> Self {
        let is_dark = matches!(mode, ThemeMode::Dark);
        let alias = AliasTokens::from_global(&self.global, is_dark);

        Self {
            global: self.global,
            alias,
            mode,
        }
    }

    /// Check if this is a dark theme
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::Theme;
    ///
    /// let theme = Theme::dark();
    /// assert!(theme.is_dark());
    /// ```
    pub fn is_dark(&self) -> bool {
        matches!(self.mode, ThemeMode::Dark)
    }

    /// Check if this is a light theme
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// use purdah_gpui_components::theme::Theme;
    ///
    /// let theme = Theme::light();
    /// assert!(theme.is_light());
    /// ```
    pub fn is_light(&self) -> bool {
        matches!(self.mode, ThemeMode::Light)
    }
}

impl Default for Theme {
    /// Returns the default theme, which is the light theme.
    fn default() -> Self {
        Self::light()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_theme_creation() {
        let theme = Theme::light();
        assert!(theme.is_light());
        assert!(!theme.is_dark());
        assert_eq!(theme.mode, ThemeMode::Light);
    }

    #[test]
    fn test_dark_theme_creation() {
        let theme = Theme::dark();
        assert!(theme.is_dark());
        assert!(!theme.is_light());
        assert_eq!(theme.mode, ThemeMode::Dark);
    }

    #[test]
    fn test_theme_mode_switching() {
        let theme = Theme::light();
        let dark_theme = theme.with_mode(ThemeMode::Dark);
        assert!(dark_theme.is_dark());
    }

    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert!(theme.is_light());
    }

    #[test]
    fn test_from_mode() {
        let light = Theme::from_mode(ThemeMode::Light);
        assert!(light.is_light());

        let dark = Theme::from_mode(ThemeMode::Dark);
        assert!(dark.is_dark());
    }
}
