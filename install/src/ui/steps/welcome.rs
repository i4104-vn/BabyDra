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
        Line::from("Công cụ cài đặt TUI chính thức cho môi trường desktop Wayland BabyDra trên Arch Linux."),
        Line::from(Span::styled("Cơ chế triển khai: Đồng bộ mã nguồn từ nhánh, biên dịch và sao chép tệp nhị phân vào hệ thống.", Style::default().fg(Color::Green))),
        Line::from(Span::styled("Nhấn phím [c] để chuyển đổi kênh Release và Develop.", Style::default().fg(Color::Yellow))),
    ];

    let banner = Paragraph::new(banner_text)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" 1. Tổng quan & Giới thiệu ")
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
            Span::styled("Kênh cài đặt:    ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.install_channel.name(), Style::default().fg(channel_color).add_modifier(Modifier::BOLD)),
            Span::styled("  [Nhấn 'c' để đổi kênh]", Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("Tên nhánh Git:   ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(branch_name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::styled("    Mã Hash Commit: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(commit_hash, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Người push:      ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(author_name, Style::default().fg(Color::White)),
            Span::styled("    Ngày update:    ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(update_date, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Nội dung commit: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(commit_msg, Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("Thư mục nhị phân:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(app.source_binary_dir.to_string_lossy().to_string(), Style::default().fg(Color::White)),
            Span::styled("  [Nhấn 's' để đổi đường dẫn]", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let sys_box = Paragraph::new(sys_info)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(" Thông tin Bản cài đặt & Nhánh nguồn Git ")
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
            .title(" Chọn Hồ sơ cài đặt [↑/↓: Chọn hồ sơ | c: Đổi kênh Release/Develop | Enter: Tiếp tục] ")
            .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Magenta)),
    );
    f.render_widget(profile_list, chunks[2]);
}
