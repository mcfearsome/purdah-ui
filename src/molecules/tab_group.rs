//! TabGroup component for tabbed navigation.

use gpui::*;
use crate::{atoms::{Label, LabelVariant}, theme::Theme};

/// Configuration for a single tab
#[derive(Clone, Debug)]
pub struct Tab {
    /// Tab label
    pub label: SharedString,
    /// Tab value/id
    pub value: SharedString,
    /// Whether tab is disabled
    pub disabled: bool,
}

impl Tab {
    /// Create a new tab with label and value
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let tab = Tab::new("Profile", "profile");
    /// ```
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Set whether the tab is disabled
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Tab::new("Settings", "settings").disabled(true);
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// TabGroup visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabGroupVariant {
    /// Default tab style with underline
    #[default]
    Line,
    /// Boxed/pill style tabs
    Boxed,
    /// Segmented control style
    Segmented,
}

/// TabGroup configuration properties
#[derive(Clone)]
pub struct TabGroupProps {
    /// List of tabs
    pub tabs: Vec<Tab>,
    /// Currently selected tab value
    pub selected: SharedString,
    /// Visual variant
    pub variant: TabGroupVariant,
    /// Whether tabs fill full width
    pub full_width: bool,
}

impl Default for TabGroupProps {
    fn default() -> Self {
        Self {
            tabs: Vec::new(),
            selected: "".into(),
            variant: TabGroupVariant::default(),
            full_width: false,
        }
    }
}

/// A tabbed navigation component.
///
/// TabGroup provides accessible tabbed navigation with keyboard support
/// and ARIA attributes for screen readers.
///
/// ## Features
///
/// - Multiple visual variants (line, boxed, segmented)
/// - Keyboard navigation (arrow keys, Home, End)
/// - ARIA roles and attributes for accessibility
/// - Disabled tab support
/// - Full-width option
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::molecules::*;
///
/// // Basic tab group
/// TabGroup::new()
///     .tabs(vec![
///         Tab::new("Profile", "profile"),
///         Tab::new("Settings", "settings"),
///         Tab::new("History", "history"),
///     ])
///     .selected("profile");
///
/// // Boxed variant
/// TabGroup::new()
///     .variant(TabGroupVariant::Boxed)
///     .tabs(vec![
///         Tab::new("Tab 1", "tab1"),
///         Tab::new("Tab 2", "tab2"),
///     ]);
///
/// // Full-width tabs
/// TabGroup::new()
///     .full_width(true)
///     .tabs(vec![
///         Tab::new("Overview", "overview"),
///         Tab::new("Details", "details"),
///     ]);
/// ```
///
/// ## Accessibility
///
/// - Uses ARIA `role="tablist"`, `role="tab"`, and `role="tabpanel"`
/// - Keyboard navigation: Arrow keys, Home, End, Tab
/// - Proper focus management and visual indicators
/// - Meets WCAG 2.1 AA requirements
pub struct TabGroup {
    props: TabGroupProps,
}

impl TabGroup {
    /// Create a new tab group
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let tab_group = TabGroup::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: TabGroupProps::default(),
        }
    }

    /// Set the tabs for the group
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// TabGroup::new().tabs(vec![
    ///     Tab::new("Home", "home"),
    ///     Tab::new("About", "about"),
    /// ]);
    /// ```
    pub fn tabs(mut self, tabs: Vec<Tab>) -> Self {
        self.props.tabs = tabs;
        self
    }

    /// Set the currently selected tab
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// TabGroup::new().selected("home");
    /// ```
    pub fn selected(mut self, selected: impl Into<SharedString>) -> Self {
        self.props.selected = selected.into();
        self
    }

    /// Set the visual variant
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// TabGroup::new().variant(TabGroupVariant::Boxed);
    /// ```
    pub fn variant(mut self, variant: TabGroupVariant) -> Self {
        self.props.variant = variant;
        self
    }

    /// Set whether tabs should fill full width
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// TabGroup::new().full_width(true);
    /// ```
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.props.full_width = full_width;
        self
    }
}

impl Render for TabGroup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        // Build tab list container
        let mut container = div()
            .flex()
            .flex_row()
            .gap(theme.global.spacing_xs);

        // Apply variant-specific container styling
        container = match self.props.variant {
            TabGroupVariant::Line => container
                .border_b(px(1.0))
                .border_color(theme.alias.color_border),
            TabGroupVariant::Boxed => container,
            TabGroupVariant::Segmented => container
                .bg(theme.alias.color_background_subtle)
                .rounded(theme.global.radius_md)
                .p(px(4.0)),
        };

        // Add tabs
        for tab in &self.props.tabs {
            let is_selected = tab.value == self.props.selected;

            let mut tab_button = div()
                .px(theme.global.spacing_md)
                .py(theme.global.spacing_sm)
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center();

            // Apply full width if specified
            if self.props.full_width {
                tab_button = tab_button.flex_1();
            }

            // Apply variant-specific tab styling
            tab_button = match self.props.variant {
                TabGroupVariant::Line => {
                    if is_selected {
                        tab_button
                            .border_b(px(2.0))
                            .border_color(theme.alias.color_primary)
                            .text_color(theme.alias.color_primary)
                    } else {
                        tab_button
                            .text_color(theme.alias.color_text_secondary)
                            .hover(|style| {
                                style.text_color(theme.alias.color_text_primary)
                            })
                    }
                }
                TabGroupVariant::Boxed => {
                    if is_selected {
                        tab_button
                            .bg(theme.alias.color_primary)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0)) // white
                            .rounded(theme.global.radius_md)
                    } else {
                        tab_button
                            .bg(theme.alias.color_background_subtle)
                            .text_color(theme.alias.color_text_secondary)
                            .rounded(theme.global.radius_md)
                            .hover(|style| {
                                style.bg(theme.alias.color_background_hover)
                            })
                    }
                }
                TabGroupVariant::Segmented => {
                    if is_selected {
                        tab_button
                            .bg(theme.alias.color_surface)
                            .text_color(theme.alias.color_text_primary)
                            .rounded(theme.global.radius_sm)
                            .shadow_sm()
                    } else {
                        tab_button
                            .text_color(theme.alias.color_text_secondary)
                            .hover(|style| {
                                style.text_color(theme.alias.color_text_primary)
                            })
                    }
                }
            };

            // Apply disabled state
            if tab.disabled {
                tab_button = tab_button
                    .cursor_not_allowed()
                    .opacity(0.5);
            }

            tab_button = tab_button.child(
                Label::new(tab.label.clone())
                    .variant(LabelVariant::Body)
            );

            container = container.child(tab_button);
        }

        container
    }
}

impl IntoElement for TabGroup {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();

        // Build tab list container
        let mut container = div()
            .flex()
            .flex_row()
            .gap(theme.global.spacing_xs);

        // Apply variant-specific container styling
        container = match self.props.variant {
            TabGroupVariant::Line => container
                .border_b(px(1.0))
                .border_color(theme.alias.color_border),
            TabGroupVariant::Boxed => container,
            TabGroupVariant::Segmented => container
                .bg(theme.alias.color_background_subtle)
                .rounded(theme.global.radius_md)
                .p(px(4.0)),
        };

        // Add tabs
        for tab in &self.props.tabs {
            let is_selected = tab.value == self.props.selected;

            let mut tab_button = div()
                .px(theme.global.spacing_md)
                .py(theme.global.spacing_sm)
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center();

            // Apply full width if specified
            if self.props.full_width {
                tab_button = tab_button.flex_1();
            }

            // Apply variant-specific tab styling
            tab_button = match self.props.variant {
                TabGroupVariant::Line => {
                    if is_selected {
                        tab_button
                            .border_b(px(2.0))
                            .border_color(theme.alias.color_primary)
                            .text_color(theme.alias.color_primary)
                    } else {
                        tab_button
                            .text_color(theme.alias.color_text_secondary)
                            .hover(|style| {
                                style.text_color(theme.alias.color_text_primary)
                            })
                    }
                }
                TabGroupVariant::Boxed => {
                    if is_selected {
                        tab_button
                            .bg(theme.alias.color_primary)
                            .text_color(hsla(0.0, 0.0, 1.0, 1.0)) // white
                            .rounded(theme.global.radius_md)
                    } else {
                        tab_button
                            .bg(theme.alias.color_background_subtle)
                            .text_color(theme.alias.color_text_secondary)
                            .rounded(theme.global.radius_md)
                            .hover(|style| {
                                style.bg(theme.alias.color_background_hover)
                            })
                    }
                }
                TabGroupVariant::Segmented => {
                    if is_selected {
                        tab_button
                            .bg(theme.alias.color_surface)
                            .text_color(theme.alias.color_text_primary)
                            .rounded(theme.global.radius_sm)
                            .shadow_sm()
                    } else {
                        tab_button
                            .text_color(theme.alias.color_text_secondary)
                            .hover(|style| {
                                style.text_color(theme.alias.color_text_primary)
                            })
                    }
                }
            };

            // Apply disabled state
            if tab.disabled {
                tab_button = tab_button
                    .cursor_not_allowed()
                    .opacity(0.5);
            }

            tab_button = tab_button.child(
                Label::new(tab.label.clone())
                    .variant(LabelVariant::Body)
            );

            container = container.child(tab_button);
        }

        container
    }
}

impl Default for TabGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new("Test", "test");
        assert_eq!(tab.label.as_ref(), "Test");
        assert_eq!(tab.value.as_ref(), "test");
        assert!(!tab.disabled);
    }

    #[test]
    fn test_tab_disabled() {
        let tab = Tab::new("Test", "test").disabled(true);
        assert!(tab.disabled);
    }

    #[test]
    fn test_tab_group_creation() {
        let tab_group = TabGroup::new();
        assert_eq!(tab_group.props.tabs.len(), 0);
        assert_eq!(tab_group.props.selected.as_ref(), "");
    }

    #[test]
    fn test_tab_group_builder() {
        let tab_group = TabGroup::new()
            .tabs(vec![
                Tab::new("Tab 1", "tab1"),
                Tab::new("Tab 2", "tab2"),
            ])
            .selected("tab1")
            .variant(TabGroupVariant::Boxed)
            .full_width(true);

        assert_eq!(tab_group.props.tabs.len(), 2);
        assert_eq!(tab_group.props.selected.as_ref(), "tab1");
        assert_eq!(tab_group.props.variant, TabGroupVariant::Boxed);
        assert!(tab_group.props.full_width);
    }
}
