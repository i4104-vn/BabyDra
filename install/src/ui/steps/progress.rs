use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::{InstallState, LogLevel};

pub fn draw_execute_install_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(8)])
        .split(area);

    let gauge_color = match app.install_state {
        InstallState::Idle => Color::Blue,
        InstallState::Installing => Color::Cyan,
        InstallState::Completed { success, .. } => {
            if success { Color::Green } else { Color::Yellow }
        }
    };

    let gauge_title = format!(" Progress: {}% - {}", app.progress_percent, app.current_step_desc);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(gauge_title)
                .title_style(Style::default().fg(gauge_color).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(gauge_color)),
        )
        .gauge_style(Style::default().fg(gauge_color).bg(Color::DarkGray))
        .percent(app.progress_percent);
    f.render_widget(gauge, chunks[0]);

    let log_lines: Vec<Line> = app
        .logs
        .iter()
        .skip(app.log_scroll)
        .map(|msg| {
            let (badge, style) = match msg.level {
                LogLevel::Info => ("[INFO  ] ", Style::default().fg(Color::Cyan)),
                LogLevel::Success => ("[  OK  ] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                LogLevel::Warn => ("[ WARN ] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                LogLevel::Error => ("[ FAIL ] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                LogLevel::Copy => ("[ COPY ] ", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                LogLevel::Bundle => ("[BUNDLE] ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
                LogLevel::Config => ("[ CONFIG] ", Style::default().fg(Color::Rgb(255, 165, 0)).add_modifier(Modifier::BOLD)),
            };

            Line::from(vec![
                Span::styled(format!("{} ", msg.timestamp), Style::default().fg(Color::DarkGray)),
                Span::styled(badge, style),
                Span::styled(&msg.message, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let logs_block = Block::default()
        .title(format!(" 7. Live Installation Logs ({}) [j/k: Scroll | c: Clear | g/G: Top/Bottom] ", app.logs.len()))
        .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let logs_widget = Paragraph::new(log_lines)
        .block(logs_block)
        .wrap(Wrap { trim: false });
    f.render_widget(logs_widget, chunks[1]);
}
