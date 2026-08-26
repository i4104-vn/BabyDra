use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ui::layout::centered_rect;
use crate::ui::THEME;

pub fn draw_confirm_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect(62, 34, area);
    f.render_widget(Clear, popup_area);

    let selected_bins = app
        .binaries
        .iter()
        .filter(|b| b.selected && (b.exists_in_source || app.is_build_from_source()))
        .count();

    let lines = vec![
        Line::from(Span::styled(
            "Ready to Execute BabyDra Installation Plan",
            THEME.title_cyan(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("◆ Components:       ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_bins} selected (deploy to ~/.local/bin, /usr/bin)"),
                Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Automatic tasks:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "system packages, /var/lib staging, configs & themes, greetd",
                Style::default()
                    .fg(THEME.amber)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            if app.is_build_from_source() {
                format!(
                    "Source: Branch '{}' will be compiled with cargo --release.",
                    app.selected_branch
                )
            } else {
                "Source: Pre-built binaries will be copied directly.".to_string()
            },
            Style::default().fg(THEME.text_dim),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" y / Enter ", THEME.key_badge_green()),
            Span::styled(
                " Start Installation   ",
                Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" n / Esc ", THEME.key_badge_red()),
            Span::styled(" Cancel", Style::default().fg(THEME.rose)),
        ]),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Confirmation ")
                .title_style(THEME.title_mint())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.mint)),
        )
        .alignment(Alignment::Center);

    f.render_widget(block, popup_area);
}
