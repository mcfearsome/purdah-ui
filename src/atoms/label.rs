//! Text label component with typography variants.

use gpui::*;

/// Label text variants for different typography styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelVariant {
    /// Body text (16px, normal weight)
    #[default]
    Body,
    /// Caption text (14px, normal weight)
    Caption,
    /// Small heading (20px, semibold)
    Heading3,
    /// Medium heading (24px, semibold)
    Heading2,
    /// Large heading (30px, bold)
    Heading1,
}

/// A text label component with typography variants.
///
/// Label is the simplest atom for displaying text with consistent typography.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::atoms::*;
///
/// // Body text
/// Label::new("Regular body text");
///
/// // Heading
/// Label::new("Page Title")
///     .variant(LabelVariant::Heading1);
///
/// // Caption with custom color
/// Label::new("Helper text")
///     .variant(LabelVariant::Caption)
///     .color(theme.alias.color_text_muted);
/// ```
pub struct Label {
    text: SharedString,
    variant: LabelVariant,
    color: Option<Hsla>,
}

impl Label {
    /// Create a new label with the given text.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let label = Label::new("Hello, World!");
    /// ```
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            variant: LabelVariant::default(),
            color: None,
        }
    }

    /// Set the label variant for typography styling.
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Label::new("Title").variant(LabelVariant::Heading1);
    /// ```
    pub fn variant(mut self, variant: LabelVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set a custom text color (overrides variant default).
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Label::new("Error").color(theme.alias.color_danger);
    /// ```
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Get the font size for this label's variant
    fn font_size(&self, theme: &crate::theme::Theme) -> Pixels {
        match self.variant {
            LabelVariant::Body => theme.alias.font_size_body,
            LabelVariant::Caption => theme.alias.font_size_caption,
            LabelVariant::Heading3 => theme.global.font_size_xl,
            LabelVariant::Heading2 => theme.global.font_size_2xl,
            LabelVariant::Heading1 => theme.global.font_size_3xl,
        }
    }

    /// Get the font weight for this label's variant
    fn font_weight(&self, theme: &crate::theme::Theme) -> FontWeight {
        match self.variant {
            LabelVariant::Body | LabelVariant::Caption => FontWeight::NORMAL,
            LabelVariant::Heading3 | LabelVariant::Heading2 => FontWeight::SEMIBOLD,
            LabelVariant::Heading1 => FontWeight::BOLD,
        }
    }

    /// Get the text color for this label
    fn text_color(&self, theme: &crate::theme::Theme) -> Hsla {
        self.color.unwrap_or_else(|| match self.variant {
            LabelVariant::Body | LabelVariant::Heading1 | LabelVariant::Heading2 | LabelVariant::Heading3 => {
                theme.alias.color_text_primary
            }
            LabelVariant::Caption => theme.alias.color_text_secondary,
        })
    }
}

impl Render for Label {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        // Access theme from global context (will be implemented with ThemeProvider)
        // For now, create a default theme
        let theme = crate::theme::Theme::default();

        div()
            .text_size(self.font_size(&theme))
            .font_weight(self.font_weight(&theme))
            .text_color(self.text_color(&theme))
            .child(self.text.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;

    #[test]
    fn test_label_variants() {
        let theme = Theme::light();

        let body = Label::new("body");
        assert_eq!(body.font_size(&theme), theme.alias.font_size_body);

        let caption = Label::new("caption").variant(LabelVariant::Caption);
        assert_eq!(caption.font_size(&theme), theme.alias.font_size_caption);

        let h1 = Label::new("h1").variant(LabelVariant::Heading1);
        assert_eq!(h1.font_size(&theme), theme.global.font_size_3xl);
    }

    #[test]
    fn test_label_custom_color() {
        let theme = Theme::light();
        let custom_color = theme.alias.color_danger;

        let label = Label::new("error").color(custom_color);
        assert_eq!(label.text_color(&theme), custom_color);
    }

    #[test]
    fn test_label_default_colors() {
        let theme = Theme::light();

        let body = Label::new("body");
        assert_eq!(body.text_color(&theme), theme.alias.color_text_primary);

        let caption = Label::new("caption").variant(LabelVariant::Caption);
        assert_eq!(caption.text_color(&theme), theme.alias.color_text_secondary);
    }
}
