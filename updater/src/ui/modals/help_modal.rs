use crate::ui::layout::centered_rect;
use crate::ui::theme::*;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn render_help_modal(frame: &mut Frame, area: Rect) {
    let popup_area = centered_rect(50, 50, area);
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_CYAN))
        .title(" ❓ Help & Keybindings ")
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD));

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  ↑ / k          ",
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Move selection up", Style::default().fg(COLOR_TEXT_WHITE)),
        ]),
        Line::from(vec![
            Span::styled(
                "  ↓ / j          ",
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Move selection down", Style::default().fg(COLOR_TEXT_WHITE)),
        ]),
        Line::from(vec![
            Span::styled(
                "  Enter          ",
                Style::default()
                    .fg(COLOR_EMERALD)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Execute selected action or confirm dialog",
                Style::default().fg(COLOR_TEXT_WHITE),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Esc            ",
                Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Cancel modal dialog or return to menu",
                Style::default().fg(COLOR_TEXT_WHITE),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Space          ",
                Style::default()
                    .fg(COLOR_PURPLE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Toggle log auto-scroll during execution",
                Style::default().fg(COLOR_TEXT_WHITE),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  ?              ",
                Style::default()
                    .fg(COLOR_ORANGE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Toggle this help dialog",
                Style::default().fg(COLOR_TEXT_WHITE),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  q              ",
                Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
            ),
            Span::styled("Exit updater", Style::default().fg(COLOR_TEXT_WHITE)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Configuration: ", Style::default().fg(COLOR_TEXT_MUTED)),
            Span::styled(
                "Edit updater/updater.toml to customize packages and binaries.",
                Style::default().fg(COLOR_TEXT_WHITE),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press [Esc] or [Enter] to close help.",
            Style::default().fg(COLOR_CYAN),
        )),
    ];

    let para = Paragraph::new(text).block(block);
    frame.render_widget(para, popup_area);
}
