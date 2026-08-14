use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_varlib_bundle_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(6)])
        .split(area);

    let items: Vec<ListItem> = app
        .varlib_options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let is_cursor = i == app.varlib_cursor;
            let check = if opt.selected {
                Span::styled("[x] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
            };

            let title_style = if is_cursor {
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            };

            let row_style = if is_cursor { Style::default().bg(Color::Rgb(25, 30, 48)) } else { Style::default() };

            ListItem::new(vec![
                Line::from(vec![check, Span::styled(&opt.title, title_style)]),
                Line::from(Span::styled(format!("    {}", opt.description), Style::default().fg(Color::Gray))),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 4. System Staging & /var/lib/babydra Bundle [Space: Toggle | Enter/n: Next] ")
            .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    f.render_widget(list, chunks[0]);

    let info_box = Paragraph::new(vec![
        Line::from(Span::styled("Why /var/lib/babydra?", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("Staging all binaries and wallpapers in /var/lib/babydra allows system services (like greetd login manager)"),
        Line::from("to access icons, wallpapers, and components without needing access to private user home directories."),
    ])
    .block(
        Block::default()
            .title(" Bundle Staging Purpose ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(info_box, chunks[1]);
}
