//! Form demo application showing form components and validation.
//!
//! This example demonstrates:
//! - Form inputs and controls
//! - Form groups with labels
//! - Validation states
//! - Submit buttons
//!
//! Run with: `cargo run --example form_demo`

use gpui::*;
use purdah_gpui_components::prelude::*;

struct FormDemoApp {
    email: String,
    password: String,
    remember_me: bool,
    notifications: bool,
    account_type: String,
}

impl FormDemoApp {
    fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            remember_me: false,
            notifications: true,
            account_type: "personal".to_string(),
        }
    }
}

impl Render for FormDemoApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::light();

        div()
            .flex()
            .items_center()
            .justify_center()
            .w_full()
            .h_full()
            .bg(theme.alias.color_background)
            .child(
                Container::new()
                    .child(
                        Card::new()
                            .title("Create Account")
                            .variant(CardVariant::Elevated)
                            .child(
                                VStack::new()
                                    .gap(theme.global.spacing_lg)
                                    // Email field
                                    .child(
                                        FormGroup::new()
                                            .label("Email Address")
                                            .required(true)
                                            .child(
                                                Input::new()
                                                    .placeholder("you@example.com")
                                            )
                                    )
                                    // Password field
                                    .child(
                                        FormGroup::new()
                                            .label("Password")
                                            .required(true)
                                            .help_text("Must be at least 8 characters")
                                            .child(
                                                Input::new()
                                                    .placeholder("••••••••")
                                            )
                                    )
                                    // Confirm password field
                                    .child(
                                        FormGroup::new()
                                            .label("Confirm Password")
                                            .required(true)
                                            .child(
                                                Input::new()
                                                    .placeholder("••••••••")
                                            )
                                    )
                                    // Account type dropdown
                                    .child(
                                        FormGroup::new()
                                            .label("Account Type")
                                            .child(
                                                Dropdown::new()
                                                    .options(vec![
                                                        DropdownOption::new("Personal", "personal"),
                                                        DropdownOption::new("Business", "business"),
                                                        DropdownOption::new("Enterprise", "enterprise"),
                                                    ])
                                                    .placeholder("Select account type")
                                            )
                                    )
                                    // Divider
                                    .child(
                                        Divider::new()
                                            .orientation(DividerOrientation::Horizontal)
                                    )
                                    // Checkbox options
                                    .child(
                                        VStack::new()
                                            .gap(theme.global.spacing_sm)
                                            .child(
                                                Checkbox::new()
                                                    .label("Remember me")
                                            )
                                            .child(
                                                Checkbox::new()
                                                    .label("Enable email notifications")
                                            )
                                            .child(
                                                Checkbox::new()
                                                    .label("I agree to the Terms and Conditions")
                                            )
                                    )
                                    // Submit buttons
                                    .child(
                                        HStack::new()
                                            .gap(theme.global.spacing_sm)
                                            .child(Spacer::new())
                                            .child(
                                                Button::new()
                                                    .label("Cancel")
                                                    .variant(ButtonVariant::Outline)
                                            )
                                            .child(
                                                Button::new()
                                                    .label("Create Account")
                                                    .variant(ButtonVariant::Primary)
                                            )
                                    )
                            )
                    )
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| FormDemoApp::new())
        })
        .unwrap();
    });
}
