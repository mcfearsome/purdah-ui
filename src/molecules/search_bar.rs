//! SearchBar component combining input with search functionality.

use gpui::*;
use crate::{atoms::{Input, Icon, IconSize, IconColor}, theme::Theme};

/// SearchBar configuration properties
#[derive(Clone)]
pub struct SearchBarProps {
    /// Search query value
    pub value: SharedString,
    /// Placeholder text
    pub placeholder: SharedString,
    /// Whether search is in loading state
    pub loading: bool,
}

impl Default for SearchBarProps {
    fn default() -> Self {
        Self {
            value: "".into(),
            placeholder: "Search...".into(),
            loading: false,
        }
    }
}

/// A search bar component with input and search icon.
///
/// SearchBar combines an input field with a search icon and optional loading state.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::molecules::*;
///
/// // Basic search bar
/// SearchBar::new();
///
/// // Search bar with placeholder
/// SearchBar::new()
///     .placeholder("Search documents...");
///
/// // Search bar with value
/// SearchBar::new()
///     .value("query")
///     .placeholder("Search...");
/// ```
pub struct SearchBar {
    props: SearchBarProps,
}

impl SearchBar {
    /// Create a new search bar
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let search_bar = SearchBar::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: SearchBarProps::default(),
        }
    }

    /// Set the search value
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// SearchBar::new().value("search query");
    /// ```
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.props.value = value.into();
        self
    }

    /// Set the placeholder text
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// SearchBar::new().placeholder("Search...");
    /// ```
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.props.placeholder = placeholder.into();
        self
    }

    /// Set the loading state
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// SearchBar::new().loading(true);
    /// ```
    pub fn loading(mut self, loading: bool) -> Self {
        self.props.loading = loading;
        self
    }
}

impl Render for SearchBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        // Build search bar container
        div()
            .relative()
            .flex()
            .items_center()
            .child(
                // Search icon on the left
                div()
                    .absolute()
                    .left(theme.global.spacing_sm)
                    .child(
                        Icon::new("M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z") // Search icon path
                            .size(IconSize::Sm)
                            .color(IconColor::Muted)
                    )
            )
            .child(
                // Input field with left padding for icon
                div()
                    .pl(theme.global.spacing_2xl) // Space for search icon
                    .child(
                        Input::new()
                            .value(self.props.value.clone())
                            .placeholder(self.props.placeholder.clone())
                    )
            )
    }
}

impl IntoElement for SearchBar {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();

        // Build search bar container
        div()
            .relative()
            .flex()
            .items_center()
            .child(
                // Search icon on the left
                div()
                    .absolute()
                    .left(theme.global.spacing_sm)
                    .child(
                        Icon::new("M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z") // Search icon path
                            .size(IconSize::Sm)
                            .color(IconColor::Muted)
                    )
            )
            .child(
                // Input field with left padding for icon
                div()
                    .pl(theme.global.spacing_2xl) // Space for search icon
                    .child(
                        Input::new()
                            .value(self.props.value.clone())
                            .placeholder(self.props.placeholder.clone())
                    )
            )
    }
}
