use crate::core::app::App;
use crate::ui::theme::*;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let help_line = Line::from(vec![
        Span::styled(
            " [↑/↓/j/k] ",
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Navigate   ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "[Enter] ",
            Style::default()
                .fg(COLOR_EMERALD)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Execute   ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "[?] ",
            Style::default()
                .fg(COLOR_PURPLE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Help   ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "[q] ",
            Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Quit", Style::default().fg(COLOR_TEXT_WHITE)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER));

    let paragraph = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(COLOR_TEXT_MUTED)),
            Span::styled(&app.status_message, Style::default().fg(COLOR_TEXT_WHITE)),
        ]),
        help_line,
    ])
    .block(block);

    frame.render_widget(paragraph, area);
}
