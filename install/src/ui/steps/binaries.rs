use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::App;
use crate::models::BinaryLocation;
use crate::system::format_size;

pub fn draw_binaries_step(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(11), Constraint::Length(6)])
        .split(area);

    let build_from_source = app.is_build_from_source();

    let header_cells = ["", "Binary Name", "Status", "Size", "Target Location"]
        .into_iter()
        .map(|h| {
            ratatui::widgets::Cell::from(h).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        });
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.binaries.iter().enumerate().map(|(i, b)| {
        let is_cursor = i == app.binary_cursor;
        let checkbox = if b.selected {
            Span::styled(
                "[✓]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("[ ]", Style::default().fg(Color::DarkGray))
        };

        // In build-from-source mode every binary will be produced by the
        // release build, so treat missing files as "will build".
        let available = b.exists_in_source || build_from_source;

        let name_style = if is_cursor {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else if available {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let status_span = if !available {
            Span::styled("Missing in src", Style::default().fg(Color::Red))
        } else if build_from_source {
            Span::styled("Build from source", Style::default().fg(Color::Cyan))
        } else if b.exists_in_target {
            Span::styled("Installed (Update)", Style::default().fg(Color::Green))
        } else {
            Span::styled("Available (New)", Style::default().fg(Color::Blue))
        };

        let size_str = b
            .source_size_bytes
            .map(format_size)
            .unwrap_or_else(|| "--".into());
        let target_str = match b.default_dest {
            BinaryLocation::UserLocalBin => "~/.local/bin/",
            BinaryLocation::SystemBin => "/usr/bin/ (sudo)",
        };

        let row_style = if is_cursor {
            Style::default().bg(Color::Rgb(25, 30, 48))
        } else {
            Style::default()
        };

        Row::new(vec![
            ratatui::widgets::Cell::from(Line::from(checkbox)),
            ratatui::widgets::Cell::from(Span::styled(&b.name, name_style)),
            ratatui::widgets::Cell::from(status_span),
            ratatui::widgets::Cell::from(Span::styled(size_str, Style::default().fg(Color::Gray))),
            ratatui::widgets::Cell::from(Span::styled(
                target_str,
                Style::default().fg(Color::DarkGray),
            )),
        ])
        .style(row_style)
        .height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(4),
            Constraint::Length(22),
            Constraint::Length(20),
            Constraint::Length(12),
            Constraint::Min(16),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" 4. BabyDra Binaries [Pre-built Copy or Build-from-Source] ")
            .title_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(table, chunks[0]);

    if let Some(selected) = app.binaries.get(app.binary_cursor) {
        let card_lines = vec![
            Line::from(vec![
                Span::styled(
                    "Component: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    &selected.name,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  ({})", selected.crate_path),
                    Style::default().fg(Color::DarkGray),
                ),
            ]),
            Line::from(vec![
                Span::styled(
                    "Description: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(&selected.description, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled(
                    "Deploy: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(
                        "Direct binary copy (0755) -> ~/.local/bin/{} & /var/lib/babydra/bin/{}",
                        selected.name, selected.name
                    ),
                    Style::default().fg(Color::Green),
                ),
            ]),
        ];

        let card = Paragraph::new(card_lines).wrap(Wrap { trim: true }).block(
            Block::default()
                .title(" Selected Binary Details ")
                .title_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(card, chunks[1]);
    }
}
