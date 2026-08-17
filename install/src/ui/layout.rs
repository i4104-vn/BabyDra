use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::WizardStep;
use crate::system::is_root;
use crate::ui::THEME;

pub fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(area);

    let user_name = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let root_badge = if is_root() {
        Span::styled(
            " ROOT ",
            Style::default()
                .fg(THEME.rose)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            " USER ",
            Style::default()
                .fg(THEME.mint)
                .bg(THEME.bg_badge)
                .add_modifier(Modifier::BOLD),
        )
    };

    let title_line = Line::from(vec![
        Span::styled(
            " 🐉 BabyDra ",
            Style::default()
                .fg(THEME.cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "Desktop Shell Installer ",
            Style::default()
                .fg(THEME.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("v1.0.0", Style::default().fg(THEME.text_muted)),
    ]);

    let left_header = Paragraph::new(title_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.cyan)),
    );
    f.render_widget(left_header, header_chunks[0]);

    let current_step_idx = app.current_step as usize + 1;
    let right_line = Line::from(vec![
        root_badge,
        Span::raw(" "),
        Span::styled(
            format!("{user_name}@arch "),
            Style::default().fg(THEME.text_body),
        ),
        Span::styled("│ Step ", Style::default().fg(THEME.text_muted)),
        Span::styled(
            format!("{current_step_idx}/10: "),
            Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            app.current_step.short_name(),
            Style::default()
                .fg(THEME.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    let right_header = Paragraph::new(right_line)
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.border_normal)),
        );
    f.render_widget(right_header, header_chunks[1]);
}

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
                    Style::default()
                        .fg(THEME.cyan)
                        .add_modifier(Modifier::BOLD),
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
            .title(" 󰇊 Navigation [1-9, 0] ")
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
    let selected_varlib = app.varlib_options.iter().filter(|o| o.selected).count();
    let selected_cfgs = app
        .configs_themes_options
        .iter()
        .filter(|o| o.selected)
        .count();
    let selected_dm = app
        .display_manager_options
        .iter()
        .filter(|o| o.selected)
        .count();
    let selected_variant = app
        .variant_options
        .iter()
        .find(|v| v.selected)
        .map(|v| v.name.as_str())
        .unwrap_or("default");

    let summary_lines = vec![
        Line::from(vec![
            Span::styled("◆ Binaries:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_bins}/{total_bins}"),
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ /var/lib:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_varlib} staged"),
                Style::default().fg(THEME.purple).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Configs:   ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_cfgs} enabled"),
                Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Greetd DM: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                if selected_dm > 0 {
                    "Enabled"
                } else {
                    "Skipped"
                },
                Style::default().fg(if selected_dm > 0 {
                    THEME.cyan
                } else {
                    THEME.text_muted
                }).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Variant:   ", Style::default().fg(THEME.text_dim)),
            Span::styled(selected_variant, Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("◆ Mode:      ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                if build_from_source {
                    format!("★ branch '{}'", app.selected_branch)
                } else {
                    "● pre-built only".to_string()
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

pub fn draw_shortcuts_panel(f: &mut Frame, _app: &App, area: Rect) {
    let shortcuts = [
        ("Space", "Toggle / Select", THEME.mint),
        ("Tab / n", "Next Step", THEME.cyan),
        ("p", "Previous Step", THEME.cyan),
        ("↑ / ↓", "Navigate Items", THEME.cyan),
        ("a", "Select All", THEME.cyan),
        ("i / ↵", "Start Install", THEME.mint),
        ("s", "Binary Folder", THEME.amber),
        ("r", "Rescan Source", THEME.purple),
        ("?", "Help Dialog", THEME.purple),
        ("q", "Quit Installer", THEME.rose),
    ];

    let items: Vec<ListItem> = shortcuts
        .iter()
        .map(|(key, desc, col)| {
            ListItem::new(vec![
                Line::from(Span::styled(
                    format!(" {key} "),
                    Style::default()
                        .fg(*col)
                        .bg(THEME.bg_badge)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::styled(
                    format!(" {desc}"),
                    Style::default().fg(THEME.text_dim),
                )),
            ])
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" 󰌌 Shortcuts ")
            .title_style(THEME.title_cyan())
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(THEME.border_normal)),
    );
    f.render_widget(list, area);
}

pub fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let step_info = app.current_step.short_name();
    let footer_text = vec![
        Span::styled(" 🐉 BabyDra Installer ", THEME.title_cyan()),
        Span::styled("│ Wayland Compositor ", Style::default().fg(THEME.text_muted)),
        Span::styled("│ Step: ", Style::default().fg(THEME.text_dim)),
        Span::styled(step_info, Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD)),
        Span::styled(" │ ", Style::default().fg(THEME.text_muted)),
        Span::styled(" Space ", THEME.key_badge_green()),
        Span::styled(" Toggle ", Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD)),
        Span::styled("│ ", Style::default().fg(THEME.text_muted)),
        Span::styled(" Tab/n ", THEME.key_badge()),
        Span::styled(" Next ", Style::default().fg(THEME.text_body)),
        Span::styled("│ ", Style::default().fg(THEME.text_muted)),
        Span::styled(" p ", THEME.key_badge()),
        Span::styled(" Prev ", Style::default().fg(THEME.text_body)),
        Span::styled("│ ", Style::default().fg(THEME.text_muted)),
        Span::styled(" ? ", Style::default().fg(THEME.purple).bg(THEME.bg_badge).add_modifier(Modifier::BOLD)),
        Span::styled(" Help ", Style::default().fg(THEME.text_body)),
    ];

    let footer = Paragraph::new(Line::from(footer_text))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.border_normal)),
        );
    f.render_widget(footer, area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

