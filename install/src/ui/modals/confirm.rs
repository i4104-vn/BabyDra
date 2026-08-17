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
    let selected_varlib = app.varlib_options.iter().filter(|o| o.selected).count();
    let selected_cfgs = app
        .configs_themes_options
        .iter()
        .filter(|o| o.selected)
        .count();
    let selected_pkgs = app.package_options.iter().filter(|o| o.selected).count();

    let lines = vec![
        Line::from(Span::styled(
            "Ready to Execute BabyDra Installation Plan",
            THEME.title_cyan(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("◆ System Packages:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_pkgs} selected (pacman, AUR yay, permissions)"),
                Style::default().fg(THEME.amber).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Binary Binaries:  ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_bins} selected (Deploy to ~/.local/bin)"),
                Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ /var/lib Staging: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_varlib} tasks enabled (/var/lib/babydra)"),
                Style::default().fg(THEME.purple).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("◆ Configs & Themes: ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_cfgs} dotfiles & themes enabled"),
                Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            if app.is_build_from_source() {
                format!("Source: Branch '{}' will be compiled with cargo --release.", app.selected_branch)
            } else {
                "Source: Pre-built binaries will be copied directly.".to_string()
            },
            Style::default().fg(THEME.text_dim),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(" y / Enter ", THEME.key_badge_green()),
            Span::styled(" Start Installation   ", Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD)),
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
