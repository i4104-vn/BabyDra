use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::{InstallChannel, PresetProfile};

pub fn draw_welcome_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Length(9),
            Constraint::Min(6),
        ])
        .split(area);

    let banner_text = vec![
        Line::from(Span::styled("BabyDra Desktop Shell Installer", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Official step-by-step TUI installer for the BabyDra Wayland desktop environment on Arch Linux."),
        Line::from(Span::styled("Direct deployment: Copies pre-built binaries into ~/.local/bin and /var/lib/babydra.", Style::default().fg(Color::Green))),
        Line::from(Span::styled("Press 'c' to switch between Release and Develop channels.", Style::default().fg(Color::Yellow))),
    ];

    let banner = Paragraph::new(banner_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" 1. Welcome & Overview ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(banner, chunks[0]);

    let found_bins = app.binaries.iter().filter(|b| b.exists_in_source).count();
    let channel_color = match app.install_channel {
        InstallChannel::Release => Color::Green,
        InstallChannel::Develop => Color::Magenta,
        InstallChannel::LocalSource => Color::Cyan,
    };

    let sys_info = vec![
        Line::from(vec![
            Span::styled("Active Channel:     ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.install_channel.name(), Style::default().fg(channel_color).add_modifier(Modifier::BOLD)),
            Span::styled(" [Press 'c' to switch channel]", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Channel Info:       ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.install_channel.description(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Binary Source Dir:  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.source_binary_dir.to_string_lossy().to_string(), Style::default().fg(Color::White)),
            Span::styled(" [Press 's' to change]", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Pre-built Status:   ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("{found_bins}/{} binaries detected and ready", app.binaries.len()),
                Style::default().fg(if found_bins == app.binaries.len() { Color::Green } else { Color::Yellow }),
            ),
        ]),
    ];

    let sys_box = Paragraph::new(sys_info).block(
        Block::default()
            .title(" Environment & Channel Configuration ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
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
                Span::styled("(●) ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            } else {
                Span::styled("( ) ", Style::default().fg(Color::DarkGray))
            };

            let title_style = if is_selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            let desc = Span::styled(format!("    {}", p.description()), Style::default().fg(Color::DarkGray));

            ListItem::new(vec![
                Line::from(vec![radio, Span::styled(p.name(), title_style)]),
                Line::from(desc),
            ])
            .style(if is_selected { Style::default().bg(Color::Rgb(25, 30, 48)) } else { Style::default() })
        })
        .collect();

    let profile_list = List::new(profile_items).block(
        Block::default()
            .title(" Choose Installation Profile [↑/↓: Switch profile | c: Switch channel | Enter/n: Next] ")
            .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    f.render_widget(profile_list, chunks[2]);
}
