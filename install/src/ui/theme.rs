use ratatui::style::{Color, Modifier, Style};

/// Centralized modern design system tokens for BabyDra TUI Installer.
/// Inspired by modern dark minimalist & cyberpunk aesthetic palettes.
pub struct Theme {
    // Primary Accents
    pub cyan: Color,
    pub blue: Color,
    pub purple: Color,
    pub pink: Color,
    pub mint: Color,
    pub amber: Color,
    pub rose: Color,

    // Typography Shades
    pub text_bright: Color,
    pub text_body: Color,
    pub text_dim: Color,
    pub text_muted: Color,

    // Borders & Dividers
    pub border_normal: Color,
    pub border_active: Color,
    pub border_focus: Color,

    // Backgrounds & Cards
    pub bg_dark: Color,
    pub bg_card: Color,
    pub bg_selected: Color,
    pub bg_cursor: Color,
    pub bg_badge: Color,
}

pub const THEME: Theme = Theme {
    cyan: Color::Rgb(100, 220, 255),       // #64DCFF - Electric Cyan
    blue: Color::Rgb(130, 185, 255),       // #82B9FF - Sky Azure
    purple: Color::Rgb(195, 160, 255),     // #C3A0FF - Lavender Violet
    pink: Color::Rgb(245, 140, 210),       // #F58CD2 - Sakura Magenta
    mint: Color::Rgb(135, 235, 175),       // #87EBAF - Mint Green
    amber: Color::Rgb(255, 200, 115),      // #FFC873 - Warm Amber
    rose: Color::Rgb(255, 125, 145),       // #FF7D91 - Coral Rose

    text_bright: Color::Rgb(252, 252, 255),// #FCFCFF - Crisp White
    text_body: Color::Rgb(220, 226, 245),  // #DCE2F5 - Soft Body
    text_dim: Color::Rgb(150, 160, 190),   // #96A0BE - Slate Dim
    text_muted: Color::Rgb(95, 105, 135),  // #5F6987 - Charcoal Muted

    border_normal: Color::Rgb(65, 75, 105),// #414B69 - Normal Border
    border_active: Color::Rgb(100, 220, 255),// #64DCFF - Active Border
    border_focus: Color::Rgb(195, 160, 255),// #C3A0FF - Focus Border

    bg_dark: Color::Rgb(17, 18, 28),       // #11121C - Deep Canvas
    bg_card: Color::Rgb(23, 26, 39),       // #171A27 - Card Surface
    bg_selected: Color::Rgb(34, 44, 72),   // #222C48 - Selected Item
    bg_cursor: Color::Rgb(42, 54, 88),     // #2A3658 - Focused/Hovered
    bg_badge: Color::Rgb(46, 56, 90),      // #2E385A - Badge Pill
};

impl Theme {
    pub fn title_cyan(&self) -> Style {
        Style::default().fg(self.cyan).add_modifier(Modifier::BOLD)
    }

    pub fn title_purple(&self) -> Style {
        Style::default().fg(self.purple).add_modifier(Modifier::BOLD)
    }

    pub fn title_amber(&self) -> Style {
        Style::default().fg(self.amber).add_modifier(Modifier::BOLD)
    }

    pub fn title_mint(&self) -> Style {
        Style::default().fg(self.mint).add_modifier(Modifier::BOLD)
    }

    pub fn key_badge(&self) -> Style {
        Style::default()
            .fg(self.cyan)
            .bg(self.bg_badge)
            .add_modifier(Modifier::BOLD)
    }

    pub fn key_badge_green(&self) -> Style {
        Style::default()
            .fg(self.mint)
            .bg(self.bg_badge)
            .add_modifier(Modifier::BOLD)
    }

    pub fn key_badge_red(&self) -> Style {
        Style::default()
            .fg(self.rose)
            .bg(self.bg_badge)
            .add_modifier(Modifier::BOLD)
    }
}
