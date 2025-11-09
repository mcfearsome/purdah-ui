//! Dialog modal component.

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::{
    atoms::{Label, LabelVariant, Button, ButtonVariant},
    theme::Theme,
};

/// Dialog configuration properties
#[derive(Clone)]
pub struct DialogProps {
    /// Dialog title
    pub title: SharedString,
    /// Dialog description/content
    pub description: Option<SharedString>,
    /// Whether dialog is open
    pub open: bool,
}

impl Default for DialogProps {
    fn default() -> Self {
        Self {
            title: "".into(),
            description: None,
            open: false,
        }
    }
}

/// A modal dialog component.
///
/// Dialog creates a modal overlay with title, content, and action buttons.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::organisms::*;
///
/// // Basic dialog
/// Dialog::new()
///     .title("Confirm")
///     .description("Are you sure?")
///     .open(true);
///
/// // Dialog with custom content
/// Dialog::new()
///     .title("Settings")
///     .open(true);
/// ```
pub struct Dialog {
    props: DialogProps,
}

impl Dialog {
    /// Create a new dialog
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let dialog = Dialog::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: DialogProps::default(),
        }
    }

    /// Set the dialog title
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Dialog::new().title("Confirm Action");
    /// ```
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.props.title = title.into();
        self
    }

    /// Set the dialog description
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Dialog::new().description("Are you sure you want to continue?");
    /// ```
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.props.description = Some(description.into());
        self
    }

    /// Set whether the dialog is open
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// Dialog::new().open(true);
    /// ```
    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }
}

impl Render for Dialog {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        if !self.props.open {
            return div(); // Return empty div if not open
        }

        // Build dialog overlay and content
        div()
            .fixed()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(hsla(0.0, 0.0, 0.0, 0.5)) // Semi-transparent overlay
            .child(
                // Dialog panel
                div()
                    .bg(theme.alias.color_surface)
                    .rounded(theme.global.radius_lg)
                    .p(theme.global.spacing_lg)
                    .min_w(px(400.0))
                    .max_w(px(600.0))
                    .shadow_lg()
                    .flex()
                    .flex_col()
                    .gap(theme.global.spacing_md)
                    .child(
                        // Title
                        Label::new(self.props.title.clone())
                            .variant(LabelVariant::Heading2)
                    )
                    .when_some(self.props.description.clone(), |div, desc| {
                        div.child(
                            Label::new(desc)
                                .variant(LabelVariant::Body)
                                .color(theme.alias.color_text_secondary)
                        )
                    })
                    .child(
                        // Action buttons
                        div()
                            .flex()
                            .flex_row()
                            .gap(theme.global.spacing_sm)
                            .justify_end()
                            .child(
                                Button::new()
                                    .label("Cancel")
                                    .variant(ButtonVariant::Outline)
                            )
                            .child(
                                Button::new()
                                    .label("Confirm")
                                    .variant(ButtonVariant::Primary)
                            )
                    )
            )
    }
}
