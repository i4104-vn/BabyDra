use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::ui::THEME;

/// Draws step 2: choose the git branch to install from.
pub fn draw_branch_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(8)])
        .split(area);

    if app.branches.is_empty() {
        // Empty state when no branches exist
        let empty_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  Không có bản cài đặt nào khả dụng trong kho mã nguồn!  ",
                THEME.title_rose(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Không tìm thấy nhánh cài đặt nào (nhánh 'release' hoặc các nhánh khác).",
                Style::default().fg(THEME.text_bright),
            )),
            Line::from(Span::styled(
                "Nhánh 'main' chỉ lưu trữ bộ cài và tài liệu, không chứa mã nguồn môi trường desktop.",
                Style::default().fg(THEME.text_dim),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Gợi ý: Kiểm tra kết nối git remote hoặc thực hiện git fetch origin release.",
                Style::default().fg(THEME.amber),
            )),
        ];

        let empty_widget = Paragraph::new(empty_lines)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(" 2. Source Branch — Không Có Bản Cài Đặt Khả Dụng ")
                    .title_style(THEME.title_rose())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(THEME.rose)),
            );
        f.render_widget(empty_widget, chunks[0]);

        let prompt_box = Paragraph::new(vec![
            Line::from(Span::styled(
                "Trạng thái: Không thể tiếp tục vì không tìm thấy bản cài đặt nào.",
                Style::default().fg(THEME.rose).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("Repository: {}", app.workspace_root.display()),
                Style::default().fg(THEME.text_dim),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled(" [q] ", THEME.key_badge_red()),
                Span::styled(" Thoát installer    ", Style::default().fg(THEME.text_dim)),
                Span::styled(" [← / p] ", THEME.key_badge_amber()),
                Span::styled(" Quay lại trang tổng quan", Style::default().fg(THEME.text_dim)),
            ]),
        ])
        .block(
            Block::default()
                .title(" Thông Báo Cài Đặt ")
                .title_style(THEME.title_rose())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.rose)),
        );
        f.render_widget(prompt_box, chunks[1]);
        return;
    }

    let mut items: Vec<ListItem> = Vec::with_capacity(app.branches.len());

    for (i, b) in app.branches.iter().enumerate() {
        let is_recommended = b.name == "release";
        let is_selected = app.selected_branch == b.name;
        items.push(branch_row(
            i,
            &app.branch_cursor,
            &b.name,
            b.is_current,
            b.has_remote,
            is_recommended,
            is_selected,
        ));
    }

    let list = List::new(items).block(
        Block::default()
            .title(" 2. Source Branch Selection [↑/↓: Move | Space: Select | Enter: Switch Branch] ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(list, chunks[0]);

    let prompt_box = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Target Branch : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("'{}'", app.selected_branch),
                Style::default()
                    .fg(THEME.cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " — Tải vào branches/ để giữ nguyên nhánh main.",
                Style::default().fg(THEME.mint),
            ),
        ]),
        Line::from(vec![
            Span::styled("Repository    : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{} ({} nhánh khả dụng)", app.workspace_root.display(), app.branches.len()),
                Style::default().fg(THEME.text_bright),
            ),
        ]),
        Line::from(vec![
            Span::styled("Khuyến nghị   : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "Nhánh 'release' là phiên bản hoàn thiện, ổn định và đầy đủ nhất.",
                Style::default().fg(THEME.amber),
            ),
        ]),
        Line::from(vec![
            Span::styled(" [Space] ", THEME.key_badge_green()),
            Span::styled(" Chọn nhánh    ", Style::default().fg(THEME.text_dim)),
            Span::styled(" [Enter] ", THEME.key_badge_cyan()),
            Span::styled(" Xác nhận & Tải nhánh    ", Style::default().fg(THEME.text_dim)),
            Span::styled(" [←] ", THEME.key_badge_amber()),
            Span::styled(" Quay lại", Style::default().fg(THEME.text_dim)),
        ]),
    ])
    .block(
        Block::default()
            .title(" Target Branch Information ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(prompt_box, chunks[1]);
}

fn branch_row<'a>(
    row_idx: usize,
    cursor: &usize,
    name: &'a str,
    is_current: bool,
    has_remote: bool,
    is_recommended: bool,
    is_selected: bool,
) -> ListItem<'a> {
    let is_cursor = row_idx == *cursor;

    let radio = if is_selected {
        Span::styled(
            "(●) ",
            Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled("( ) ", Style::default().fg(THEME.text_muted))
    };

    let title_style = if is_cursor {
        Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD)
    } else if is_selected {
        Style::default()
            .fg(THEME.text_bright)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(THEME.text_body)
    };

    let mut spans = vec![radio, Span::styled(name, title_style)];

    if is_recommended {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [Khuyến nghị] ",
            Style::default()
                .fg(THEME.amber)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        ));
    }

    if is_current {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [local] ",
            Style::default().fg(THEME.purple).bg(THEME.bg_badge),
        ));
    }

    if has_remote {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [remote] ",
            Style::default().fg(THEME.text_dim).bg(THEME.bg_badge),
        ));
    }

    if is_selected {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [ĐÃ CHỌN] ",
            Style::default()
                .fg(THEME.mint)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        ));
    } else if is_cursor {
        spans.push(Span::raw(" "));
        spans.push(Span::styled(
            " [Space: Chọn] ",
            Style::default()
                .fg(THEME.cyan)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let bg_style = if is_cursor {
        Style::default().bg(THEME.bg_cursor)
    } else if is_selected {
        Style::default().bg(THEME.bg_selected)
    } else {
        Style::default()
    };

    ListItem::new(Line::from(spans)).style(bg_style)
}
