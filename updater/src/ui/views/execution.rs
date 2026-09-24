use crate::core::app::App;
use crate::core::state::ViewState;
use crate::ui::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_execution(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(8)])
        .split(area);

    let is_running = app.view_state == ViewState::Running;
    let action_name = app.active_action_name.as_deref().unwrap_or("Action");

    // Top status banner
    let status_line = if is_running {
        Line::from(vec![
            Span::styled(
                format!(" {} ", app.spinner_char()),
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "EXECUTING: ",
                Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                action_name,
                Style::default()
                    .fg(COLOR_TEXT_WHITE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "  (Live output streaming below...)",
                Style::default().fg(COLOR_TEXT_MUTED),
            ),
        ])
    } else {
        match app.last_exit_code {
            Some(0) => Line::from(vec![
                Span::styled(
                    " ",
                    Style::default()
                        .fg(COLOR_EMERALD)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "SUCCESS: ",
                    Style::default()
                        .fg(COLOR_EMERALD)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    action_name,
                    Style::default()
                        .fg(COLOR_TEXT_WHITE)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " completed successfully! Press [Enter] or [Esc] to return.",
                    Style::default().fg(COLOR_EMERALD),
                ),
            ]),
            Some(code) => Line::from(vec![
                Span::styled(
                    " ✘ ",
                    Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("FAILED (code {}): ", code),
                    Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    action_name,
                    Style::default()
                        .fg(COLOR_TEXT_WHITE)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " — Press [Enter] or [Esc] to return.",
                    Style::default().fg(COLOR_RED),
                ),
            ]),
            None => Line::from(vec![
                Span::styled(
                    " Finished: ",
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                ),
                Span::styled(action_name, Style::default().fg(COLOR_TEXT_WHITE)),
            ]),
        }
    };

    let banner_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if is_running {
            Style::default().fg(COLOR_CYAN)
        } else if app.last_exit_code == Some(0) {
            Style::default().fg(COLOR_EMERALD)
        } else {
            Style::default().fg(COLOR_RED)
        });

    let banner = Paragraph::new(vec![status_line]).block(banner_block);
    frame.render_widget(banner, chunks[0]);

    // Log window
    let visible_height = chunks[1].height.saturating_sub(2) as usize;
    let total_logs = app.logs.len();

    let scroll_pos = if app.autoscroll {
        total_logs.saturating_sub(visible_height)
    } else {
        app.log_scroll
            .min(total_logs.saturating_sub(visible_height))
    };

    let log_lines: Vec<Line> = app
        .logs
        .iter()
        .skip(scroll_pos)
        .take(visible_height)
        .map(|line| {
            if line.starts_with("==>") {
                Line::from(Span::styled(
                    line,
                    Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
                ))
            } else if line.starts_with("✔") {
                Line::from(Span::styled(
                    line,
                    Style::default()
                        .fg(COLOR_EMERALD)
                        .add_modifier(Modifier::BOLD),
                ))
            } else if line.starts_with("✘") {
                Line::from(Span::styled(
                    line,
                    Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
                ))
            } else if line.starts_with("$") {
                Line::from(Span::styled(line, Style::default().fg(COLOR_ORANGE)))
            } else {
                Line::from(Span::styled(line, Style::default().fg(COLOR_TEXT_WHITE)))
            }
        })
        .collect();

    let autoscroll_tag = if app.autoscroll {
        "[Auto-scroll: ON]"
    } else {
        "[Auto-scroll: OFF]"
    };
    let log_title = format!(
        " 📜 Terminal Output ({} lines) {} ",
        total_logs, autoscroll_tag
    );

    let log_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER))
        .title(log_title)
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD));

    let log_paragraph = Paragraph::new(log_lines).block(log_block);
    frame.render_widget(log_paragraph, chunks[1]);
}
