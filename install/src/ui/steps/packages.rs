use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

pub fn draw_system_packages_step(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .package_options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let is_cursor = i == app.package_cursor;
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

            let root_tag = if opt.requires_root {
                Span::styled(
                    " [Sudo Required] ",
                    Style::default()
                        .fg(THEME.amber)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    " [User] ",
                    Style::default()
                        .fg(THEME.mint)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )
            };

            let title_style = if is_cursor {
                Style::default()
                    .fg(THEME.cyan)
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
                    " [Space: Toggle] ",
                    Style::default()
                        .fg(THEME.amber)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            };

            ListItem::new(vec![
                Line::from(vec![check, Span::styled(&opt.title, title_style), Span::raw(" "), root_tag, placeholder_hint]),
                Line::from(Span::styled(
                    format!("    {}", opt.description),
                    Style::default().fg(if is_cursor { THEME.text_body } else { THEME.text_dim }),
                )),
                Line::from(Span::styled(
                    format!("    {}", opt.detail),
                    Style::default().fg(THEME.text_muted),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 3. System Packages & Dependencies [Space: Toggle | a: Select All | Enter: Next] ")
            .title_style(THEME.title_amber())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.amber)),
    );
    f.render_widget(list, area);
}
