//! Table component for data display.

use gpui::*;
use crate::{atoms::Label, theme::Theme};

/// Table column definition
#[derive(Clone)]
pub struct TableColumn {
    /// Column header text
    pub header: SharedString,
    /// Column width
    pub width: Option<Pixels>,
}

/// Table configuration properties
#[derive(Clone)]
pub struct TableProps {
    /// Table columns
    pub columns: Vec<TableColumn>,
}

impl Default for TableProps {
    fn default() -> Self {
        Self {
            columns: vec![],
        }
    }
}

/// A table component for displaying data.
///
/// Table provides a structured layout for tabular data with headers.
///
/// ## Example
///
/// ```rust,ignore
/// use purdah_gpui_components::organisms::*;
///
/// Table::new()
///     .columns(vec![
///         TableColumn { header: "Name".into(), width: Some(px(200.0)) },
///         TableColumn { header: "Email".into(), width: None },
///     ]);
/// ```
pub struct Table {
    props: TableProps,
}

impl Table {
    pub fn new() -> Self {
        Self {
            props: TableProps::default(),
        }
    }

    pub fn columns(mut self, columns: Vec<TableColumn>) -> Self {
        self.props.columns = columns;
        self
    }
}

impl Render for Table {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            .w_full()
            .border_color(theme.alias.color_border)
            .border(px(1.0))
            .rounded(theme.global.radius_md)
            .overflow_hidden()
            .child(
                // Header row
                div()
                    .flex()
                    .flex_row()
                    .bg(if theme.is_dark() {
                        theme.global.gray_800
                    } else {
                        theme.global.gray_50
                    })
                    .border_color(theme.alias.color_border)
                    .border_b(px(1.0))
                    .children(
                        self.props.columns.iter().map(|col| {
                            let mut cell = div()
                                .p(theme.global.spacing_sm)
                                .flex_1();

                            if let Some(width) = col.width {
                                cell = cell.w(width).flex_none();
                            }

                            cell.child(
                                Label::new(col.header.clone())
                                    .color(theme.alias.color_text_primary)
                            )
                        }).collect::<Vec<_>>()
                    )
            )
            .child(
                // Placeholder for data rows
                div()
                    .p(theme.global.spacing_lg)
                    .text_color(theme.alias.color_text_muted)
                    .child("Table rows would go here")
            )
    }
}

impl IntoElement for Table {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let theme = Theme::default();

        div()
            .w_full()
            .border_color(theme.alias.color_border)
            .border(px(1.0))
            .rounded(theme.global.radius_md)
            .overflow_hidden()
            .child(
                // Header row
                div()
                    .flex()
                    .flex_row()
                    .bg(if theme.is_dark() {
                        theme.global.gray_800
                    } else {
                        theme.global.gray_50
                    })
                    .border_color(theme.alias.color_border)
                    .border_b(px(1.0))
                    .children(
                        self.props.columns.iter().map(|col| {
                            let mut cell = div()
                                .p(theme.global.spacing_sm)
                                .flex_1();

                            if let Some(width) = col.width {
                                cell = cell.w(width).flex_none();
                            }

                            cell.child(
                                Label::new(col.header.clone())
                                    .color(theme.alias.color_text_primary)
                            )
                        }).collect::<Vec<_>>()
                    )
            )
            .child(
                // Placeholder for data rows
                div()
                    .p(theme.global.spacing_lg)
                    .text_color(theme.alias.color_text_muted)
                    .child("Table rows would go here")
            )
    }
}
