use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::layout::centered_rect;
use crate::ui::THEME;

pub fn draw_edit_path_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(66, 22, area);
    f.render_widget(Clear, popup_area);

    let lines = vec![
        Line::from(Span::styled(
            "Enter path to directory containing pre-built binaries:",
            Style::default().fg(THEME.text_body),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "◆ Path: ",
                Style::default()
                    .fg(THEME.amber)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                &app.custom_path_input,
                Style::default()
                    .fg(THEME.cyan)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(" █", Style::default().fg(THEME.mint)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" Enter ", THEME.key_badge_green()),
            Span::styled(" Apply & Rescan   ", Style::default().fg(THEME.text_body)),
            Span::styled(" Esc ", THEME.key_badge_red()),
            Span::styled(" Cancel", Style::default().fg(THEME.rose)),
        ]),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Change Binary Source Directory ")
                .title_style(THEME.title_amber())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.amber)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
