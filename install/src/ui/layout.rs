use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::WizardStep;
use crate::system::is_root;

pub fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let user_name = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let root_badge = if is_root() {
        Span::styled(" [ROOT] ", Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(" [USER] ", Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD))
    };

    let title_line = Line::from(vec![
        Span::styled(" BabyDra ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled("Desktop Shell Installer ", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled("v1.0.0", Style::default().fg(Color::DarkGray)),
    ]);

    let left_header = Paragraph::new(title_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(left_header, header_chunks[0]);

    let right_line = Line::from(vec![
        root_badge,
        Span::raw(" "),
        Span::styled(format!("{user_name}@ArchLinux "), Style::default().fg(Color::White)),
        Span::styled("│ Step: ", Style::default().fg(Color::DarkGray)),
        Span::styled(app.current_step.short_name(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]);

    let right_header = Paragraph::new(right_line)
        .alignment(Alignment::Right)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(right_header, header_chunks[1]);
}

pub fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(8)])
        .split(area);

    let current_step_idx = app.current_step as usize;

    let items: Vec<ListItem> = WizardStep::ALL
        .iter()
        .map(|step| {
            let step_idx = *step as usize;
            let is_current = *step == app.current_step;

            let (icon, style) = if is_current {
                (
                    "►▶ ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
                )
            } else if step_idx < current_step_idx {
                (" ● ", Style::default().fg(Color::Green))
            } else {
                (" ○ ", Style::default().fg(Color::DarkGray))
            };

            let row_style = if is_current {
                Style::default().bg(Color::Rgb(25, 30, 48))
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(icon, style),
                Span::styled(step.title(), style),
            ]))
            .style(row_style)
        })
        .collect();

    let steps_list = List::new(items).block(
        Block::default()
            .title(" Steps [1-8] ")
            .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(steps_list, sidebar_chunks[0]);

    let selected_bins = app.binaries.iter().filter(|b| b.selected && b.exists_in_source).count();
    let total_bins = app.binaries.len();
    let selected_varlib = app.varlib_options.iter().filter(|o| o.selected).count();
    let selected_cfgs = app.configs_themes_options.iter().filter(|o| o.selected).count();
    let selected_dm = app.display_manager_options.iter().filter(|o| o.selected).count();

    let summary_lines = vec![
        Line::from(vec![
            Span::styled("• Binaries: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{selected_bins}/{total_bins}"), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("• /var/lib: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{selected_varlib} staged"), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::styled("• Configs:  ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{selected_cfgs} enabled"), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("• Greetd DM:", Style::default().fg(Color::DarkGray)),
            Span::styled(if selected_dm > 0 { " Enabled" } else { " Skipped" }, Style::default().fg(if selected_dm > 0 { Color::Cyan } else { Color::DarkGray })),
        ]),
    ];

    let summary_box = Paragraph::new(summary_lines).block(
        Block::default()
            .title(" Plan Summary ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray)),
    );
    f.render_widget(summary_box, sidebar_chunks[1]);
}

pub fn draw_footer(f: &mut Frame, _app: &App, area: Rect) {
    let key_hints = vec![
        Span::styled(" [Tab/n] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Next  "),
        Span::styled(" [p] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Prev  "),
        Span::styled(" [↑/↓|j/k] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Nav  "),
        Span::styled(" [Space] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("Toggle  "),
        Span::styled(" [a] ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("All  "),
        Span::styled(" [i/Enter] ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled("Install  ", Style::default().fg(Color::Green)),
        Span::styled(" [s] ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("Source  "),
        Span::styled(" [?] ", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::raw("Help  "),
        Span::styled(" [q] ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        Span::raw("Quit "),
    ];

    let footer = Paragraph::new(Line::from(key_hints))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
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
