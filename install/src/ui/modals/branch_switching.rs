use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, BranchSwitchStatus};
use crate::ui::layout::centered_rect;
use crate::ui::THEME;

const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn draw_branch_switching_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(66, 30, area);
    f.render_widget(Clear, popup_area);

    let branch = if app.selected_branch.is_empty() {
        "release"
    } else {
        &app.selected_branch
    };

    let spinner = SPINNER_FRAMES[app.branch_switch_spinner_tick % SPINNER_FRAMES.len()];

    let (border_color, lines) = match &app.branch_switch_status {
        BranchSwitchStatus::Switching | BranchSwitchStatus::Idle => (
            THEME.cyan,
            vec![
                Line::from(vec![
                    Span::styled(
                        format!(" {spinner} "),
                        Style::default()
                            .fg(THEME.amber)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("Switching to branch '{}'...", branch),
                        Style::default()
                            .fg(THEME.text_bright)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "◆ Running: git fetch --prune origin",
                    Style::default().fg(THEME.text_dim),
                )),
                Line::from(Span::styled(
                    format!("◆ Running: git checkout {branch} && git pull origin {branch}"),
                    Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    "◆ Rescanning variants, theme assets, and crate binaries...",
                    Style::default().fg(THEME.text_dim),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Please wait while repository components are synchronized.",
                    Style::default().fg(THEME.amber),
                )),
            ],
        ),
        BranchSwitchStatus::Done(Ok(())) => (
            THEME.mint,
            vec![
                Line::from(vec![
                    Span::styled(
                        " ✔ ",
                        Style::default()
                            .fg(THEME.mint)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("Successfully switched to branch '{}'!", branch),
                        Style::default()
                            .fg(THEME.mint)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "Workspace components and variants have been updated.",
                    Style::default().fg(THEME.text_bright),
                )),
                Line::from(Span::styled(
                    "Proceeding to the next step...",
                    Style::default().fg(THEME.text_muted),
                )),
            ],
        ),
        BranchSwitchStatus::Done(Err(err)) => (
            THEME.rose,
            vec![
                Line::from(vec![
                    Span::styled(
                        " ✖ ",
                        Style::default()
                            .fg(THEME.rose)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("Failed to switch to branch '{}'", branch),
                        Style::default()
                            .fg(THEME.rose)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    format!("Error: {err}"),
                    Style::default().fg(THEME.amber),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled(" [Esc / Enter / q] ", THEME.key_badge_red()),
                    Span::styled(" Dismiss and return to branch selection", Style::default().fg(THEME.text_body)),
                ]),
            ],
        ),
    };

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Switching Source Branch ")
                .title_style(
                    Style::default()
                        .fg(border_color)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(border_color)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
