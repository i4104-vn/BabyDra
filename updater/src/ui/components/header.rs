use crate::core::app::App;
use crate::ui::theme::*;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let cfg_status = if app.config_path.is_some() {
        Span::styled(
            " [updater.toml: loaded] ",
            Style::default().fg(COLOR_EMERALD),
        )
    } else {
        Span::styled(
            " [updater.toml: defaults] ",
            Style::default().fg(COLOR_ORANGE),
        )
    };

    let title_line = Line::from(vec![
        Span::styled("🐉 ", Style::default().fg(COLOR_ORANGE)),
        Span::styled(
            "BabyDra ",
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "Shell Manager & TUI Updater",
            Style::default()
                .fg(COLOR_TEXT_WHITE)
                .add_modifier(Modifier::BOLD),
        ),
        cfg_status,
    ]);

    let repo_display = app.repo_root.to_string_lossy();
    let subtitle_line = Line::from(vec![
        Span::styled("Repository: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(repo_display, Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "  │  Arch Linux Wayland Shell",
            Style::default().fg(COLOR_TEXT_MUTED),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER));

    let paragraph = Paragraph::new(vec![title_line, subtitle_line])
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
