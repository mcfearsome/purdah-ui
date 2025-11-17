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
            .bg(theme.alias.color_surface)
            .child(
                // Card-styled container (max-width wrapper)
                div()
                    .w_full()
                    .max_w(px(500.0))
                    .child(
                        // Card content with elevated styling
                        div()
                            .bg(theme.alias.color_surface)
                            .rounded(theme.global.radius_lg)
                            .p(theme.global.spacing_lg)
                            .shadow_lg()
                            .border_color(theme.alias.color_border)
                            .border(px(1.0))
                            .flex()
                            .flex_col()
                            .gap(theme.global.spacing_md)
                            // Card title
                            .child(
                                Label::new("Create Account")
                                    .variant(LabelVariant::Heading3)
                            )
                            .child(
                                VStack::new()
                                    .gap(theme.global.spacing_lg)
                                    // Email field
                                    .child(
                                        FormGroup::new()
                                            .label("Email Address")
                                            .required(true)
                                            .placeholder("you@example.com")
                                    )
                                    // Password field
                                    .child(
                                        FormGroup::new()
                                            .label("Password")
                                            .required(true)
                                            .helper_text("Must be at least 8 characters")
                                            .placeholder("••••••••")
                                    )
                                    // Confirm password field
                                    .child(
                                        FormGroup::new()
                                            .label("Confirm Password")
                                            .required(true)
                                            .placeholder("••••••••")
                                    )
                                    // Account type - using Input instead of Dropdown for now
                                    .child(
                                        FormGroup::new()
                                            .label("Account Type")
                                            .placeholder("Select account type")
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
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(600.), px(800.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| FormDemoApp::new()),
        )
        .unwrap();
    });
}
