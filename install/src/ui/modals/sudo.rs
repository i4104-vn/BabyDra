use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::system::sudo::MAX_PASSWORD_ATTEMPTS;
use crate::ui::layout::centered_rect;

/// Password prompt shown right before installation (and again after a failed
/// pre-auth). The password is masked (`*`) and only ever lives in memory —
/// it is fed to `sudo -S` through a piped stdin, never to the TTY.
pub fn draw_sudo_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 30, area);
    f.render_widget(Clear, popup_area);

    let masked: String = "*".repeat(app.sudo_password.chars().count());

    let mut lines = vec![
        Line::from(Span::styled(
            "Sudo password required",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Some installation steps need root access (pacman, /usr/bin, /var/lib, greetd).",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "The password is verified once before anything is changed.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Password: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                masked,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled("█", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
    ];

    if let Some(err) = &app.sudo_error {
        lines.push(Line::from(Span::styled(
            format!("Error: {err}"),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));
    }

    let attempts_left = (MAX_PASSWORD_ATTEMPTS as i32 - app.sudo_attempts as i32).max(0);
    lines.push(Line::from(Span::styled(
        format!("Attempts left: {attempts_left} (to avoid locking your account)"),
        Style::default().fg(if attempts_left <= 1 {
            Color::Red
        } else {
            Color::DarkGray
        }),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter] Validate & start install  |  [Esc] Cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Sudo Authentication ")
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
