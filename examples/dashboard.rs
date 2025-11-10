//! Dashboard demo application showing layout and data visualization.
//!
//! This example demonstrates:
//! - Dashboard layout patterns
//! - Cards for metrics
//! - Tables for data
//! - Navigation
//!
//! Run with: `cargo run --example dashboard`

use gpui::*;
use purdah_gpui_components::prelude::*;

struct DashboardApp {
    selected_nav: SharedString,
}

impl DashboardApp {
    fn new() -> Self {
        Self {
            selected_nav: "overview".into(),
        }
    }
}

impl Render for DashboardApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::light();

        div()
            .flex()
            .flex_row()
            .w_full()
            .h_full()
            .bg(theme.alias.color_background)
            // Sidebar
            .child(self.render_sidebar(&theme))
            // Main content
            .child(self.render_main_content(&theme))
    }
}

impl DashboardApp {
    fn render_sidebar(&self, theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w(px(250.0))
            .h_full()
            .bg(theme.alias.color_surface)
            .border_r(px(1.0))
            .border_color(theme.alias.color_border)
            .p(theme.global.spacing_lg)
            .child(
                VStack::new()
                    .gap(theme.global.spacing_md)
                    // Logo/Title
                    .child(
                        Label::new("Dashboard")
                            .variant(LabelVariant::Heading2)
                    )
                    .child(
                        Divider::new()
                            .orientation(DividerOrientation::Horizontal)
                    )
                    // Navigation items
                    .child(
                        VStack::new()
                            .gap(theme.global.spacing_xs)
                            .child(self.nav_item(theme, "Overview", "overview", icons::HOME))
                            .child(self.nav_item(theme, "Analytics", "analytics", icons::ARROW_UP))
                            .child(self.nav_item(theme, "Users", "users", icons::USER))
                            .child(self.nav_item(theme, "Settings", "settings", icons::SETTINGS))
                    )
            )
    }

    fn nav_item(&self, theme: &Theme, label: &str, value: &str, icon: &'static str) -> impl IntoElement {
        let is_selected = self.selected_nav.as_ref() == value;

        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme.global.spacing_sm)
            .px(theme.global.spacing_md)
            .py(theme.global.spacing_sm)
            .rounded(theme.global.radius_md)
            .cursor_pointer()
            .when(is_selected, |d| {
                d.bg(theme.alias.color_primary)
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0))
            })
            .when(!is_selected, |d| {
                d.text_color(theme.alias.color_text_secondary)
                    .hover(|style| {
                        style.bg(theme.alias.color_background_hover)
                    })
            })
            .child(Icon::new(icon))
            .child(Label::new(label).variant(LabelVariant::Body))
    }

    fn render_main_content(&self, theme: &Theme) -> impl IntoElement {
        div()
            .flex_1()
            .h_full()
            .overflow_y_scroll()
            .child(
                VStack::new()
                    .gap(theme.global.spacing_xl)
                    // Header
                    .child(
                        div()
                            .px(theme.global.spacing_xl)
                            .py(theme.global.spacing_lg)
                            .border_b(px(1.0))
                            .border_color(theme.alias.color_border)
                            .child(
                                HStack::new()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        Label::new("Overview")
                                            .variant(LabelVariant::Heading1)
                                    )
                                    .child(
                                        HStack::new()
                                            .gap(theme.global.spacing_sm)
                                            .child(
                                                SearchBar::new()
                                                    .placeholder("Search...")
                                            )
                                            .child(
                                                Avatar::new("JD")
                                                    .size(AvatarSize::Md)
                                                    .status(AvatarStatus::Online)
                                            )
                                    )
                            )
                    )
                    // Content
                    .child(
                        div()
                            .px(theme.global.spacing_xl)
                            .child(
                                VStack::new()
                                    .gap(theme.global.spacing_xl)
                                    // Metrics cards
                                    .child(self.render_metrics(&theme))
                                    // Recent activity
                                    .child(self.render_recent_activity(&theme))
                            )
                    )
            )
    }

    fn render_metrics(&self, theme: &Theme) -> impl IntoElement {
        div()
            .child(
                Label::new("Key Metrics")
                    .variant(LabelVariant::Heading2)
            )
            .child(
                HStack::new()
                    .gap(theme.global.spacing_lg)
                    .mt(theme.global.spacing_md)
                    .child(self.metric_card(theme, "Total Users", "12,345", "+12%", true))
                    .child(self.metric_card(theme, "Revenue", "$54,321", "+8%", true))
                    .child(self.metric_card(theme, "Active Sessions", "1,234", "-3%", false))
            )
    }

    fn metric_card(&self, theme: &Theme, title: &str, value: &str, change: &str, is_positive: bool) -> impl IntoElement {
        Card::new()
            .variant(CardVariant::Outlined)
            .hoverable(true)
            .child(
                VStack::new()
                    .gap(theme.global.spacing_sm)
                    .child(
                        Label::new(title)
                            .variant(LabelVariant::Body)
                            .color(theme.alias.color_text_secondary)
                    )
                    .child(
                        Label::new(value)
                            .variant(LabelVariant::Heading1)
                    )
                    .child(
                        HStack::new()
                            .items_center()
                            .gap(theme.global.spacing_xs)
                            .child(
                                Badge::new(change)
                                    .variant(if is_positive {
                                        BadgeVariant::Success
                                    } else {
                                        BadgeVariant::Error
                                    })
                            )
                            .child(
                                Label::new("vs last month")
                                    .variant(LabelVariant::Caption)
                                    .color(theme.alias.color_text_muted)
                            )
                    )
            )
    }

    fn render_recent_activity(&self, theme: &Theme) -> impl IntoElement {
        Card::new()
            .title("Recent Activity")
            .variant(CardVariant::Outlined)
            .child(
                VStack::new()
                    .gap(theme.global.spacing_md)
                    .child(self.activity_item(theme, "New user registration", "2 minutes ago"))
                    .child(Divider::new().orientation(DividerOrientation::Horizontal))
                    .child(self.activity_item(theme, "Payment received", "15 minutes ago"))
                    .child(Divider::new().orientation(DividerOrientation::Horizontal))
                    .child(self.activity_item(theme, "System update completed", "1 hour ago"))
                    .child(Divider::new().orientation(DividerOrientation::Horizontal))
                    .child(self.activity_item(theme, "New feature deployed", "3 hours ago"))
            )
    }

    fn activity_item(&self, theme: &Theme, title: &str, time: &str) -> impl IntoElement {
        HStack::new()
            .items_center()
            .justify_between()
            .child(
                VStack::new()
                    .child(
                        Label::new(title)
                            .variant(LabelVariant::Body)
                    )
                    .child(
                        Label::new(time)
                            .variant(LabelVariant::Caption)
                            .color(theme.alias.color_text_muted)
                    )
            )
            .child(
                Icon::new(icons::ARROW_RIGHT)
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| DashboardApp::new())
        })
        .unwrap();
    });
}
