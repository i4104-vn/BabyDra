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
            Constraint::Length(7),
            Constraint::Length(11),
            Constraint::Min(6),
        ])
        .split(area);

    let banner_text = vec![
        Line::from(Span::styled("BabyDra Desktop Shell Installer", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from("Official step-by-step TUI installer for the BabyDra Wayland desktop environment on Arch Linux."),
        Line::from(Span::styled("Deployment mechanism: Syncs branch source code, compiles binaries, and deploys system configurations.", Style::default().fg(Color::Green))),
        Line::from(Span::styled("Press [c] to switch between Release and Develop channels.", Style::default().fg(Color::Yellow))),
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

    let channel_color = match app.install_channel {
        InstallChannel::Release => Color::Green,
        InstallChannel::Develop => Color::Magenta,
        InstallChannel::LocalSource => Color::Cyan,
    };

    let meta = app.get_current_channel_meta();
    let branch_name = meta.map(|m| m.branch_name.as_str()).unwrap_or("N/A");
    let commit_hash = meta.map(|m| m.commit_hash.as_str()).unwrap_or("N/A");
    let author_name = meta.map(|m| m.author_name.as_str()).unwrap_or("N/A");
    let update_date = meta.map(|m| m.update_date.as_str()).unwrap_or("N/A");
    let commit_msg = meta.map(|m| m.commit_msg.as_str()).unwrap_or("N/A");

    let sys_info = vec![
        Line::from(vec![
            Span::styled("Install Channel:  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.install_channel.name(), Style::default().fg(channel_color).add_modifier(Modifier::BOLD)),
            Span::styled("  [Press 'c' to switch channel]", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Git Branch:       ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(branch_name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("    Commit Hash:    ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(commit_hash, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Author / Pusher:  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(author_name, Style::default().fg(Color::White)),
            Span::styled("    Update Date:    ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(update_date, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Latest Commit:    ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(commit_msg, Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("Binary Directory: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.source_binary_dir.to_string_lossy().to_string(), Style::default().fg(Color::White)),
            Span::styled("  [Press 's' to change directory]", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let sys_box = Paragraph::new(sys_info)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" Installation Channel & Git Branch Metadata ")
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
            .title(" Choose Installation Profile [↑/↓: Switch profile | c: Switch channel | Enter: Next step] ")
            .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    f.render_widget(profile_list, chunks[2]);
}
