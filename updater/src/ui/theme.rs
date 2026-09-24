use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};

pub const COLOR_BG: Color = Color::Rgb(15, 17, 26);
#[allow(dead_code)]
pub const COLOR_PANEL_BG: Color = Color::Rgb(22, 25, 38);
pub const COLOR_CYAN: Color = Color::Rgb(56, 189, 248);
pub const COLOR_EMERALD: Color = Color::Rgb(52, 211, 153);
pub const COLOR_ORANGE: Color = Color::Rgb(249, 115, 22);
pub const COLOR_RED: Color = Color::Rgb(248, 113, 113);
pub const COLOR_PURPLE: Color = Color::Rgb(168, 85, 247);
pub const COLOR_TEXT_MUTED: Color = Color::Rgb(148, 163, 184);
pub const COLOR_TEXT_WHITE: Color = Color::Rgb(241, 245, 249);
pub const COLOR_BORDER: Color = Color::Rgb(51, 65, 85);

#[allow(dead_code)]
pub fn rounded_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER))
        .title(title)
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD))
}

#[allow(dead_code)]
pub fn active_rounded_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_CYAN))
        .title(title)
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD))
}
