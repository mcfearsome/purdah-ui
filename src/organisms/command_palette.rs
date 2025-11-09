//! CommandPalette component for command interface.

use gpui::*;
use gpui::prelude::FluentBuilder;
use crate::{atoms::{Input, Label, LabelVariant}, theme::Theme};

/// Command item definition
#[derive(Clone)]
pub struct Command {
    /// Command label
    pub label: SharedString,
    /// Command description
    pub description: Option<SharedString>,
}

/// CommandPalette configuration properties
#[derive(Clone)]
pub struct CommandPaletteProps {
    /// Search query
    pub query: SharedString,
    /// Available commands
    pub commands: Vec<Command>,
    /// Whether palette is open
    pub open: bool,
}

impl Default for CommandPaletteProps {
    fn default() -> Self {
        Self {
            query: "".into(),
            commands: vec![],
            open: false,
        }
    }
}

/// A command palette component.
///
/// CommandPalette provides a searchable command interface.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::organisms::*;
///
/// CommandPalette::new()
///     .commands(vec![
///         Command {
///             label: "Open File".into(),
///             description: Some("Ctrl+O".into()),
///         },
///     ])
///     .open(true);
/// ```
pub struct CommandPalette {
    props: CommandPaletteProps,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            props: CommandPaletteProps::default(),
        }
    }

    pub fn query(mut self, query: impl Into<SharedString>) -> Self {
        self.props.query = query.into();
        self
    }

    pub fn commands(mut self, commands: Vec<Command>) -> Self {
        self.props.commands = commands;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.props.open = open;
        self
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        if !self.props.open {
            return div(); // Return empty div if not open
        }

        div()
            .fixed()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h_full()
            .flex()
            .items_start()
            .justify_center()
            .pt(px(100.0))
            .bg(hsla(0.0, 0.0, 0.0, 0.5))
            .child(
                // Command palette panel
                div()
                    .w(px(600.0))
                    .bg(theme.alias.color_surface)
                    .rounded(theme.global.radius_lg)
                    .shadow_xl()
                    .overflow_hidden()
                    .child(
                        // Search input
                        div()
                            .p(theme.global.spacing_sm)
                            .border_color(theme.alias.color_border)
                            .border_b(px(1.0))
                            .child(
                                Input::new()
                                    .value(self.props.query.clone())
                                    .placeholder("Search commands...")
                            )
                    )
                    .child(
                        // Commands list
                        div()
                            .max_h(px(400.0))
                            .overflow_y_scroll()
                            .children(
                                self.props.commands.iter().map(|cmd| {
                                    div()
                                        .p(theme.global.spacing_sm)
                                        .flex()
                                        .flex_col()
                                        .gap(px(2.0))
                                        .hover(|style| {
                                            style.bg(theme.alias.color_surface_hover)
                                        })
                                        .child(
                                            Label::new(cmd.label.clone())
                                                .variant(LabelVariant::Body)
                                        )
                                        .when_some(cmd.description.clone(), |div, desc| {
                                            div.child(
                                                Label::new(desc)
                                                    .variant(LabelVariant::Caption)
                                                    .color(theme.alias.color_text_muted)
                                            )
                                        })
                                }).collect::<Vec<_>>()
                            )
                    )
            )
    }
}
