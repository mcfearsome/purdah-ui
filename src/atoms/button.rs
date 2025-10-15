//! Button component with multiple variants and states.

use gpui::*;
use crate::theme::{ButtonTokens, Theme};

/// Button visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Primary action button (filled, high emphasis)
    #[default]
    Primary,
    /// Secondary action button (filled, medium emphasis)
    Secondary,
    /// Outline button (bordered, medium emphasis)
    Outline,
    /// Ghost button (transparent, low emphasis)
    Ghost,
    /// Danger button (destructive actions)
    Danger,
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Small button (compact)
    Sm,
    /// Medium button (default)
    #[default]
    Md,
    /// Large button (prominent)
    Lg,
}

/// Button configuration properties
#[derive(Clone)]
pub struct ButtonProps {
    /// Button label text
    pub label: SharedString,
    /// Visual variant
    pub variant: ButtonVariant,
    /// Size variant
    pub size: ButtonSize,
    /// Whether button is disabled
    pub disabled: bool,
    /// Whether button is in loading state
    pub loading: bool,
    /// Click event handler
    pub on_click: Option<Arc<dyn Fn(&ClickEvent, &mut WindowContext) + Send + Sync>>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            label: "Button".into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            disabled: false,
            loading: false,
            on_click: None,
        }
    }
}

/// A button component with multiple variants and states.
///
/// Button is the primary interactive component for user actions.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::atoms::*;
///
/// // Basic button
/// Button::new()
///     .label("Click me")
///     .on_click(|_, cx| {
///         println!("Clicked!");
///     });
///
/// // Primary button
/// Button::new()
///     .label("Save")
///     .variant(ButtonVariant::Primary)
///     .size(ButtonSize::Lg);
///
/// // Disabled button
/// Button::new()
///     .label("Submit")
///     .disabled(true);
/// ```
pub struct Button {
    props: ButtonProps,
    focused: bool,
    hovered: bool,
    pressed: bool,
}

impl Button {
    /// Create a new button with default props
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let button = Button::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: ButtonProps::default(),
            focused: false,
            hovered: false,
            pressed: false,
        }
    }

    /// Set the button label text
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Button::new().label("Click me");
    /// ```
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.props.label = label.into();
        self
    }

    /// Set the button variant
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Button::new().variant(ButtonVariant::Primary);
    /// ```
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.props.variant = variant;
        self
    }

    /// Set the button size
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Button::new().size(ButtonSize::Lg);
    /// ```
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.props.size = size;
        self
    }

    /// Set whether the button is disabled
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Button::new().disabled(true);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    /// Set whether the button is in loading state
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Button::new().loading(is_loading);
    /// ```
    pub fn loading(mut self, loading: bool) -> Self {
        self.props.loading = loading;
        self
    }

    /// Set the click event handler
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Button::new()
    ///     .label("Save")
    ///     .on_click(|_, cx| {
    ///         // Handle click
    ///     });
    /// ```
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut WindowContext) + Send + Sync + 'static,
    ) -> Self {
        self.props.on_click = Some(Arc::new(handler));
        self
    }

    /// Get background color based on variant and state
    fn background_color(&self, tokens: &ButtonTokens) -> Hsla {
        if self.props.disabled {
            return tokens.background_primary_disabled;
        }

        match (&self.props.variant, self.hovered, self.pressed) {
            // Primary variant
            (ButtonVariant::Primary, _, true) => tokens.background_primary_active,
            (ButtonVariant::Primary, true, false) => tokens.background_primary_hover,
            (ButtonVariant::Primary, false, false) => tokens.background_primary,

            // Secondary variant
            (ButtonVariant::Secondary, _, true) | (ButtonVariant::Secondary, true, false) => {
                tokens.background_secondary_hover
            }
            (ButtonVariant::Secondary, false, false) => tokens.background_secondary,

            // Outline variant
            (ButtonVariant::Outline, true, _) => tokens.background_outline_hover,
            (ButtonVariant::Outline, false, _) => tokens.background_outline,

            // Ghost variant
            (ButtonVariant::Ghost, true, _) => tokens.background_ghost_hover,
            (ButtonVariant::Ghost, false, _) => tokens.background_ghost,

            // Danger variant
            (ButtonVariant::Danger, _, true) | (ButtonVariant::Danger, true, false) => {
                tokens.background_danger_hover
            }
            (ButtonVariant::Danger, false, false) => tokens.background_danger,
        }
    }

    /// Get text color based on variant
    fn text_color(&self, tokens: &ButtonTokens) -> Hsla {
        if self.props.disabled {
            return tokens.text_disabled;
        }

        match self.props.variant {
            ButtonVariant::Primary => tokens.text_primary,
            ButtonVariant::Secondary => tokens.text_secondary,
            ButtonVariant::Outline => tokens.text_outline,
            ButtonVariant::Ghost => tokens.text_ghost,
            ButtonVariant::Danger => tokens.text_danger,
        }
    }

    /// Get padding based on size
    fn padding(&self, tokens: &ButtonTokens) -> (Pixels, Pixels) {
        match self.props.size {
            ButtonSize::Sm => (tokens.padding_x_sm, tokens.padding_y_sm),
            ButtonSize::Md => (tokens.padding_x_md, tokens.padding_y_md),
            ButtonSize::Lg => (tokens.padding_x_lg, tokens.padding_y_lg),
        }
    }

    /// Get font size based on size
    fn font_size(&self, tokens: &ButtonTokens) -> Pixels {
        match self.props.size {
            ButtonSize::Sm => tokens.font_size_sm,
            ButtonSize::Md => tokens.font_size_md,
            ButtonSize::Lg => tokens.font_size_lg,
        }
    }

    /// Get border styling for outline variant
    fn border_style(&self, tokens: &ButtonTokens) -> Option<(Pixels, Hsla)> {
        if self.props.variant == ButtonVariant::Outline {
            let color = if self.hovered {
                tokens.border_outline_hover
            } else {
                tokens.border_outline
            };
