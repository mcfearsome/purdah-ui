//! Showcase application demonstrating all Purdah GPUI components.
//!
//! This example provides an interactive demo of every component in the library,
//! organized by type (atoms, molecules, organisms, layout).
//!
//! Run with: `cargo run --example showcase`

use gpui::*;
use purdah_gpui_components::prelude::*;

/// Main showcase application state
struct ShowcaseApp {
    /// Currently selected tab
    selected_tab: SharedString,
    /// Whether dark mode is enabled
    dark_mode: bool,
}

impl ShowcaseApp {
    fn new() -> Self {
        Self {
            selected_tab: "atoms".into(),
            dark_mode: false,
        }
    }
}

impl Render for ShowcaseApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = if self.dark_mode {
            Theme::dark()
        } else {
            Theme::light()
        };

        div()
            .flex()
            .flex_col()
            .w_full()
            .h_full()
            .bg(theme.alias.color_background)
            // Header
            .child(self.render_header(&theme))
            // Navigation tabs
            .child(self.render_navigation(&theme))
            // Content area
            .child(self.render_content(&theme))
    }
}

impl ShowcaseApp {
    /// Render the application header
    fn render_header(&self, theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px(theme.global.spacing_xl)
            .py(theme.global.spacing_lg)
            .border_b(px(1.0))
            .border_color(theme.alias.color_border)
            .child(
                Label::new("Purdah GPUI Components Showcase")
                    .variant(LabelVariant::Heading1)
            )
            .child(
                Button::new()
                    .label(if self.dark_mode { "☀️ Light" } else { "🌙 Dark" })
                    .variant(ButtonVariant::Outline)
            )
    }

    /// Render the navigation tabs
    fn render_navigation(&self, theme: &Theme) -> impl IntoElement {
        div()
            .px(theme.global.spacing_xl)
            .py(theme.global.spacing_md)
            .child(
                TabGroup::new()
                    .tabs(vec![
                        Tab::new("Atoms", "atoms"),
                        Tab::new("Molecules", "molecules"),
                        Tab::new("Organisms", "organisms"),
                        Tab::new("Layout", "layout"),
                        Tab::new("Utils", "utils"),
                    ])
                    .selected(self.selected_tab.clone())
                    .variant(TabGroupVariant::Line)
            )
    }

    /// Render the main content area
    fn render_content(&self, theme: &Theme) -> impl IntoElement {
        div()
            .flex_1()
            .overflow_y_scroll()
            .px(theme.global.spacing_xl)
            .py(theme.global.spacing_lg)
            .child(match self.selected_tab.as_ref() {
                "atoms" => self.render_atoms_showcase(theme),
                "molecules" => self.render_molecules_showcase(theme),
                "organisms" => self.render_organisms_showcase(theme),
                "layout" => self.render_layout_showcase(theme),
                "utils" => self.render_utils_showcase(theme),
                _ => div(),
            })
    }

    /// Render atoms showcase
    fn render_atoms_showcase(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_xl)
            .child(self.showcase_section(theme, "Buttons", self.render_buttons(theme)))
            .child(self.showcase_section(theme, "Inputs", self.render_inputs(theme)))
            .child(self.showcase_section(theme, "Labels", self.render_labels(theme)))
            .child(self.showcase_section(theme, "Badges", self.render_badges(theme)))
            .child(self.showcase_section(theme, "Avatars", self.render_avatars(theme)))
            .child(self.showcase_section(theme, "Form Controls", self.render_form_controls(theme)))
            .child(self.showcase_section(theme, "Icons & Spinners", self.render_icons_spinners(theme)))
    }

    /// Render molecules showcase
    fn render_molecules_showcase(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_xl)
            .child(self.showcase_section(theme, "Search Bar", self.render_search_bar(theme)))
            .child(self.showcase_section(theme, "Form Group", self.render_form_group(theme)))
            .child(self.showcase_section(theme, "Card", self.render_card(theme)))
            .child(self.showcase_section(theme, "Tab Group", self.render_tab_group(theme)))
            .child(self.showcase_section(theme, "Dropdown", self.render_dropdown(theme)))
            .child(self.showcase_section(theme, "Tooltip", self.render_tooltip(theme)))
            .child(self.showcase_section(theme, "Popover", self.render_popover(theme)))
    }

    /// Render organisms showcase
    fn render_organisms_showcase(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_xl)
            .child(self.showcase_section(theme, "Dialog", self.render_dialog(theme)))
            .child(self.showcase_section(theme, "Drawer", self.render_drawer(theme)))
            .child(self.showcase_section(theme, "Table", self.render_table(theme)))
            .child(self.showcase_section(theme, "Command Palette", self.render_command_palette(theme)))
    }

    /// Render layout showcase
    fn render_layout_showcase(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_xl)
            .child(self.showcase_section(theme, "Stack Layouts", self.render_stacks(theme)))
            .child(self.showcase_section(theme, "Container", self.render_container(theme)))
            .child(self.showcase_section(theme, "Divider", self.render_divider(theme)))
            .child(self.showcase_section(theme, "Spacer", self.render_spacer(theme)))
    }

    /// Render utils showcase
    fn render_utils_showcase(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_xl)
            .child(
                Card::new()
                    .title("Accessibility Utilities")
                    .variant(CardVariant::Outlined)
            )
            .child(
                Label::new("Focus Trap: Manages focus within dialogs and modals")
                    .variant(LabelVariant::Body)
            )
            .child(
                Label::new("Announcer: Screen reader announcements for status updates")
                    .variant(LabelVariant::Body)
            )
    }

    /// Helper to create a showcase section
    fn showcase_section(&self, theme: &Theme, title: &str, content: impl IntoElement) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_md)
            .child(
                Label::new(title)
                    .variant(LabelVariant::Heading2)
            )
            .child(
                Card::new()
                    .variant(CardVariant::Outlined)
            )
            .child(content)
    }

    /// Render button examples
    fn render_buttons(&self, theme: &Theme) -> impl IntoElement {
        HStack::new()
            .gap(theme.global.spacing_sm)
            .child(Button::new().label("Primary").variant(ButtonVariant::Primary))
            .child(Button::new().label("Secondary").variant(ButtonVariant::Secondary))
            .child(Button::new().label("Outline").variant(ButtonVariant::Outline))
            .child(Button::new().label("Ghost").variant(ButtonVariant::Ghost))
            .child(Button::new().label("Danger").variant(ButtonVariant::Danger))
            .child(Button::new().label("Disabled").disabled(true))
    }

    /// Render input examples
    fn render_inputs(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_sm)
            .child(Input::new().placeholder("Default input"))
            .child(Input::new().placeholder("Disabled input").disabled(true))
    }

    /// Render label examples
    fn render_labels(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_xs)
            .child(Label::new("Heading 1").variant(LabelVariant::Heading1))
            .child(Label::new("Heading 2").variant(LabelVariant::Heading2))
            .child(Label::new("Heading 3").variant(LabelVariant::Heading3))
            .child(Label::new("Heading 4").variant(LabelVariant::Heading4))
            .child(Label::new("Body text").variant(LabelVariant::Body))
            .child(Label::new("Caption text").variant(LabelVariant::Caption))
    }

    /// Render badge examples
    fn render_badges(&self, theme: &Theme) -> impl IntoElement {
        HStack::new()
            .gap(theme.global.spacing_sm)
            .child(Badge::new("Default").variant(BadgeVariant::Default))
            .child(Badge::new("Success").variant(BadgeVariant::Success))
            .child(Badge::new("Warning").variant(BadgeVariant::Warning))
            .child(Badge::new("Error").variant(BadgeVariant::Error))
            .child(Badge::new("Info").variant(BadgeVariant::Info))
    }

    /// Render avatar examples
    fn render_avatars(&self, theme: &Theme) -> impl IntoElement {
        HStack::new()
            .gap(theme.global.spacing_sm)
            .child(Avatar::new("JD").size(AvatarSize::Sm))
            .child(Avatar::new("AB").size(AvatarSize::Md))
            .child(Avatar::new("CD").size(AvatarSize::Lg))
            .child(Avatar::new("EF").size(AvatarSize::Xl).status(AvatarStatus::Online))
    }

    /// Render form control examples
    fn render_form_controls(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_sm)
            .child(Checkbox::new().label("Checkbox option"))
            .child(Radio::new().label("Radio option"))
            .child(Switch::new().label("Switch option"))
    }

    /// Render icon and spinner examples
    fn render_icons_spinners(&self, theme: &Theme) -> impl IntoElement {
        HStack::new()
            .gap(theme.global.spacing_sm)
            .child(Icon::new(icons::HOME))
            .child(Icon::new(icons::SETTINGS))
            .child(Icon::new(icons::USER))
            .child(Spinner::new().size(SpinnerSize::Sm))
            .child(Spinner::new().size(SpinnerSize::Md))
            .child(Spinner::new().size(SpinnerSize::Lg))
    }

    fn render_search_bar(&self, _theme: &Theme) -> impl IntoElement {
        SearchBar::new().placeholder("Search...")
    }

    fn render_form_group(&self, _theme: &Theme) -> impl IntoElement {
        FormGroup::new()
            .label("Email")
            .required(true)
    }

    fn render_card(&self, theme: &Theme) -> impl IntoElement {
        HStack::new()
            .gap(theme.global.spacing_md)
            .child(Card::new().title("Flat Card").variant(CardVariant::Flat))
            .child(Card::new().title("Outlined Card").variant(CardVariant::Outlined))
            .child(Card::new().title("Elevated Card").variant(CardVariant::Elevated))
    }

    fn render_tab_group(&self, _theme: &Theme) -> impl IntoElement {
        TabGroup::new()
            .tabs(vec![
                Tab::new("Tab 1", "tab1"),
                Tab::new("Tab 2", "tab2"),
                Tab::new("Tab 3", "tab3"),
            ])
            .selected("tab1")
    }

    fn render_dropdown(&self, _theme: &Theme) -> impl IntoElement {
        Dropdown::new()
            .options(vec![
                DropdownOption::new("Option 1", "opt1"),
                DropdownOption::new("Option 2", "opt2"),
                DropdownOption::new("Option 3", "opt3"),
            ])
            .placeholder("Select an option")
    }

    fn render_tooltip(&self, _theme: &Theme) -> impl IntoElement {
        Tooltip::new("This is a tooltip").visible(true)
    }

    fn render_popover(&self, _theme: &Theme) -> impl IntoElement {
        Popover::new("This is popover content").title("Popover").open(true)
    }

    fn render_dialog(&self, _theme: &Theme) -> impl IntoElement {
        Label::new("Click to open dialog demo").variant(LabelVariant::Body)
    }

    fn render_drawer(&self, _theme: &Theme) -> impl IntoElement {
        Label::new("Click to open drawer demo").variant(LabelVariant::Body)
    }

    fn render_table(&self, _theme: &Theme) -> impl IntoElement {
        Label::new("Table component demo").variant(LabelVariant::Body)
    }

    fn render_command_palette(&self, _theme: &Theme) -> impl IntoElement {
        Label::new("Press Cmd+K to open command palette").variant(LabelVariant::Body)
    }

    fn render_stacks(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_md)
            .child(
                HStack::new()
                    .gap(theme.global.spacing_sm)
                    .child(Button::new().label("Item 1"))
                    .child(Button::new().label("Item 2"))
                    .child(Button::new().label("Item 3"))
            )
    }

    fn render_container(&self, _theme: &Theme) -> impl IntoElement {
        Container::new()
    }

    fn render_divider(&self, theme: &Theme) -> impl IntoElement {
        VStack::new()
            .gap(theme.global.spacing_md)
            .child(Label::new("Content above").variant(LabelVariant::Body))
            .child(Divider::new().orientation(DividerOrientation::Horizontal))
            .child(Label::new("Content below").variant(LabelVariant::Body))
    }

    fn render_spacer(&self, _theme: &Theme) -> impl IntoElement {
        HStack::new()
            .child(Label::new("Left").variant(LabelVariant::Body))
            .child(Spacer::new())
            .child(Label::new("Right").variant(LabelVariant::Body))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| ShowcaseApp::new())
        })
        .unwrap();
    });
}
