use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::layout::centered_rect;

pub fn draw_edit_path_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(65, 22, area);
    f.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from(Span::styled("Enter path to directory containing pre-built binaries:", Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(vec![
            Span::styled("Path: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(&app.custom_path_input, Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)),
            Span::styled("█", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from(Span::styled("[Enter] Apply & Rescan  |  [Esc] Cancel", Style::default().fg(Color::DarkGray))),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Change Binary Source Directory ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
