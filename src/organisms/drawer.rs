//! Drawer side panel component.

use gpui::*;
use crate::{atoms::{Label, LabelVariant, Button, ButtonVariant}, theme::Theme};

/// Drawer position variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPosition {
    /// Drawer from left side
    Left,
    /// Drawer from right side (default)
    #[default]
    Right,
}

/// Drawer configuration properties
#[derive(Clone)]
pub struct DrawerProps {
    /// Drawer title
    pub title: SharedString,
    /// Drawer position
    pub position: DrawerPosition,
    /// Whether drawer is open
    pub open: bool,
    /// Drawer width
    pub width: Pixels,
}

impl Default for DrawerProps {
    fn default() -> Self {
        Self {
            title: "".into(),
            position: DrawerPosition::default(),
            open: false,
            width: px(400.0),
        }
    }
}

/// A drawer side panel component.
///
/// Drawer creates a sliding panel from the side of the screen.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::organisms::*;
///
/// Drawer::new()
///     .title("Settings")
///     .position(DrawerPosition::Right)
///     .open(true);
/// ```
pub struct Drawer {
    props: DrawerProps,
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            props: DrawerProps::default(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.props.title = title.into();
        self
    }

    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.props.position = position;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.props.width = width;
        self
    }
}

impl Render for Drawer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        if !self.props.open {
            return div(); // Return empty div if not open
        }

        div()
            .fixed()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .flex()
            .flex_row()
            .child(
                // Overlay
                div()
                    .flex_1()
                    .bg(hsla(0.0, 0.0, 0.0, 0.5))
            )
            .child(
                // Drawer panel
                div()
                    .w(self.props.width)
                    .h_full()
                    .bg(theme.alias.color_surface)
                    .shadow_xl()
                    .flex()
                    .flex_col()
                    .child(
                        // Header
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .p(theme.global.spacing_lg)
                            .border_color(theme.alias.color_border)
                            .border_b(px(1.0))
                            .child(
                                Label::new(self.props.title.clone())
                                    .variant(LabelVariant::Heading2)
                            )
                            .child(
                                Button::new()
                                    .label("✕")
                                    .variant(ButtonVariant::Ghost)
                            )
                    )
                    .child(
                        // Content area
                        div()
                            .flex_1()
                            .p(theme.global.spacing_lg)
                            .child("Drawer content goes here")
                    )
            )
    }
}
