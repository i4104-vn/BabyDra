use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::InstallState;
use crate::ui::THEME;

pub fn draw_summary_step(f: &mut Frame, app: &App, area: Rect) {
    let (status_text, status_color) = match app.install_state {
        InstallState::Completed { success: true, .. } => (
            "  ✔  BabyDra Desktop Shell Installation Completed Successfully!  ".to_string(),
            THEME.mint,
        ),
        InstallState::Completed {
            success: false,
            total_errors,
            ..
        } => (
            format!("  ⚠  Completed with {} warnings / errors  ", total_errors),
            THEME.amber,
        ),
        _ => ("  Installation Ready  ".to_string(), THEME.cyan),
    };

    let summary_lines = vec![
        Line::from(Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("◆ Installed Binaries:    ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "~/.local/bin & /var/lib/babydra/bin",
                Style::default().fg(THEME.text_bright).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Compositor Config:     ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "~/.config/labwc (autostart, rc.xml, scripts)",
                Style::default().fg(THEME.cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ System Themes & Icons: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "We10X icons, BabyDra GTK theme, Twilight cursors",
                Style::default().fg(THEME.purple),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Selected Variant:      ", Style::default().fg(THEME.text_dim)),
            Span::styled(&app.selected_variant, Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("◆ Installation Source:   ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                if app.is_build_from_source() {
                    format!("Branch '{}' (compiled with cargo --release)", app.selected_branch)
                } else {
                    "Pre-built binaries (direct copy)".to_string()
                },
                Style::default().fg(THEME.amber),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "🚀 Next Steps & Commands:",
            THEME.title_cyan(),
        )),
        Line::from(vec![
            Span::styled("  • Launch Desktop Session:   ", Style::default().fg(THEME.text_dim)),
            Span::styled("labwc", Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  • Start Greeter Display Mgr:", Style::default().fg(THEME.text_dim)),
            Span::styled("sudo systemctl restart greetd", Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("  • Real-time Desktop Logs:   ", Style::default().fg(THEME.text_dim)),
            Span::styled("tail -f ~/.cache/babydra/panel.log", Style::default().fg(THEME.cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" [Enter / q] ", THEME.key_badge_green()),
            Span::styled(" Exit installer and return to shell", Style::default().fg(THEME.text_body)),
        ]),
    ];

    let summary_widget = Paragraph::new(summary_lines).block(
        Block::default()
            .title(" 10. Summary & Launch Instructions ")
            .title_style(Style::default().fg(status_color).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(status_color)),
    );
    f.render_widget(summary_widget, area);
}
