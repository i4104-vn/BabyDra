use crate::core::app::App;
use crate::ui::layout::centered_rect;
use crate::ui::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn render_reset_modal(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(65, 55, area);
    frame.render_widget(Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(popup_area);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_RED))
        .title(" FACTORY RESET SELECTION ")
        .title_style(Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD));

    let title_para = Paragraph::new(vec![Line::from(vec![
        Span::styled("Choose reset mode. ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "WARNING: This restores Arch Linux to pure TTY login!",
            Style::default().fg(COLOR_ORANGE),
        ),
    ])])
    .block(title_block);
    frame.render_widget(title_para, chunks[0]);

    let options = [
        (
            "Keep Packages",
            "Revert greetd to TTY1, clean configs & binaries, keep pacman/AUR packages",
        ),
        (
            "Remove Shell Packages",
            "Clean configs and uninstall BabyDra shell packages (labwc, greetd, etc.)",
        ),
        (
            "Dry Run (Simulation)",
            "Test run: print actions to terminal without modifying files",
        ),
        (
            "Remove ALL User Apps",
            "Restore pure vanilla Arch baseline: uninstall all user packages",
        ),
        ("Cancel", "Return to main menu without making changes"),
    ];

    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let is_sel = i == app.reset_mode_selected;
            let (prefix, name_style, desc_style) = if is_sel {
                (
                    " ▶ ",
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                    Style::default().fg(COLOR_TEXT_WHITE),
                )
            } else {
                (
                    "   ",
                    Style::default().fg(COLOR_TEXT_WHITE),
                    Style::default().fg(COLOR_TEXT_MUTED),
                )
            };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        prefix,
                        Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(format!("{}. {}", i + 1, name), name_style),
                ]),
                Line::from(vec![Span::raw("      "), Span::styled(*desc, desc_style)]),
                Line::from(""),
            ])
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER));

    let list = List::new(items).block(list_block);
    frame.render_widget(list, chunks[1]);

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER));

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " [↑/↓] ",
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Select   ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "[Enter] ",
            Style::default()
                .fg(COLOR_EMERALD)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Confirm   ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "[Esc] ",
            Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Cancel", Style::default().fg(COLOR_TEXT_WHITE)),
    ]))
    .block(footer_block);
    frame.render_widget(footer, chunks[2]);
}
