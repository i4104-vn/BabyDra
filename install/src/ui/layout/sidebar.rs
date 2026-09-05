use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::WizardStep;
use crate::ui::THEME;

pub fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(9)])
        .split(area);

    let current_step_idx = app.current_step as usize;

    let items: Vec<ListItem> = WizardStep::ALL
        .iter()
        .map(|step| {
            let step_idx = *step as usize;
            let is_current = *step == app.current_step;

            let (icon, style) = if is_current {
                (
                    "▶ ",
                    Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
                )
            } else if step_idx < current_step_idx {
                ("✔ ", Style::default().fg(THEME.mint))
            } else {
                ("○ ", Style::default().fg(THEME.text_muted))
            };

            let text_style = if is_current {
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD)
            } else if step_idx < current_step_idx {
                Style::default().fg(THEME.text_body)
            } else {
                Style::default().fg(THEME.text_dim)
            };

            let row_style = if is_current {
                Style::default().bg(THEME.bg_cursor)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(icon, style),
                Span::styled(step.title(), text_style),
            ]))
            .style(row_style)
        })
        .collect();

    let steps_list = List::new(items).block(
        Block::default()
            .title(" Navigation [1-6] ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(steps_list, sidebar_chunks[0]);

    let build_from_source = app.is_build_from_source();
    let selected_bins = app
        .binaries
        .iter()
        .filter(|b| b.selected && (b.exists_in_source || build_from_source))
        .count();
    let total_bins = app.binaries.len();
    let selected_variant = app
        .variant_options
        .iter()
        .find(|v| v.selected)
        .map(|v| v.name.as_str())
        .unwrap_or("default");

    let summary_lines = vec![
        Line::from(vec![
            Span::styled("◆ Components: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_bins}/{total_bins}"),
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Variant:    ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                selected_variant,
                Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Mode:       ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                if build_from_source {
                    format!("branch '{}'", app.selected_branch)
                } else {
                    "pre-built only".to_string()
                },
                Style::default()
                    .fg(if build_from_source {
                        THEME.amber
                    } else {
                        THEME.mint
                    })
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Automatic:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "packages, configs, greetd",
                Style::default().fg(THEME.text_body),
            ),
        ]),
    ];

    let summary_box = Paragraph::new(summary_lines).block(
        Block::default()
            .title(" Plan Summary ")
            .title_style(THEME.title_amber())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(summary_box, sidebar_chunks[1]);
}
