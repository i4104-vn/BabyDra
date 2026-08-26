use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

pub fn draw_welcome_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(6)])
        .split(area);

    let banner_text = vec![
        Line::from(vec![
            Span::styled("Welcome to the BabyDra Desktop Shell Installer ", THEME.title_cyan()),
            Span::styled("— a lightweight Wayland desktop environment for Arch Linux", Style::default().fg(THEME.text_dim)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("• Installation modes: ", Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD)),
            Span::styled("deploy pre-built binaries directly, or check out a git branch and rebuild from source.", Style::default().fg(THEME.text_body)),
        ]),
        Line::from(vec![
            Span::styled("• Automatic configuration: ", Style::default().fg(THEME.purple).add_modifier(Modifier::BOLD)),
            Span::styled("system packages, /var/lib staging, labwc/GTK/terminal configs, themes, icons and greetd are installed unconditionally — no per-task prompts.", Style::default().fg(THEME.text_body)),
        ]),
        Line::from(vec![
            Span::styled("• Component selection: ", Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD)),
            Span::styled("all discovered BabyDra components are selected by default; individual components can be deselected in the next steps.", Style::default().fg(THEME.text_body)),
        ]),
    ];

    let banner = Paragraph::new(banner_text).wrap(Wrap { trim: true }).block(
        Block::default()
            .title(" 1. Welcome & Overview ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(banner, chunks[0]);

    let found_bins = app.binaries.iter().filter(|b| b.exists_in_source).count();
    let sys_info = vec![
        Line::from(vec![
            Span::styled("Workspace Root:     ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                app.workspace_root.to_string_lossy().to_string(),
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Binary Source Dir:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                app.source_binary_dir.to_string_lossy().to_string(),
                Style::default().fg(THEME.text_bright),
            ),
            Span::styled(
                "  [Press 's' to customize]",
                Style::default().fg(THEME.text_muted),
            ),
        ]),
        Line::from(vec![
            Span::styled("Components:         ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!(
                    "{} discovered ({} pre-built available) — all selected by default",
                    app.binaries.len(),
                    found_bins
                ),
                Style::default()
                    .fg(
                        if !app.binaries.is_empty() && found_bins == app.binaries.len() {
                            THEME.mint
                        } else {
                            THEME.amber
                        },
                    )
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Install Targets:    ", Style::default().fg(THEME.text_dim)),
            Span::styled("~/.local/bin", Style::default().fg(THEME.cyan)),
            Span::styled(" and ", Style::default().fg(THEME.text_muted)),
            Span::styled("/var/lib/babydra/bin", Style::default().fg(THEME.purple)),
            Span::styled(
                "  [Enter: Next step]",
                Style::default().fg(THEME.text_muted),
            ),
        ]),
    ];

    let sys_box = Paragraph::new(sys_info).block(
        Block::default()
            .title(" Pre-flight Environment Inspection ")
            .title_style(THEME.title_amber())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(sys_box, chunks[1]);
}
