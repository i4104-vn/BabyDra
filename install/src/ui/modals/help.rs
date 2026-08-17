use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::layout::centered_rect;

pub fn draw_help_modal(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(70, 65, area);
    f.render_widget(Clear, popup_area);

    let shortcuts = vec![
        ("1 - 9 / 0", "Jump directly to wizard step (0 = Summary)"),
        ("Tab / n", "Navigate to Next step"),
        ("BackTab / p", "Navigate to Previous step"),
        (
            "Up / Down / j / k",
            "Navigate items in the current active step",
        ),
        ("Space", "Toggle selection of the focused item"),
        ("a / A", "Select / Deselect all items in active step"),
        ("Enter / i", "Start installation or confirm"),
        (
            "s",
            "Change binary source folder path (e.g. target/release)",
        ),
        (
            "Sudo modal",
            "Password is masked; Enter validates once before any change",
        ),
        ("r", "Rescan source directory for binary files"),
        ("c", "Clear installation log buffer"),
        ("g / G", "Jump to top / bottom of logs"),
        ("? / Esc", "Toggle this help dialog"),
        ("q / Ctrl+C", "Quit installer"),
    ];

    let mut lines = vec![
        Line::from(Span::styled(
            "BabyDra Step-by-Step TUI Installer - Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (key, desc) in shortcuts {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<20}", key),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(desc, Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press [Esc] or [?] or [Enter] to close",
        Style::default().fg(Color::DarkGray),
    )));

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Help & Keybindings ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, popup_area);
}
