use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

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
                Span::styled(
                    "[✔] ",
                    Style::default()
                        .fg(THEME.mint)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("[ ] ", Style::default().fg(THEME.text_muted))
            };

            let title_style = if is_cursor {
                Style::default()
                    .fg(THEME.purple)
                    .add_modifier(Modifier::BOLD)
            } else if opt.selected {
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(THEME.text_dim)
            };

            let row_style = if is_cursor {
                Style::default().bg(THEME.bg_cursor)
            } else if opt.selected {
                Style::default().bg(THEME.bg_card)
            } else {
                Style::default()
            };

            let placeholder_hint = if is_cursor {
                Span::styled(
                    " [Space: Bật/Tắt] ",
                    Style::default()
                        .fg(THEME.amber)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            };

            ListItem::new(vec![
                Line::from(vec![check, Span::styled(&opt.title, title_style), Span::raw(" "), placeholder_hint]),
                Line::from(Span::styled(
                    format!("    {}", opt.description),
                    Style::default().fg(if is_cursor { THEME.text_body } else { THEME.text_dim }),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 5. System Staging & /var/lib/babydra Bundle [Space: Toggle | Enter: Next] ")
            .title_style(THEME.title_purple())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.purple)),
    );
    f.render_widget(list, chunks[0]);

    let info_box = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("◆ Why /var/lib/babydra? ", THEME.title_amber()),
            Span::styled("Central system staging directory (mode 0777)", Style::default().fg(THEME.text_dim)),
        ]),
        Line::from("Staging binaries and assets in /var/lib/babydra allows display managers (like greetd/cage)"),
        Line::from("and system daemons to access theme icons, wallpapers, and components without accessing private home folders."),
    ])
    .block(
        Block::default()
            .title(" Bundle Staging Purpose ")
            .title_style(THEME.title_amber())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(info_box, chunks[1]);
}
