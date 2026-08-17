use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::PresetProfile;
use crate::ui::THEME;

pub fn draw_welcome_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(8),
            Constraint::Min(6),
        ])
        .split(area);

    let banner_text = vec![
        Line::from(vec![
            Span::styled("✨ Welcome to BabyDra Desktop Shell Installer ", THEME.title_cyan()),
            Span::styled("— Modern Wayland Shell for Arch Linux", Style::default().fg(THEME.text_dim)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("• Flexible Installation: ", Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD)),
            Span::styled("Deploy pre-built binaries instantly or check out a git branch to rebuild from source.", Style::default().fg(THEME.text_body)),
        ]),
        Line::from(vec![
            Span::styled("• System Staging: ", Style::default().fg(THEME.purple).add_modifier(Modifier::BOLD)),
            Span::styled("Executables deployed to ~/.local/bin and /var/lib/babydra for seamless desktop and greeter access.", Style::default().fg(THEME.text_body)),
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
            Span::styled("◆ Workspace Root:    ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                app.workspace_root.to_string_lossy().to_string(),
                Style::default().fg(THEME.text_bright).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Binary Source Dir: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                app.source_binary_dir.to_string_lossy().to_string(),
                Style::default().fg(THEME.text_bright),
            ),
            Span::styled("  [Press 's' to customize]", Style::default().fg(THEME.text_muted)),
        ]),
        Line::from(vec![
            Span::styled("◆ Pre-built Binaries:", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!(" {found_bins}/{} available", app.binaries.len()),
                Style::default()
                    .fg(if found_bins == app.binaries.len() {
                        THEME.mint
                    } else {
                        THEME.amber
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Target Bin Folder: ", Style::default().fg(THEME.text_dim)),
            Span::styled("~/.local/bin", Style::default().fg(THEME.cyan)),
            Span::styled(" & ", Style::default().fg(THEME.text_muted)),
            Span::styled("/var/lib/babydra/bin", Style::default().fg(THEME.purple)),
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

    let profiles = [
        PresetProfile::FullDesktop,
        PresetProfile::BinariesAndBundle,
        PresetProfile::Custom,
    ];

    let profile_items: Vec<ListItem> = profiles
        .iter()
        .map(|p| {
            let is_selected = *p == app.current_profile;
            let radio = if is_selected {
                Span::styled(
                    "(●) ",
                    Style::default()
                        .fg(THEME.cyan)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("( ) ", Style::default().fg(THEME.text_muted))
            };

            let title_style = if is_selected {
                Style::default()
                    .fg(THEME.cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(THEME.text_body)
            };

            let desc = Span::styled(
                format!("    {}", p.description()),
                Style::default().fg(if is_selected { THEME.text_dim } else { THEME.text_muted }),
            );

            let hint = if is_selected {
                Span::styled("  [↑/↓: Đổi Profile]", Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD))
            } else {
                Span::raw("")
            };

            ListItem::new(vec![
                Line::from(vec![radio, Span::styled(p.name(), title_style), hint]),
                Line::from(desc),
            ])
            .style(if is_selected {
                Style::default().bg(THEME.bg_selected)
            } else {
                Style::default()
            })
        })
        .collect();

    let profile_list = List::new(profile_items).block(
        Block::default()
            .title(" Choose Preset Installation Profile [↑/↓: Switch | Enter: Next] ")
            .title_style(THEME.title_purple())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.purple)),
    );
    f.render_widget(profile_list, chunks[2]);
}
