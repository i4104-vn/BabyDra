use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::App;
use crate::system::is_root;
use crate::ui::layout::centered_rect_exact;
use crate::ui::THEME;

pub fn draw_confirm_modal(f: &mut Frame, app: &App, area: Rect) {
    let popup_area = centered_rect_exact(76, 17, area);
    f.render_widget(Clear, popup_area);

    let selected_bins = app
        .binaries
        .iter()
        .filter(|b| b.selected && (b.exists_in_source || app.is_build_from_source()))
        .count();

    let variant_display = if app.selected_variant.is_empty() {
        "Default".to_string()
    } else {
        app.selected_variant.clone()
    };

    let source_display = if app.is_build_from_source() {
        format!("Branch '{}' (cargo build --release)", app.selected_branch)
    } else {
        "Pre-built binaries (direct copy)".to_string()
    };

    let priv_display = if is_root() {
        ("Root privileges active (direct install)", THEME.mint)
    } else {
        ("Standard user (sudo prompt will follow)", THEME.amber)
    };

    let lines = vec![
        Line::from(vec![
            Span::styled(
                "  🚀 Ready to Execute BabyDra Installation",
                THEME.title_cyan(),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Review your chosen configuration before starting the deployment:",
                Style::default().fg(THEME.text_dim),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ◆ Source Branch : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                source_display,
                Style::default().fg(THEME.cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ◆ Theme Variant : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                variant_display,
                Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ◆ Components    : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                format!("{selected_bins} crates selected (deploy ~/.local/bin & /usr/bin)"),
                Style::default()
                    .fg(THEME.text_bright)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ◆ System Tasks  : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                "Pacman packages, /var/lib staging, configs, greetd, services",
                Style::default()
                    .fg(THEME.purple)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("  ◆ Privileges    : ", Style::default().fg(THEME.text_dim)),
            Span::styled(
                priv_display.0,
                Style::default()
                    .fg(priv_display.1)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "  Would you like to start installing BabyDra now?",
                Style::default()
                    .fg(THEME.amber)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(" y / Enter ", THEME.key_badge_green()),
            Span::styled(
                " Start Installation   ",
                Style::default().fg(THEME.mint).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" n / Esc ", THEME.key_badge_red()),
            Span::styled(" Stay / Logs   ", Style::default().fg(THEME.rose)),
            Span::styled(" ← / Back ", THEME.key_badge_amber()),
            Span::styled(" Review Setup", Style::default().fg(THEME.amber)),
        ]),
    ];

    let block = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" ⚡ Confirmation ")
                .title_style(THEME.title_mint())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(THEME.mint)),
        )
        .alignment(Alignment::Left);

    f.render_widget(block, popup_area);
}
