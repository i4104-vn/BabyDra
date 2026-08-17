use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

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
            .title(" 8. Display Manager & Greetd Login Setup [Space: Toggle | Enter: Review & Install] ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(list, chunks[0]);

    let prompt_box = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("🚀 Ready to Execute BabyDra Installation? ", THEME.title_mint()),
        ]),
        Line::from("All configuration steps have been reviewed. Pressing [Enter] or [i] will prompt for confirmation and start installation."),
        Line::from(Span::styled(
            "Controls: [Enter / i] Proceed to Install | [p] Previous Step | [1-8] Jump to Step",
            Style::default().fg(THEME.text_muted),
        )),
    ])
    .block(
        Block::default()
            .title(" Installation Launch Confirmation ")
            .title_style(THEME.title_mint())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.mint)),
    );
    f.render_widget(prompt_box, chunks[1]);
}
