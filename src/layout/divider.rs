//! Divider component for visual separation.

use gpui::*;
use crate::theme::Theme;

/// Divider orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DividerOrientation {
    /// Horizontal divider (default)
    #[default]
    Horizontal,
    /// Vertical divider
    Vertical,
}

/// A divider component for visual separation
///
/// Divider creates a line to separate content sections.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::layout::*;
///
/// // Horizontal divider
/// Divider::new();
///
/// // Vertical divider
/// Divider::new()
///     .orientation(DividerOrientation::Vertical);
/// ```
pub struct Divider {
    orientation: DividerOrientation,
}

impl Divider {
    /// Create a new horizontal divider
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let divider = Divider::new();
    /// ```
    pub fn new() -> Self {
        Self {
            orientation: DividerOrientation::default(),
        }
    }

    /// Set the divider orientation
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Divider::new().orientation(DividerOrientation::Vertical);
    /// ```
    pub fn orientation(mut self, orientation: DividerOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Render for Divider {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();
        let color = theme.alias.color_border;

        match self.orientation {
            DividerOrientation::Horizontal => {
                div()
                    .w_full()
                    .h(px(1.0))
                    .bg(color)
            }
            DividerOrientation::Vertical => {
                div()
                    .h_full()
                    .w(px(1.0))
                    .bg(color)
            }
        }
    }
}
