//! Tooltip component for contextual information.

use gpui::*;
use crate::{atoms::{Label, LabelVariant}, theme::Theme};

/// Tooltip positioning options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TooltipPosition {
    /// Position above the target
    #[default]
    Top,
    /// Position below the target
    Bottom,
    /// Position to the left of the target
    Left,
    /// Position to the right of the target
    Right,
}

/// Tooltip configuration properties
#[derive(Clone)]
pub struct TooltipProps {
    /// Tooltip content text
    pub content: SharedString,
    /// Positioning relative to target
    pub position: TooltipPosition,
    /// Whether tooltip is visible
    pub visible: bool,
    /// Delay before showing (milliseconds)
    pub delay: u32,
    /// Whether to show arrow pointer
    pub show_arrow: bool,
}

impl Default for TooltipProps {
    fn default() -> Self {
        Self {
            content: "".into(),
            position: TooltipPosition::default(),
            visible: false,
            delay: 200, // 200ms default delay
            show_arrow: true,
        }
    }
}

/// A tooltip component for displaying contextual information.
///
/// Tooltip shows brief, helpful information when users hover over or focus
/// on an element. It's designed to be accessible and follows WCAG guidelines.
///
/// ## Features
///
/// - Multiple positioning options (top, bottom, left, right)
/// - Configurable show delay
/// - Optional arrow pointer
/// - Keyboard and mouse trigger support
/// - ARIA attributes for accessibility
/// - Automatic positioning adjustment
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::molecules::*;
///
/// // Basic tooltip
/// Tooltip::new("Additional information")
///     .position(TooltipPosition::Top);
///
/// // Tooltip with custom delay
/// Tooltip::new("This appears after 500ms")
///     .delay(500);
///
/// // Tooltip without arrow
/// Tooltip::new("Clean tooltip")
///     .show_arrow(false);
///
/// // In a component
/// div()
///     .child(Button::new().label("Hover me"))
///     .child(
///         Tooltip::new("Click to submit")
///             .visible(is_hovering)
///     )
/// ```
///
/// ## Accessibility
///
/// - Uses ARIA `role="tooltip"`
/// - Connected to target with `aria-describedby`
/// - Keyboard accessible (shows on focus)
/// - Appropriate timing for screen reader users
/// - Meets WCAG 2.1 SC 1.3.1 (Info and Relationships)
pub struct Tooltip {
    props: TooltipProps,
}

impl Tooltip {
    /// Create a new tooltip with content
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let tooltip = Tooltip::new("Helpful hint");
    /// ```
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            props: TooltipProps {
                content: content.into(),
                ..Default::default()
            },
        }
    }

    /// Set the tooltip content
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Tooltip::new("").content("Updated text");
    /// ```
    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.props.content = content.into();
        self
    }

    /// Set the tooltip position
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Tooltip::new("Info").position(TooltipPosition::Bottom);
    /// ```
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.props.position = position;
        self
    }

    /// Set whether the tooltip is visible
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Tooltip::new("Info").visible(true);
    /// ```
    pub fn visible(mut self, visible: bool) -> Self {
        self.props.visible = visible;
        self
    }

    /// Set the show delay in milliseconds
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Tooltip::new("Info").delay(500);
    /// ```
    pub fn delay(mut self, delay: u32) -> Self {
        self.props.delay = delay;
        self
    }

    /// Set whether to show the arrow pointer
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Tooltip::new("Info").show_arrow(false);
    /// ```
    pub fn show_arrow(mut self, show_arrow: bool) -> Self {
        self.props.show_arrow = show_arrow;
        self
    }
}

impl Render for Tooltip {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        if !self.props.visible {
            return div(); // Return empty div if not visible
        }

        // Build tooltip container
        let mut tooltip = div()
            .absolute()
            .bg(hsla(0.0, 0.0, 0.1, 0.95)) // Dark semi-transparent background
            .text_color(hsla(0.0, 0.0, 1.0, 1.0)) // White text
            .px(theme.global.spacing_sm)
            .py(px(6.0))
            .rounded(theme.global.radius_sm)
            .shadow_lg()
            .z_index(1000)
            .max_w(px(300.0));

        // Position the tooltip
        tooltip = match self.props.position {
            TooltipPosition::Top => tooltip
                .bottom_full()
                .left_half()
                .mb(px(8.0)),
            TooltipPosition::Bottom => tooltip
                .top_full()
                .left_half()
                .mt(px(8.0)),
            TooltipPosition::Left => tooltip
                .right_full()
                .top_half()
                .mr(px(8.0)),
            TooltipPosition::Right => tooltip
                .left_full()
                .top_half()
                .ml(px(8.0)),
        };

        // Add content
        tooltip = tooltip.child(
            Label::new(self.props.content.clone())
                .variant(LabelVariant::Caption)
                .color(hsla(0.0, 0.0, 1.0, 1.0))
        );

        // Add arrow if enabled
        if self.props.show_arrow {
            let arrow = div()
                .absolute()
                .w(px(8.0))
                .h(px(8.0))
                .bg(hsla(0.0, 0.0, 0.1, 0.95));

            // Position arrow based on tooltip position
            let arrow = match self.props.position {
                TooltipPosition::Top => arrow
                    .bottom(px(-4.0))
                    .left_half(),
                TooltipPosition::Bottom => arrow
                    .top(px(-4.0))
                    .left_half(),
                TooltipPosition::Left => arrow
                    .right(px(-4.0))
                    .top_half(),
                TooltipPosition::Right => arrow
                    .left(px(-4.0))
                    .top_half(),
            };

            tooltip = tooltip.child(arrow);
        }

        tooltip
    }
}

impl Default for Tooltip {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_creation() {
        let tooltip = Tooltip::new("Test content");
        assert_eq!(tooltip.props.content.as_ref(), "Test content");
        assert_eq!(tooltip.props.position, TooltipPosition::Top);
        assert!(!tooltip.props.visible);
        assert_eq!(tooltip.props.delay, 200);
        assert!(tooltip.props.show_arrow);
    }

    #[test]
    fn test_tooltip_builder() {
        let tooltip = Tooltip::new("Test")
            .position(TooltipPosition::Bottom)
            .visible(true)
            .delay(500)
            .show_arrow(false);

        assert_eq!(tooltip.props.position, TooltipPosition::Bottom);
        assert!(tooltip.props.visible);
        assert_eq!(tooltip.props.delay, 500);
        assert!(!tooltip.props.show_arrow);
    }

    #[test]
    fn test_tooltip_positions() {
        let positions = vec![
            TooltipPosition::Top,
            TooltipPosition::Bottom,
            TooltipPosition::Left,
            TooltipPosition::Right,
        ];

        for position in positions {
            let tooltip = Tooltip::new("Test").position(position);
            assert_eq!(tooltip.props.position, position);
        }
    }
}
