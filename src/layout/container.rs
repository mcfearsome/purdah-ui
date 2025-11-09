//! Container component for max-width and centering.

use gpui::*;

/// A container component with max-width and centering
///
/// Container provides a centered layout with optional maximum width.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::layout::*;
///
/// Container::new()
///     .max_width(px(1200.0))
///     .centered(true)
///     .child(content);
/// ```
pub struct Container {
    max_width: Option<Pixels>,
    centered: bool,
    padding: Option<Pixels>,
}

impl Container {
    /// Create a new container
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let container = Container::new();
    /// ```
    pub fn new() -> Self {
        Self {
            max_width: None,
            centered: false,
            padding: None,
        }
    }

    /// Set the maximum width
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Container::new().max_width(px(1200.0));
    /// ```
    pub fn max_width(mut self, width: Pixels) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set whether the container should be centered
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Container::new().centered(true);
    /// ```
    pub fn centered(mut self, centered: bool) -> Self {
        self.centered = centered;
        self
    }

    /// Set the padding
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Container::new().padding(px(16.0));
    /// ```
    pub fn padding(mut self, padding: Pixels) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Convert to a GPUI div with container layout
    pub fn to_element(self) -> Div {
        let mut element = div()
            .w_full();

        // Apply max width
        if let Some(max_width) = self.max_width {
            element = element.max_w(max_width);
        }

        // Apply centering
        if self.centered {
            element = element.mx_auto();
        }

        // Apply padding
        if let Some(padding) = self.padding {
            element = element.p(padding);
        }

        element
    }
}
