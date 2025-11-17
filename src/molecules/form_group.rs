//! FormGroup component combining label, input, and error message.

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::{atoms::{Label, LabelVariant, Input}, theme::Theme};

/// FormGroup configuration properties
#[derive(Clone)]
pub struct FormGroupProps {
    /// Label text
    pub label: SharedString,
    /// Whether field is required
    pub required: bool,
    /// Optional helper text
    pub helper_text: Option<SharedString>,
    /// Optional error message
    pub error_message: Option<SharedString>,
    /// Input value
    pub value: SharedString,
    /// Input placeholder
    pub placeholder: SharedString,
}

impl Default for FormGroupProps {
    fn default() -> Self {
        Self {
            label: "".into(),
            required: false,
            helper_text: None,
            error_message: None,
            value: "".into(),
            placeholder: "".into(),
        }
    }
}

/// A form group component combining label, input, and validation.
///
/// FormGroup provides a complete form field with label, input, helper text,
/// and error message display.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::molecules::*;
///
/// // Basic form group
/// FormGroup::new()
///     .label("Email")
///     .placeholder("you@example.com");
///
/// // Required field with helper text
/// FormGroup::new()
///     .label("Password")
///     .required(true)
///     .helper_text("Must be at least 8 characters");
///
/// // Field with error
/// FormGroup::new()
///     .label("Username")
///     .error_message("Username is required");
/// ```
pub struct FormGroup {
    props: FormGroupProps,
}

impl FormGroup {
    /// Create a new form group
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// let form_group = FormGroup::new();
    /// ```
    pub fn new() -> Self {
        Self {
            props: FormGroupProps::default(),
        }
    }

    /// Set the label text
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// FormGroup::new().label("Email");
    /// ```
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.props.label = label.into();
        self
    }

    /// Set whether the field is required
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// FormGroup::new().required(true);
    /// ```
    pub fn required(mut self, required: bool) -> Self {
        self.props.required = required;
        self
    }

    /// Set helper text
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// FormGroup::new().helper_text("Optional hint");
    /// ```
    pub fn helper_text(mut self, text: impl Into<SharedString>) -> Self {
        self.props.helper_text = Some(text.into());
        self
    }

    /// Set error message
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// FormGroup::new().error_message("Field is required");
    /// ```
    pub fn error_message(mut self, message: impl Into<SharedString>) -> Self {
        self.props.error_message = Some(message.into());
        self
    }

    /// Set the input value
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// FormGroup::new().value("example@email.com");
    /// ```
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.props.value = value.into();
        self
    }

    /// Set the input placeholder
    ///
    /// ## Example
    ///
    /// ```rust,ignore
    /// FormGroup::new().placeholder("Enter text...");
    /// ```
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.props.placeholder = placeholder.into();
        self
    }
}

impl Render for FormGroup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();
        let has_error = self.props.error_message.is_some();

        // Build form group container
        div()
            .flex()
            .flex_col()
            .gap(theme.global.spacing_xs)
            .child(
                // Label with optional required indicator
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .child(
                        Label::new(self.props.label.clone())
                            .variant(LabelVariant::Body)
                    )
                    .when(self.props.required, |div| {
                        div.child(
                            Label::new("*")
                                .variant(LabelVariant::Body)
                                .color(theme.alias.color_danger)
                        )
                    })
            )
            .child(
                // Input field
                Input::new()
                    .value(self.props.value.clone())
                    .placeholder(self.props.placeholder.clone())
                    .error(has_error)
                    .when_some(self.props.error_message.clone(), |input, msg| {
                        input.error_message(msg)
                    })
            )
            .when_some(self.props.helper_text.clone(), |div, text| {
                div.child(
                    Label::new(text)
                        .variant(LabelVariant::Caption)
                        .color(theme.alias.color_text_muted)
                )
            })
    }
}

impl IntoElement for FormGroup {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();
        let has_error = self.props.error_message.is_some();

        // Build form group container
        div()
            .flex()
            .flex_col()
            .gap(theme.global.spacing_xs)
            .child(
                // Label with optional required indicator
                div()
                    .flex()
                    .flex_row()
                    .gap(px(4.0))
                    .child(
                        Label::new(self.props.label.clone())
                            .variant(LabelVariant::Body)
                    )
                    .when(self.props.required, |div| {
                        div.child(
                            Label::new("*")
                                .variant(LabelVariant::Body)
                                .color(theme.alias.color_danger)
                        )
                    })
            )
            .child(
                // Input field
                Input::new()
                    .value(self.props.value.clone())
                    .placeholder(self.props.placeholder.clone())
                    .error(has_error)
                    .when_some(self.props.error_message.clone(), |input, msg| {
                        input.error_message(msg)
                    })
            )
            .when_some(self.props.helper_text.clone(), |div, text| {
                div.child(
                    Label::new(text)
                        .variant(LabelVariant::Caption)
                        .color(theme.alias.color_text_muted)
                )
            })
    }
}
