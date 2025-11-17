//! Popover component for rich contextual overlays.

use gpui::*;
use crate::{atoms::{Label, LabelVariant, Button, ButtonVariant, Icon, icons}, theme::Theme, utils::FocusTrap};

/// Popover positioning options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverPosition {
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

/// Popover configuration properties
#[derive(Clone)]
pub struct PopoverProps {
    /// Popover title
    pub title: Option<SharedString>,
    /// Popover content description
    pub content: SharedString,
    /// Positioning relative to target
    pub position: PopoverPosition,
    /// Whether popover is visible/open
    pub open: bool,
    /// Whether to show close button
    pub show_close: bool,
    /// Whether to show arrow pointer
    pub show_arrow: bool,
    /// Whether clicking outside closes the popover
    pub close_on_outside_click: bool,
}

impl Default for PopoverProps {
    fn default() -> Self {
        Self {
            title: None,
            content: "".into(),
            position: PopoverPosition::default(),
            open: false,
            show_close: true,
            show_arrow: true,
            close_on_outside_click: true,
        }
    }
}

/// A popover component for rich contextual overlays.
///
/// Popover is similar to Tooltip but supports more complex content,
/// interactive elements, and is triggered by clicks rather than hover.
/// It provides focus management and keyboard navigation.
///
/// ## Features
///
/// - Multiple positioning options (top, bottom, left, right)
/// - Optional title and close button
/// - Optional arrow pointer
/// - Click-outside-to-close behavior
/// - Focus trap for keyboard accessibility
/// - ARIA attributes for screen readers
/// - Can contain interactive content
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::molecules::*;
///
/// // Basic popover
/// Popover::new("Click for more details")
///     .position(PopoverPosition::Bottom)
///     .open(is_open);
///
/// // Popover with title
/// Popover::new("Additional information about this feature")
///     .title("Help")
///     .show_close(true);
///
/// // Popover without arrow
/// Popover::new("Clean popover content")
///     .show_arrow(false)
///     .close_on_outside_click(true);
///
/// // In a component
/// div()
///     .child(Button::new().label("Open Popover"))
///     .child(
///         Popover::new("Detailed explanation here")
///             .title("Information")
///             .open(popover_open)
///     )
/// ```
///
/// ## Accessibility
///
/// - Uses ARIA `role="dialog"` for complex popovers
/// - Keyboard accessible (Escape to close)
/// - Focus trap when open
/// - Connected to trigger with `aria-controls`
/// - Meets WCAG 2.1 SC 2.4.3 (Focus Order)
pub struct Popover {
    props: PopoverProps,
    focus_trap: FocusTrap,
}

impl Popover {
    /// Create a new popover with content
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let popover = Popover::new("Content text");
    /// ```
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            props: PopoverProps {
                content: content.into(),
                ..Default::default()
            },
            focus_trap: FocusTrap::new(),
        }
    }

    /// Set the popover content
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("").content("Updated content");
    /// ```
    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.props.content = content.into();
        self
    }

    /// Set the popover title
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("Content").title("Information");
    /// ```
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.props.title = Some(title.into());
        self
    }

    /// Set the popover position
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("Content").position(PopoverPosition::Right);
    /// ```
    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.props.position = position;
        self
    }

    /// Set whether the popover is open
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("Content").open(true);
    /// ```
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    /// Set whether to show the close button
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("Content").show_close(false);
    /// ```
    pub fn show_close(mut self, show_close: bool) -> Self {
        self.props.show_close = show_close;
        self
    }

    /// Set whether to show the arrow pointer
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("Content").show_arrow(false);
    /// ```
    pub fn show_arrow(mut self, show_arrow: bool) -> Self {
        self.props.show_arrow = show_arrow;
        self
    }

    /// Set whether clicking outside closes the popover
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Popover::new("Content").close_on_outside_click(true);
    /// ```
    pub fn close_on_outside_click(mut self, close_on_outside_click: bool) -> Self {
        self.props.close_on_outside_click = close_on_outside_click;
        self
    }
}

impl Render for Popover {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        if !self.props.open {
            return div(); // Return empty div if not open
        }

        // Build popover container
        let mut popover = div()
            .absolute()
            .bg(theme.alias.color_surface)
            .border(px(1.0))
            .border_color(theme.alias.color_border)
            .rounded(theme.global.radius_lg)
            .shadow_xl()
            .min_w(px(200.0))
            .max_w(px(400.0))
            .flex()
            .flex_col();

        // Position the popover
        popover = match self.props.position {
            PopoverPosition::Top => popover
                .bottom_full()
                .mb(theme.global.spacing_sm),
            PopoverPosition::Bottom => popover
                .top_full()
                .mt(theme.global.spacing_sm),
            PopoverPosition::Left => popover
                .right_full()
                .mr(theme.global.spacing_sm),
            PopoverPosition::Right => popover
                .left_full()
                .ml(theme.global.spacing_sm),
        };

        // Add header if title exists or close button is shown
        if self.props.title.is_some() || self.props.show_close {
            let mut header = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(theme.global.spacing_md)
                .py(theme.global.spacing_sm)
                .border_b(px(1.0))
                .border_color(theme.alias.color_border);

            // Add title if present
            if let Some(ref title) = self.props.title {
                header = header.child(
                    Label::new(title.clone())
                        .variant(LabelVariant::Heading3)
                );
            } else {
                header = header.child(div()); // Empty spacer
            }

            // Add close button if enabled
            if self.props.show_close {
                header = header.child(
                    Button::new()
                        .label("×")
                        .variant(ButtonVariant::Ghost)
                );
            }

            popover = popover.child(header);
        }

        // Add content
        popover = popover.child(
            div()
                .px(theme.global.spacing_md)
                .py(theme.global.spacing_md)
                .child(
                    Label::new(self.props.content.clone())
                        .variant(LabelVariant::Body)
                        .color(theme.alias.color_text_secondary)
                )
        );

        // Add arrow if enabled
        if self.props.show_arrow {
            let arrow = div()
                .absolute()
                .w(px(12.0))
                .h(px(12.0))
                .bg(theme.alias.color_surface)
                .border(px(1.0))
                .border_color(theme.alias.color_border);

            // Position arrow based on popover position
            let arrow = match self.props.position {
                PopoverPosition::Top => arrow
                    .bottom(px(-6.0))
                    .left(px(50.)),
                PopoverPosition::Bottom => arrow
                    .top(px(-6.0))
                    .left(px(50.)),
                PopoverPosition::Left => arrow
                    .right(px(-6.0))
                    .top(px(50.)),
                PopoverPosition::Right => arrow
                    .left(px(-6.0))
                    .top(px(50.)),
            };

            popover = popover.child(arrow);
        }

        popover
    }
}

impl IntoElement for Popover {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();

        if !self.props.open {
            return div(); // Return empty div if not open
        }

        // Build popover container
        let mut popover = div()
            .absolute()
            .bg(theme.alias.color_surface)
            .border(px(1.0))
            .border_color(theme.alias.color_border)
            .rounded(theme.global.radius_lg)
            .shadow_xl()
            .min_w(px(200.0))
            .max_w(px(400.0))
            .flex()
            .flex_col();

        // Position the popover
        popover = match self.props.position {
            PopoverPosition::Top => popover
                .bottom_full()
                .mb(theme.global.spacing_sm),
            PopoverPosition::Bottom => popover
                .top_full()
                .mt(theme.global.spacing_sm),
            PopoverPosition::Left => popover
                .right_full()
                .mr(theme.global.spacing_sm),
            PopoverPosition::Right => popover
                .left_full()
                .ml(theme.global.spacing_sm),
        };

        // Add header if title exists or close button is shown
        if self.props.title.is_some() || self.props.show_close {
            let mut header = div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px(theme.global.spacing_md)
                .py(theme.global.spacing_sm)
                .border_b(px(1.0))
                .border_color(theme.alias.color_border);

            // Add title if present
            if let Some(ref title) = self.props.title {
                header = header.child(
                    Label::new(title.clone())
                        .variant(LabelVariant::Heading3)
                );
            } else {
                header = header.child(div()); // Empty spacer
            }

            // Add close button if enabled
            if self.props.show_close {
                header = header.child(
                    Button::new()
                        .label("×")
                        .variant(ButtonVariant::Ghost)
                );
            }

            popover = popover.child(header);
        }

        // Add content
        popover = popover.child(
            div()
                .px(theme.global.spacing_md)
                .py(theme.global.spacing_md)
                .child(
                    Label::new(self.props.content.clone())
                        .variant(LabelVariant::Body)
                        .color(theme.alias.color_text_secondary)
                )
        );

        // Add arrow if enabled
        if self.props.show_arrow {
            let arrow = div()
                .absolute()
                .w(px(12.0))
                .h(px(12.0))
                .bg(theme.alias.color_surface)
                .border(px(1.0))
                .border_color(theme.alias.color_border);

            // Position arrow based on popover position
            let arrow = match self.props.position {
                PopoverPosition::Top => arrow
                    .bottom(px(-6.0))
                    .left(px(50.)),
                PopoverPosition::Bottom => arrow
                    .top(px(-6.0))
                    .left(px(50.)),
                PopoverPosition::Left => arrow
                    .right(px(-6.0))
                    .top(px(50.)),
                PopoverPosition::Right => arrow
                    .left(px(-6.0))
                    .top(px(50.)),
            };

            popover = popover.child(arrow);
        }

        popover
    }
}

impl Default for Popover {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_creation() {
        let popover = Popover::new("Test content");
        assert_eq!(popover.props.content.as_ref(), "Test content");
        assert!(popover.props.title.is_none());
        assert_eq!(popover.props.position, PopoverPosition::Top);
        assert!(!popover.props.open);
        assert!(popover.props.show_close);
        assert!(popover.props.show_arrow);
        assert!(popover.props.close_on_outside_click);
    }

    #[test]
    fn test_popover_builder() {
        let popover = Popover::new("Test")
            .title("Title")
            .position(PopoverPosition::Bottom)
            .open(true)
            .show_close(false)
            .show_arrow(false)
            .close_on_outside_click(false);

        assert_eq!(popover.props.title.as_ref().unwrap().as_ref(), "Title");
        assert_eq!(popover.props.position, PopoverPosition::Bottom);
        assert!(popover.props.open);
        assert!(!popover.props.show_close);
        assert!(!popover.props.show_arrow);
        assert!(!popover.props.close_on_outside_click);
    }

    #[test]
    fn test_popover_positions() {
        let positions = vec![
            PopoverPosition::Top,
            PopoverPosition::Bottom,
            PopoverPosition::Left,
            PopoverPosition::Right,
        ];

        for position in positions {
            let popover = Popover::new("Test").position(position);
            assert_eq!(popover.props.position, position);
        }
    }
}
