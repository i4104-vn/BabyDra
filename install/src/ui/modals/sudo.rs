use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::system::sudo::MAX_PASSWORD_ATTEMPTS;
use crate::ui::layout::centered_rect;
use crate::ui::THEME;

pub fn draw_sudo_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(62, 30, area);
    f.render_widget(Clear, popup_area);

    let masked: String = "● ".repeat(app.sudo_password.chars().count());

    let mut lines = vec![
        Line::from(Span::styled(
            "🔒 Sudo Authentication Required",
            THEME.title_amber(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Root privileges are required for system packages, /usr/bin, /var/lib, and greetd.",
            Style::default().fg(THEME.text_body),
        )),
        Line::from(Span::styled(
            "Your password is held in memory and validated once via piped stdin.",
            Style::default().fg(THEME.text_dim),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("◆ Sudo Password: ", Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD)),
            Span::styled(
                if masked.is_empty() { "Enter password...".into() } else { masked },
                Style::default().fg(if app.sudo_password.is_empty() { THEME.text_muted } else { THEME.cyan }),
            ),
            Span::styled(" █", Style::default().fg(THEME.mint)),
        ]),
        Line::from(""),
    ];

    if let Some(err) = &app.sudo_error {
        lines.push(Line::from(Span::styled(
            format!("✖ Error: {err}"),
            Style::default().fg(THEME.rose).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    let attempts_left = (MAX_PASSWORD_ATTEMPTS as i32 - app.sudo_attempts as i32).max(0);
    lines.push(Line::from(Span::styled(
        format!("Remaining attempts: {attempts_left} of {MAX_PASSWORD_ATTEMPTS}"),
        Style::default().fg(if attempts_left <= 1 {
            THEME.rose
        } else {
            THEME.text_dim
        }),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" Enter ", THEME.key_badge_green()),
        Span::styled(" Authenticate & Begin   ", Style::default().fg(THEME.text_body)),
        Span::styled(" Esc ", THEME.key_badge_red()),
        Span::styled(" Cancel", Style::default().fg(THEME.rose)),
    ]));

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Sudo Authentication ")
                .title_style(THEME.title_amber())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.amber)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
