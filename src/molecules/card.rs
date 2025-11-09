//! Card component for content containers.

use gpui::*;
use crate::{atoms::{Label, LabelVariant}, theme::Theme};

/// Card visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CardVariant {
    /// Flat card with no border or shadow
    Flat,
    /// Outlined card with border
    #[default]
    Outlined,
    /// Elevated card with shadow
    Elevated,
}

/// Card configuration properties
#[derive(Clone)]
pub struct CardProps {
    /// Card title
    pub title: Option<SharedString>,
    /// Card variant
    pub variant: CardVariant,
    /// Whether card is hoverable/clickable
    pub hoverable: bool,
}

impl Default for CardProps {
    fn default() -> Self {
        Self {
            title: None,
            variant: CardVariant::default(),
            hoverable: false,
        }
    }
}

/// A card component for content containers.
///
/// Card provides a styled container for grouping related content.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::organisms::*;
///
/// // Basic card
/// Card::new()
///     .title("User Profile");
///
/// // Elevated card
/// Card::new()
///     .variant(CardVariant::Elevated)
///     .hoverable(true);
///
/// // Card with content
/// Card::new()
///     .title("Settings")
///     .variant(CardVariant::Outlined);
/// ```
pub struct Card {
    props: CardProps,
}

impl Card {
    /// Create a new card
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let card = Card::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: CardProps::default(),
        }
    }

    /// Set the card title
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Card::new().title("Profile");
    /// ```
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.props.title = Some(title.into());
        self
    }

    /// Set the card variant
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Card::new().variant(CardVariant::Elevated);
    /// ```
    pub fn variant(mut self, variant: CardVariant) -> Self {
        self.props.variant = variant;
        self
    }

    /// Set whether the card is hoverable
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Card::new().hoverable(true);
    /// ```
    pub fn hoverable(mut self, hoverable: bool) -> Self {
        self.props.hoverable = hoverable;
        self
    }
}

impl Render for Card {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        // Build card container
        let mut card = div()
            .bg(theme.alias.color_surface)
            .rounded(theme.global.radius_lg)
            .p(theme.global.spacing_lg)
            .flex()
            .flex_col()
            .gap(theme.global.spacing_md);

        // Apply variant styling
        card = match self.props.variant {
            CardVariant::Flat => card,
            CardVariant::Outlined => card
                .border_color(theme.alias.color_border)
                .border(px(1.0)),
            CardVariant::Elevated => card
                .shadow_lg()
                .when(self.props.hoverable, |c| c.hover(|style| {
                    style.shadow_xl()
                })),
        };

        // Add title if present
        if let Some(title) = &self.props.title {
            card = card.child(
                Label::new(title.clone())
                    .variant(LabelVariant::Heading3)
            );
        }

        // Add placeholder content area
        card.child(
            div()
                .text_size(theme.alias.font_size_body)
                .text_color(theme.alias.color_text_secondary)
                .child("Card content goes here")
        )
    }
}
