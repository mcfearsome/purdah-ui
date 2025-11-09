//! Spacer component for flexible spacing.

use gpui::*;

/// A flexible spacer component
///
/// Spacer creates flexible space in flex layouts, pushing siblings apart.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::layout::*;
///
/// // Create a toolbar with buttons on left and right
/// HStack::new()
///     .children(vec![
///         Button::new().label("Back"),
///         Spacer::new(), // Pushes next button to the right
///         Button::new().label("Forward"),
///     ]);
/// ```
pub struct Spacer {
    size: Option<Pixels>,
}

impl Spacer {
    /// Create a new flexible spacer
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let spacer = Spacer::new();
    /// ```
    pub fn new() -> Self {
        Self { size: None }
    }

    /// Create a spacer with a fixed size
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Spacer::fixed(px(16.0));
    /// ```
    pub fn fixed(size: Pixels) -> Self {
        Self { size: Some(size) }
    }
}

impl Render for Spacer {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        if let Some(size) = self.size {
            // Fixed size spacer
            div().size(size)
        } else {
            // Flexible spacer
            div().flex_1()
        }
    }
}
