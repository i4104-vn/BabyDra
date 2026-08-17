use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::{InstallState, LogLevel};
use crate::ui::THEME;

pub fn draw_execute_install_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let gauge_color = match app.install_state {
        InstallState::Idle => THEME.blue,
        InstallState::Installing => THEME.cyan,
        InstallState::Completed { success, .. } => {
            if success {
                THEME.mint
            } else {
                THEME.amber
            }
        }
    };

    let gauge_title = format!(
        " 󰐥 Progress: {}% ── {} ",
        app.progress_percent, app.current_step_desc
    );
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(gauge_title)
                .title_style(
                    Style::default()
                        .fg(gauge_color)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(gauge_color)),
        )
        .gauge_style(
            Style::default()
                .fg(gauge_color)
                .bg(THEME.bg_selected),
        )
        .percent(app.progress_percent);
    f.render_widget(gauge, chunks[0]);

    // Available inner height for log rows (minus top and bottom border)
    let inner_height = chunks[1].height.saturating_sub(2) as usize;
    let total_logs = app.logs.len();

    let display_scroll = if app.auto_scroll_logs && total_logs > inner_height {
        total_logs.saturating_sub(inner_height)
    } else {
        app.log_scroll
    };

    let log_lines: Vec<Line> = app
        .logs
        .iter()
        .skip(display_scroll)
        .map(|msg| {
            let (badge, style) = match msg.level {
                LogLevel::Info => (
                    " [INFO  ] ",
                    Style::default().fg(THEME.cyan).bg(THEME.bg_card),
                ),
                LogLevel::Success => (
                    " [  OK  ] ",
                    Style::default()
                        .fg(THEME.mint)
                        .bg(THEME.bg_card)
                        .add_modifier(Modifier::BOLD),
                ),
                LogLevel::Warn => (
                    " [ WARN ] ",
                    Style::default()
                        .fg(THEME.amber)
                        .bg(THEME.bg_card)
                        .add_modifier(Modifier::BOLD),
                ),
                LogLevel::Error => (
                    " [ FAIL ] ",
                    Style::default()
                        .fg(THEME.rose)
                        .bg(THEME.bg_card)
                        .add_modifier(Modifier::BOLD),
                ),
                LogLevel::Copy => (
                    " [ COPY ] ",
                    Style::default()
                        .fg(THEME.blue)
                        .bg(THEME.bg_card)
                        .add_modifier(Modifier::BOLD),
                ),
                LogLevel::Bundle => (
                    " [BUNDLE] ",
                    Style::default()
                        .fg(THEME.purple)
                        .bg(THEME.bg_card)
                        .add_modifier(Modifier::BOLD),
                ),
                LogLevel::Config => (
                    " [CONFIG] ",
                    Style::default()
                        .fg(THEME.pink)
                        .bg(THEME.bg_card)
                        .add_modifier(Modifier::BOLD),
                ),
            };

            Line::from(vec![
                Span::styled(
                    format!("{} ", msg.timestamp),
                    Style::default().fg(THEME.text_muted),
                ),
                Span::styled(badge, style),
                Span::raw(" "),
                Span::styled(&msg.message, Style::default().fg(THEME.text_bright)),
            ])
        })
        .collect();

    let scroll_status = if app.auto_scroll_logs {
        "Auto-scroll: ON"
    } else {
        "Auto-scroll: OFF"
    };

    let logs_block = Block::default()
        .title(format!(
            " 9. Live Installation Logs ({total_logs} entries | {scroll_status}) [j/k: Scroll | c: Clear | g/G: Top/Bottom] "
        ))
        .title_style(THEME.title_cyan())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(THEME.cyan));

    let logs_widget = Paragraph::new(log_lines)
        .block(logs_block)
        .wrap(Wrap { trim: false });
    f.render_widget(logs_widget, chunks[1]);
}
