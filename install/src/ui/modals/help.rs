use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::layout::centered_rect;
use crate::ui::THEME;

pub fn draw_help_modal(f: &mut Frame, area: Rect) {
    let popup_area = centered_rect(72, 68, area);
    f.render_widget(Clear, popup_area);

    let shortcuts = vec![
        ("1 - 9 / 0", "Jump directly to specific wizard step (0 = Summary)"),
        ("Tab / n", "Navigate to Next configuration step"),
        ("BackTab / p", "Navigate to Previous configuration step"),
        ("↑ / ↓ / j / k", "Navigate items in the current active step"),
        ("Space", "Toggle selection of the currently focused item"),
        ("a / A", "Select or Deselect all items in active step"),
        ("Enter / i", "Start installation or confirm current step"),
        ("s", "Change binary source folder path (e.g. target/release)"),
        ("r", "Rescan workspace and pre-built binary source directory"),
        ("c", "Clear installation log buffer"),
        ("g / G", "Jump to top / bottom of installation logs"),
        ("? / Esc", "Toggle or dismiss this help modal"),
        ("q / Ctrl+C", "Exit installer safely"),
    ];

    let mut lines = vec![
        Line::from(Span::styled(
            "BabyDra TUI Installer ── Keyboard Navigation",
            THEME.title_cyan(),
        )),
        Line::from(""),
    ];

    for (key, desc) in shortcuts {
        lines.push(Line::from(vec![
            Span::styled(
                format!(" {:<14} ", key),
                THEME.key_badge(),
            ),
            Span::raw(" "),
            Span::styled(desc, Style::default().fg(THEME.text_body)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(" Esc / ? / Enter ", THEME.key_badge_green()),
        Span::styled(" Close this help dialog", Style::default().fg(THEME.text_dim)),
    ]));

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Help & Keybindings ")
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, popup_area);
}
