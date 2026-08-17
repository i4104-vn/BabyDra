use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::InstallState;

pub fn draw_summary_step(f: &mut Frame, app: &App, area: Rect) {
    let (status_text, status_color) = match app.install_state {
        InstallState::Completed { success: true, .. } => (
            "✓ Installation & Configuration Completed Successfully!".to_string(),
            Color::Green,
        ),
        InstallState::Completed {
            success: false,
            total_errors,
            ..
        } => (
            format!("⚠ Completed with {} warnings/errors", total_errors),
            Color::Yellow,
        ),
        _ => ("Installation Ready".to_string(), Color::Cyan),
    };

    let summary_lines = vec![
        Line::from(Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Installed Binaries:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "~/.local/bin & /var/lib/babydra/bin",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Compositor Config:    ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "~/.config/labwc (autostart, rc.xml, scripts)",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "System Themes & Icons:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "We10X icons, BabyDra GTK theme, Twilight cursors",
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Selected Variant:     ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(&app.selected_variant, Style::default().fg(Color::Magenta)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Next Steps & Launching:",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  • Launch Desktop Shell:       labwc"),
        Line::from("  • Start/Restart Greeter:      sudo systemctl restart greetd"),
        Line::from("  • Log File Location:          ~/.cache/babydra/panel.log"),
        Line::from(""),
        Line::from(Span::styled(
            "Press [Enter] or [q] to exit the installer.",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let summary_widget = Paragraph::new(summary_lines).block(
        Block::default()
            .title(" 9. Summary & Launch Instructions ")
            .title_style(
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(status_color)),
    );
    f.render_widget(summary_widget, area);
}
