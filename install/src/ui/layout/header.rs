use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::WizardStep;
use crate::system::is_root;
use crate::ui::THEME;

pub fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(area);

    let user_name = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let root_badge = if is_root() {
        Span::styled(
            " ROOT ",
            Style::default()
                .fg(THEME.rose)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            " USER ",
            Style::default()
                .fg(THEME.mint)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        )
    };

    let title_line = Line::from(vec![
        Span::styled(
            " 🐉 BabyDra ",
            Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "Desktop Shell Installer ",
            Style::default()
                .fg(THEME.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("v1.0.0", Style::default().fg(THEME.text_muted)),
    ]);

    let left_header = Paragraph::new(title_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(left_header, header_chunks[0]);

    let current_step_idx = app.current_step as usize + 1;
    let right_line = Line::from(vec![
        root_badge,
        Span::raw(" "),
        Span::styled(
            format!("{user_name}@arch "),
            Style::default().fg(THEME.text_body),
        ),
        Span::styled("│ Step ", Style::default().fg(THEME.text_muted)),
        Span::styled(
            format!("{current_step_idx}/{}: ", WizardStep::ALL.len()),
            Style::default()
                .fg(THEME.amber)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            app.current_step.short_name(),
            Style::default()
                .fg(THEME.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    let right_header = Paragraph::new(right_line)
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.border_normal)),
        );
    f.render_widget(right_header, header_chunks[1]);
}
