use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::{InstallState, WizardStep};
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
            Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
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
            format!("{current_step_idx}/{}: ", WizardStep::ALL.len()),
            Style::default()
                .fg(THEME.amber)
                .add_modifier(Modifier::BOLD),
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
            .title(" 󰇊 Navigation [1-6] ")
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
            Span::styled("\u{25c6} Components: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_bins}/{total_bins}"),
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("\u{25c6} Variant:    ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                selected_variant,
                Style::default().fg(THEME.pink).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("\u{25c6} Mode:       ", Style::default().fg(THEME.text_dim)),
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
            Span::styled("\u{25c6} Automatic:  ", Style::default().fg(THEME.text_dim)),
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

pub fn draw_footer_shortcuts(f: &mut Frame, app: &App, area: Rect) {
    if area.height < 1 || area.width < 20 {
        return;
    }

    let (step_shortcuts, title) = match app.current_step {
        WizardStep::Welcome => (
            vec![
                ("Enter / →", "Get Started", THEME.mint),
                ("Tab / n", "Next Step", THEME.cyan),
                ("1-6", "Jump Step", THEME.purple),
            ],
            " 󰌌 Shortcuts (Welcome) ",
        ),
        WizardStep::SourceBranch => (
            vec![
                ("Space", "Select", THEME.mint),
                ("Enter", "Switch & Next", THEME.cyan),
                ("↑ / ↓", "Move", THEME.blue),
                ("← / →", "Step", THEME.amber),
            ],
            " 󰌌 Shortcuts (Branch) ",
        ),
        WizardStep::Binaries => (
            vec![
                ("Space", "Toggle", THEME.mint),
                ("a", "Toggle All", THEME.cyan),
                ("Enter / →", "Next Step", THEME.mint),
                ("←", "Back", THEME.amber),
                ("s", "Binary Path", THEME.purple),
                ("r", "Rescan", THEME.purple),
            ],
            " 󰌌 Shortcuts (Binaries) ",
        ),
        WizardStep::VariantSelection => (
            vec![
                ("Space", "Select", THEME.mint),
                ("Enter / →", "Apply & Next", THEME.cyan),
                ("↑ / ↓", "Move", THEME.blue),
                ("←", "Back", THEME.amber),
            ],
            " 󰌌 Shortcuts (Themes) ",
        ),
        WizardStep::ExecuteInstall => (
            if app.show_confirm_dialog {
                vec![
                    ("Enter", "Start Installation", THEME.mint),
                    ("Esc / ←", "Back", THEME.amber),
                ]
            } else if app.install_state == InstallState::Installing {
                vec![
                    ("↑ / ↓", "Scroll", THEME.blue),
                    ("PgUp/Dn", "Fast", THEME.blue),
                    ("c", "Clear", THEME.amber),
                    ("g / G", "Top/End", THEME.purple),
                ]
            } else {
                vec![
                    ("Enter / i", "Start Install", THEME.mint),
                    ("← / p", "Back to Setup", THEME.amber),
                ]
            },
            " 󰌌 Shortcuts (Install) ",
        ),
        WizardStep::Summary => (
            vec![
                ("Enter / q", "Finish & Exit", THEME.mint),
                ("←", "Back", THEME.amber),
                ("1-5", "Review Step", THEME.purple),
            ],
            " 󰌌 Shortcuts (Summary) ",
        ),
    };

    if area.width < 75 {
        // Compact single-block footer
        let mut spans = Vec::new();
        spans.push(Span::raw(" "));
        for (i, (key, desc, col)) in step_shortcuts.into_iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" ", Style::default()));
            }
            spans.push(Span::styled(
                format!(" {} ", key),
                Style::default()
                    .fg(col)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {} ", desc),
                Style::default().fg(THEME.text_body),
            ));
        }

        let p = Paragraph::new(Line::from(spans)).block(
            Block::default()
                .title(title)
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.cyan)),
        );
        f.render_widget(p, area);
    } else {
        // Dual block footer (Context Shortcuts on Left, Global on Right)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(35), Constraint::Length(34)])
            .split(area);

        let mut left_spans = Vec::new();
        left_spans.push(Span::raw(" "));
        for (i, (key, desc, col)) in step_shortcuts.into_iter().enumerate() {
            if i > 0 {
                left_spans.push(Span::styled(" ", Style::default()));
            }
            left_spans.push(Span::styled(
                format!(" {} ", key),
                Style::default()
                    .fg(col)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ));
            left_spans.push(Span::styled(
                format!(" {} ", desc),
                Style::default().fg(THEME.text_body),
            ));
        }

        let left_para = Paragraph::new(Line::from(left_spans)).block(
            Block::default()
                .title(title)
                .title_style(THEME.title_cyan())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.cyan)),
        );
        f.render_widget(left_para, chunks[0]);

        let right_spans = vec![
            Span::styled(
                " ? ",
                Style::default()
                    .fg(THEME.purple)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Help ", Style::default().fg(THEME.text_body)),
            Span::styled("│ ", Style::default().fg(THEME.border_normal)),
            Span::styled(
                " q ",
                Style::default()
                    .fg(THEME.rose)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Quit ", Style::default().fg(THEME.text_body)),
            Span::styled("│ ", Style::default().fg(THEME.border_normal)),
            Span::styled(
                " 1-6 ",
                Style::default()
                    .fg(THEME.amber)
                    .bg(THEME.bg_badge)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" Jump ", Style::default().fg(THEME.text_body)),
        ];

        let right_para = Paragraph::new(Line::from(right_spans))
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    .title(" Global ")
                    .title_style(THEME.title_amber())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(THEME.border_normal)),
            );
        f.render_widget(right_para, chunks[1]);
    }
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

/// Computes a centered rectangle with exact character dimensions, clamped
/// within the parent area so it never overflows small terminals.
pub fn centered_rect_exact(width: u16, height: u16, r: Rect) -> Rect {
    let w = width.min(r.width.saturating_sub(2));
    let h = height.min(r.height.saturating_sub(2));
    let x = r.x + (r.width.saturating_sub(w)) / 2;
    let y = r.y + (r.height.saturating_sub(h)) / 2;
    Rect::new(x, y, w, h)
}
