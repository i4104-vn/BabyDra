use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::BinaryLocation;
use crate::system::format_size;
use crate::ui::THEME;

pub fn draw_binaries_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(11), Constraint::Length(6)])
        .split(area);

    let build_from_source = app.is_build_from_source();

    let header_cells = ["", "Binary Executable", "Build Status", "Size", "Destination Target"]
        .into_iter()
        .map(|h| {
            ratatui::widgets::Cell::from(h).style(THEME.title_cyan())
        });
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.binaries.iter().enumerate().map(|(i, b)| {
        let is_cursor = i == app.binary_cursor;
        let checkbox = if b.selected {
            Span::styled(
                "[✔]",
                Style::default()
                    .fg(THEME.mint)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("[ ]", Style::default().fg(THEME.text_muted))
        };

        let available = b.exists_in_source || build_from_source;

        let name_style = if is_cursor {
            Style::default()
                .fg(THEME.cyan)
                .add_modifier(Modifier::BOLD)
        } else if b.selected {
            Style::default()
                .fg(THEME.text_bright)
                .add_modifier(Modifier::BOLD)
        } else if available {
            Style::default().fg(THEME.text_body)
        } else {
            Style::default().fg(THEME.text_muted)
        };

        let status_span = if !available {
            Span::styled("Missing in src", Style::default().fg(THEME.rose))
        } else if build_from_source {
            Span::styled("▲ Build from source", Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD))
        } else if b.exists_in_target {
            Span::styled("● Installed (Update)", Style::default().fg(THEME.mint))
        } else {
            Span::styled("✦ Available (New)", Style::default().fg(THEME.blue))
        };

        let size_str = b
            .source_size_bytes
            .map(format_size)
            .unwrap_or_else(|| "--".into());
        let target_str = match b.default_dest {
            BinaryLocation::UserLocalBin => "~/.local/bin/ (user)",
            BinaryLocation::SystemBin => "/usr/bin/ (system root)",
        };

        let row_style = if is_cursor {
            Style::default().bg(THEME.bg_cursor)
        } else if b.selected {
            Style::default().bg(THEME.bg_card)
        } else {
            Style::default()
        };

        let name_line = if is_cursor {
            Line::from(vec![
                Span::styled(&b.name, name_style),
                Span::styled(" [Space: Bật/Tắt]", Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD)),
            ])
        } else {
            Line::from(Span::styled(&b.name, name_style))
        };

        Row::new(vec![
            ratatui::widgets::Cell::from(Line::from(checkbox)),
            ratatui::widgets::Cell::from(name_line),
            ratatui::widgets::Cell::from(status_span),
            ratatui::widgets::Cell::from(Span::styled(size_str, Style::default().fg(THEME.text_dim))),
            ratatui::widgets::Cell::from(Span::styled(
                target_str,
                Style::default().fg(THEME.text_muted),
            )),
        ])
        .style(row_style)
        .height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(32),
            Constraint::Length(23),
            Constraint::Length(12),
            Constraint::Min(16),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" 4. BabyDra Binary Executables [Space: Toggle | a: Select All | Enter: Next] ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(table, chunks[0]);

    if let Some(selected) = app.binaries.get(app.binary_cursor) {
        let card_lines = vec![
            Line::from(vec![
                Span::styled("◆ Component:   ", Style::default().fg(THEME.text_dim)),
                Span::styled(
                    &selected.name,
                    Style::default()
                        .fg(THEME.text_bright)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  ({})", selected.crate_path),
                    Style::default().fg(THEME.text_muted),
                ),
            ]),
            Line::from(vec![
                Span::styled("◆ Description: ", Style::default().fg(THEME.text_dim)),
                Span::styled(&selected.description, Style::default().fg(THEME.text_body)),
            ]),
            Line::from(vec![
                Span::styled("◆ Deploy Path: ", Style::default().fg(THEME.text_dim)),
                Span::styled(
                    format!(
                        "Executable permissions (0755) -> ~/.local/bin/{} & /var/lib/babydra/bin/{}",
                        selected.name, selected.name
                    ),
                    Style::default().fg(THEME.mint),
                ),
            ]),
        ];

        let card = Paragraph::new(card_lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" Selected Component Inspection ")
                .title_style(THEME.title_amber())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.border_normal)),
        );
        f.render_widget(card, chunks[1]);
    }
}
