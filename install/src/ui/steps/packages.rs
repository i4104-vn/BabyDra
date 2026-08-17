use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

pub fn draw_system_packages_step(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .package_options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let is_cursor = i == app.package_cursor;
            let check = if opt.selected {
                Span::styled(
                    "[✓] ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
            };

            let root_tag = if opt.requires_root {
                Span::styled(" [Sudo] ", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            };

            let title_style = if is_cursor {
                Style::default()
                    .fg(Color::Cyan)
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
                Line::from(vec![check, Span::styled(&opt.title, title_style), root_tag]),
                Line::from(Span::styled(
                    format!("    {}", opt.description),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!("    Command: {}", opt.detail),
                    Style::default().fg(Color::DarkGray),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 3. System Packages & Dependencies [Space: Toggle | a: Select All | Enter/n: Next] ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(list, area);
}
