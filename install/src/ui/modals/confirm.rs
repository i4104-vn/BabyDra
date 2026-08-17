use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::layout::centered_rect;

pub fn draw_confirm_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(60, 32, area);
    f.render_widget(Clear, popup_area);

    let selected_bins = app
        .binaries
        .iter()
        .filter(|b| b.selected && b.exists_in_source)
        .count();
    let selected_varlib = app.varlib_options.iter().filter(|o| o.selected).count();
    let selected_cfgs = app
        .configs_themes_options
        .iter()
        .filter(|o| o.selected)
        .count();

    let lines = vec![
        Line::from(Span::styled(
            "Confirm BabyDra Installation Plan",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Pre-built Binaries: ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{selected_bins} selected (Copy to ~/.local/bin)"),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "/var/lib Staging:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{selected_varlib} tasks enabled (/var/lib/babydra)"),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Configs & Themes:   ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{selected_cfgs} dotfiles & themes enabled"),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Binaries will be copied directly without source recompilation.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " [y / Enter] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Start Installation    "),
            Span::styled(
                " [n / Esc] ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Cancel"),
        ]),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Confirmation ")
                .title_style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Green)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
