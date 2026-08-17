use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_variant_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(7)])
        .split(area);

    let items: Vec<ListItem> = app
        .variant_options
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let is_cursor = i == app.variant_cursor;
            let radio = if v.selected {
                Span::styled(
                    "(●) ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("( ) ", Style::default().fg(Color::DarkGray))
            };

            let title_style = if is_cursor {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            };

            let row_style = if is_cursor {
                Style::default().bg(Color::Rgb(25, 30, 48))
            } else {
                Style::default()
            };

            ListItem::new(vec![
                Line::from(vec![
                    radio,
                    Span::styled(&v.name, title_style),
                    Span::raw("  "),
                    Span::styled(
                        format!("(theme: {})", v.theme),
                        Style::default().fg(Color::Cyan),
                    ),
                ]),
                Line::from(Span::styled(
                    format!("    Apps: {}", v.apps_preview()),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 7. Variant Selection (Theme + Apps + Keybinds) [↑/↓: Move | Space: Select | Enter: Next] ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(list, chunks[0]);

    let selected = app
        .variant_options
        .iter()
        .find(|v| v.selected)
        .map(|v| v.name.as_str())
        .unwrap_or("default");

    let prompt_box = Paragraph::new(vec![
        Line::from(Span::styled(
            format!("Currently selected variant: {selected}"),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("A variant bundles a theme package, an app list and keybinds in one folder."),
        Line::from(Span::styled(
            "Press [Space] to select, [Enter/n] to continue.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(
        Block::default()
            .title(" Variant Info ")
            .title_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green)),
    );
    f.render_widget(prompt_box, chunks[1]);
}
