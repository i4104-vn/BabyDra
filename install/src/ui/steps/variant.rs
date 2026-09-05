use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

pub fn draw_variant_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(7)])
        .split(area);

    let items: Vec<ListItem> = app
        .variant_options
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let is_cursor = i == app.variant_cursor;
            let radio = if v.selected {
                Span::styled(
                    "(●) ",
                    Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("( ) ", Style::default().fg(THEME.text_muted))
            };

            let title_style = if is_cursor {
                Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD)
            } else if v.selected {
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(THEME.text_dim)
            };

            let row_style = if is_cursor {
                Style::default().bg(THEME.bg_cursor)
            } else if v.selected {
                Style::default().bg(THEME.bg_card)
            } else {
                Style::default()
            };

            let placeholder_hint = if v.selected {
                Span::styled(
                    " [Selected] ",
                    Style::default()
                        .fg(THEME.mint)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )
            } else if is_cursor {
                Span::styled(
                    " [Space: Select Variant] ",
                    Style::default()
                        .fg(THEME.amber)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("")
            };

            ListItem::new(vec![
                Line::from(vec![
                    radio,
                    Span::styled(&v.name, title_style),
                    Span::raw("  "),
                    Span::styled(
                        format!("[theme: {}]", v.theme),
                        Style::default().fg(THEME.cyan),
                    ),
                    Span::raw(" "),
                    placeholder_hint,
                ]),
                Line::from(Span::styled(
                    format!("    Apps: {}", v.apps_preview()),
                    Style::default().fg(if is_cursor {
                        THEME.text_body
                    } else {
                        THEME.text_muted
                    }),
                )),
            ])
            .style(row_style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 4. Theme & Variant Selection [↑/↓: Move | Space: Select | Enter: Next] ")
            .title_style(THEME.title_purple())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.purple)),
    );
    f.render_widget(list, chunks[0]);

    let selected = app
        .variant_options
        .iter()
        .find(|v| v.selected)
        .map(|v| v.name.as_str())
        .unwrap_or("default");

    let prompt_box = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("◆ Active Variant: ", Style::default().fg(THEME.text_dim)),
            Span::styled(selected, Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD)),
            Span::styled(" — Cấu hình theme GTK, icons, và phím tắt compositor.", Style::default().fg(THEME.text_muted)),
        ]),
        Line::from("Mỗi biến thể định nghĩa bộ cấu hình hoàn chỉnh cho desktop shell labwc và các ứng dụng đi kèm."),
        Line::from(vec![
            Span::styled(" [Space] ", THEME.key_badge_green()),
            Span::styled(" Chọn biến thể    ", Style::default().fg(THEME.text_dim)),
            Span::styled(" [Enter] ", THEME.key_badge_cyan()),
            Span::styled(" Xác nhận & Tiếp tục    ", Style::default().fg(THEME.text_dim)),
            Span::styled(" [←] ", THEME.key_badge_amber()),
            Span::styled(" Quay lại", Style::default().fg(THEME.text_dim)),
        ]),
    ])
    .block(
        Block::default()
            .title(" Variant System Description ")
            .title_style(THEME.title_purple())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(prompt_box, chunks[1]);
}
