use crate::core::app::App;
use crate::ui::layout::centered_rect_fixed;
use crate::ui::theme::*;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;

pub fn render_sudo_modal(frame: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect_fixed(56, 12, area);
    frame.render_widget(Clear, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title & notice
            Constraint::Length(3), // Password input box
            Constraint::Length(2), // Error message if any
            Constraint::Length(2), // Key hints
        ])
        .split(popup_area);

    let action_title = app
        .pending_sudo_action
        .map(|a| match a {
            crate::core::state::ActionId::UpdateReload => "Hot Update & Reload",
            crate::core::state::ActionId::SyncConfigs => "Sync Configs & Themes",
            crate::core::state::ActionId::FactoryReset => "Factory Reset Arch Linux",
            _ => "Privileged Action",
        })
        .unwrap_or("Privileged Action");

    // Title Block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_ORANGE))
        .title(" ROOT AUTHENTICATION REQUIRED ")
        .title_style(
            Style::default()
                .fg(COLOR_ORANGE)
                .add_modifier(Modifier::BOLD),
        );

    let header_para = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            format!("'{}'", action_title),
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " requires administrator privileges.",
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
    ])])
    .block(block);
    frame.render_widget(header_para, chunks[0]);

    // Password Input Box
    let masked_password: String = "•".repeat(app.sudo_password_input.len());
    let input_line = Line::from(vec![
        Span::styled(
            " Password: ",
            Style::default()
                .fg(COLOR_TEXT_WHITE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            masked_password,
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ),
        Span::styled("▎", Style::default().fg(COLOR_ORANGE)),
    ]);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_CYAN));

    let input_para = Paragraph::new(vec![input_line]).block(input_block);
    frame.render_widget(input_para, chunks[1]);

    // Error Message
    let error_line = if let Some(err) = &app.sudo_error_message {
        Line::from(vec![Span::styled(
            format!("  [ERROR] {} ", err),
            Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
        )])
    } else {
        Line::from(vec![Span::styled(
            "  Enter your Linux sudo user password.",
            Style::default().fg(COLOR_TEXT_MUTED),
        )])
    };
    let error_para = Paragraph::new(vec![error_line]);
    frame.render_widget(error_para, chunks[2]);

    // Footer Hints
    let footer_line = Line::from(vec![
        Span::styled(
            " [Enter] ",
            Style::default()
                .fg(COLOR_EMERALD)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Authenticate   ", Style::default().fg(COLOR_TEXT_WHITE)),
        Span::styled(
            "[Esc] ",
            Style::default().fg(COLOR_RED).add_modifier(Modifier::BOLD),
        ),
        Span::styled("Cancel", Style::default().fg(COLOR_TEXT_WHITE)),
    ]);
    let footer_para = Paragraph::new(vec![footer_line]);
    frame.render_widget(footer_para, chunks[3]);
}
