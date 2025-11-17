//! Stack layout components for vertical and horizontal arrangement.

use gpui::*;

/// Alignment options for cross-axis alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Align to start (top for VStack, left for HStack)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// Align to end (bottom for VStack, right for HStack)
    End,
    /// Stretch to fill
    Stretch,
}

/// Justify options for main-axis alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Justify {
    /// Justify to start
    #[default]
    Start,
    /// Center justify
    Center,
    /// Justify to end
    End,
    /// Space between items
    Between,
    /// Space around items
    Around,
}

/// Vertical stack layout component
///
/// VStack arranges children vertically with configurable gap and alignment.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::layout::*;
///
/// VStack::new()
///     .gap(px(16.0))
///     .align(Alignment::Center)
///     .children(vec![
///         Label::new("Title"),
///         Input::new(),
///         Button::new().label("Submit"),
///     ]);
/// ```
pub struct VStack {
    gap: Option<Pixels>,
    align: Alignment,
    justify: Justify,
    children: Vec<AnyElement>,
}

impl VStack {
    /// Create a new vertical stack
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let stack = VStack::new();
    /// ```
    pub fn new() -> Self {
        Self {
            gap: None,
            align: Alignment::default(),
            justify: Justify::default(),
            children: Vec::new(),
        }
    }

    /// Set the gap between children
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// VStack::new().gap(px(16.0));
    /// ```
    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Set the cross-axis alignment
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// VStack::new().align(Alignment::Center);
    /// ```
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    /// Set the main-axis justification
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// VStack::new().justify(Justify::Between);
    /// ```
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// Add a child element
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// VStack::new().child(Label::new("Hello"));
    /// ```
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Add multiple children
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// VStack::new().children(vec![
    ///     Label::new("First"),
    ///     Label::new("Second"),
    /// ]);
    /// ```
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    /// Convert to a GPUI div with flex column layout
    pub fn to_element(self) -> Div {
        let mut element = div()
            .flex()
            .flex_col();

        // Apply gap
        if let Some(gap) = self.gap {
            element = element.gap(gap);
        }

        // Apply alignment (horizontal in column)
        element = match self.align {
            Alignment::Start => element.items_start(),
            Alignment::Center => element.items_center(),
            Alignment::End => element.items_end(),
            Alignment::Stretch => element.items_start(), // GPUI doesn't have items_stretch
        };

        // Apply justification (vertical in column)
        element = match self.justify {
            Justify::Start => element.justify_start(),
            Justify::Center => element.justify_center(),
            Justify::End => element.justify_end(),
            Justify::Between => element.justify_between(),
            Justify::Around => element.justify_start(), // GPUI doesn't have justify_around
        };

        // Add children
        for child in self.children {
            element = element.child(child);
        }

        element
    }
}

impl IntoElement for VStack {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        self.to_element()
    }
}

/// Horizontal stack layout component
///
/// HStack arranges children horizontally with configurable gap and alignment.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::layout::*;
///
/// HStack::new()
///     .gap(px(8.0))
///     .align(Alignment::Center)
///     .children(vec![
///         Icon::new(IconName::User),
///         Label::new("John Doe"),
///     ]);
/// ```
pub struct HStack {
    gap: Option<Pixels>,
    align: Alignment,
    justify: Justify,
    children: Vec<AnyElement>,
}

impl HStack {
    /// Create a new horizontal stack
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let stack = HStack::new();
    /// ```
    pub fn new() -> Self {
        Self {
            gap: None,
            align: Alignment::default(),
            justify: Justify::default(),
            children: Vec::new(),
        }
    }

    /// Set the gap between children
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// HStack::new().gap(px(8.0));
    /// ```
    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = Some(gap);
        self
    }

    /// Set the cross-axis alignment
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// HStack::new().align(Alignment::Center);
    /// ```
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }

    /// Set the main-axis justification
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// HStack::new().justify(Justify::Between);
    /// ```
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }

    /// Add a child element
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// HStack::new().child(Label::new("Hello"));
    /// ```
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Add multiple children
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// HStack::new().children(vec![
    ///     Label::new("First"),
    ///     Label::new("Second"),
    /// ]);
    /// ```
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children.extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    /// Convert to a GPUI div with flex row layout
    pub fn to_element(self) -> Div {
        let mut element = div()
            .flex()
            .flex_row();

        // Apply gap
        if let Some(gap) = self.gap {
            element = element.gap(gap);
        }

        // Apply alignment (vertical in row)
        element = match self.align {
            Alignment::Start => element.items_start(),
            Alignment::Center => element.items_center(),
            Alignment::End => element.items_end(),
            Alignment::Stretch => element.items_start(), // GPUI doesn't have items_stretch
        };

        // Apply justification (horizontal in row)
        element = match self.justify {
            Justify::Start => element.justify_start(),
            Justify::Center => element.justify_center(),
            Justify::End => element.justify_end(),
            Justify::Between => element.justify_between(),
            Justify::Around => element.justify_start(), // GPUI doesn't have justify_around
        };

        // Add children
        for child in self.children {
            element = element.child(child);
        }

        element
    }
}

impl IntoElement for HStack {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        self.to_element()
    }
}
