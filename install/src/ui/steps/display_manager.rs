use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_display_manager_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(7)])
        .split(area);

    let items: Vec<ListItem> = app
        .display_manager_options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let is_cursor = i == app.display_manager_cursor;
            let check = if opt.selected {
                Span::styled("[x] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("[ ] ", Style::default().fg(Color::DarkGray))
            };

            let title_style = if is_cursor {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            } else {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            };

            let row_style = if is_cursor { Style::default().bg(Color::Rgb(25, 30, 48)) } else { Style::default() };

            ListItem::new(vec![
                Line::from(vec![check, Span::styled(&opt.title, title_style)]),
                Line::from(Span::styled(format!("    {}", opt.description), Style::default().fg(Color::DarkGray))),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 6. Display Manager & Greetd Login Setup [Space: Toggle | Enter: Confirm Install] ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Yellow)),
    );
    f.render_widget(list, chunks[0]);

    let prompt_box = Paragraph::new(vec![
        Line::from(Span::styled("Ready to Execute Installation?", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))),
        Line::from("All configuration steps have been specified. Press [Enter] or [i] to begin direct binary deployment."),
        Line::from(Span::styled("Or use [p] to review previous configuration steps.", Style::default().fg(Color::DarkGray))),
    ])
    .block(
        Block::default()
            .title(" Ready ")
            .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green)),
    );
    f.render_widget(prompt_box, chunks[1]);
}
