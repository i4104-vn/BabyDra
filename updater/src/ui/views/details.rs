use crate::core::app::App;
use crate::core::state::ActionId;
use crate::ui::theme::*;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

pub fn render_action_details(frame: &mut Frame, area: Rect, app: &App) {
    let item = app.selected_action();

    let mut lines = Vec::new();
    lines.push(Line::from(""));

    // Title & Tag
    lines.push(Line::from(vec![
        Span::styled(
            format!(" {} ", item.icon),
            Style::default().fg(COLOR_ORANGE),
        ),
        Span::styled(
            item.title,
            Style::default()
                .fg(COLOR_TEXT_WHITE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            format!("[{}]", item.tag),
            Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(""));

    // Description
    lines.push(Line::from(vec![
        Span::styled("Description: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(item.description, Style::default().fg(COLOR_TEXT_WHITE)),
    ]));
    lines.push(Line::from(""));

    // Sudo badge
    if item.requires_sudo {
        lines.push(Line::from(vec![
            Span::styled(
                " 🔒 ROOT PRIVILEGES: ",
                Style::default()
                    .fg(COLOR_ORANGE)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "This action requires sudo for system-level changes.",
                Style::default().fg(COLOR_TEXT_MUTED),
            ),
        ]));
        lines.push(Line::from(""));
    }

    // Step-by-step detail
    lines.push(Line::from(Span::styled(
        "Execution Details:",
        Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
    )));

    match item.id {
        ActionId::UpdateReload => {
            lines.push(Line::from(
                "  1. Rebuild all workspace crates in release mode",
            ));
            lines.push(Line::from(
                "  2. Stop active running panel, desktop, switcher processes",
            ));
            lines.push(Line::from(
                "  3. Copy new binaries to ~/.local/bin and /usr/bin",
            ));
            lines.push(Line::from(
                "  4. Install desktop application entries (.desktop files)",
            ));
            lines.push(Line::from(
                "  5. Sync configs (labwc, GTK, kitty, fastfetch, themes)",
            ));
            lines.push(Line::from("  6. Reconfigure labwc & start shell daemons"));
        }
        ActionId::SafetyCheck => {
            lines.push(Line::from("  1. Run 'cargo check --all-targets'"));
            lines.push(Line::from(
                "  2. Run 'cargo clippy --all-targets -- -D warnings'",
            ));
            lines.push(Line::from(
                "  3. Run 'cargo test' (using xvfb-run if headless)",
            ));
        }
        ActionId::StartDesktop => {
            lines.push(Line::from("  1. Stop stale shell instances"));
            lines.push(Line::from("  2. Prepare ~/.config/labwc and scripts"));
            lines.push(Line::from("  3. Check external display controls (ddcutil)"));
            lines.push(Line::from("  4. Launch labwc Wayland compositor"));
        }
        ActionId::FullInstall => {
            lines.push(Line::from(format!(
                "  1. Install {} official Arch Linux packages via pacman",
                app.config.packages.pacman.len()
            )));
            lines.push(Line::from(
                "  2. Configure kernel module (i2c-dev) and CPU governor",
            ));
            lines.push(Line::from("  3. Configure user input group and udev rules"));
            lines.push(Line::from(format!(
                "  4. Install {} AUR packages via yay",
                app.config.packages.yay.len()
            )));
            lines.push(Line::from("  5. Build wtype from source if missing"));
            lines.push(Line::from(
                "  6. Build and deploy all BabyDra binaries & configs",
            ));
            lines.push(Line::from(
                "  7. Setup greetd display manager (cage -- babydra-greeter)",
            ));
        }
        ActionId::ComponentRestart => {
            lines.push(Line::from(
                "  Select a single component to restart without a full reload:",
            ));
            lines.push(Line::from("  • babydra-panel (Top/Bottom Bar)"));
            lines.push(Line::from("  • babydra-desktop (Background & Icons)"));
            lines.push(Line::from("  • babydra-switcher (Alt-Tab Daemon)"));
            lines.push(Line::from("  • babydra-keymap (Keyboard layout daemon)"));
            lines.push(Line::from("  • Reconfigure labwc (labwc --reconfigure)"));
            lines.push(Line::from("  • Refresh GTK & font caches"));
        }
        ActionId::SyncConfigs => {
            lines.push(Line::from(
                "  1. Sync wallpapers and logo assets to ~/.babydra and /usr/share",
            ));
            lines.push(Line::from(
                "  2. Sync labwc configuration (autostart, rc.xml, scripts)",
            ));
            lines.push(Line::from(
                "  3. Sync GTK 3 & 4 settings.ini and fontconfig fonts.conf",
            ));
            lines.push(Line::from(
                "  4. Sync Kitty terminal, Neovim, and Fastfetch configs",
            ));
            lines.push(Line::from("  5. Sync BabyDra themes, cursors, and icons"));
            lines.push(Line::from("  6. Apply gsettings and refresh font cache"));
        }
        ActionId::CleanWorkspace => {
            lines.push(Line::from(
                "  1. Run 'cargo clean' to remove build cache in target/",
            ));
            lines.push(Line::from("  2. Reclaim disk space"));
        }
        ActionId::FactoryReset => {
            lines.push(Line::from("  1. Stop all BabyDra and compositor processes"));
            lines.push(Line::from(
                "  2. Restore standard Arch Linux TTY login console",
            ));
            lines.push(Line::from("  3. Remove system-wide files and modules"));
            lines.push(Line::from(
                "  4. Clean ~/.babydra, ~/.config/labwc, desktop entries",
            ));
            lines.push(Line::from(
                "  5. Optional: uninstall installed shell packages",
            ));
        }
        ActionId::Quit => {
            lines.push(Line::from("  Exits the updater terminal interface."));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Configuration Overview (updater.toml):",
        Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(vec![
        Span::styled(
            "  • Pacman Packages: ",
            Style::default().fg(COLOR_TEXT_MUTED),
        ),
        Span::styled(
            format!("{} items", app.config.packages.pacman.len()),
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
        Span::styled("   • AUR Packages: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(
            format!("{} items", app.config.packages.yay.len()),
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  • User Binaries: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(
            format!("{} binaries", app.config.binaries.user_binaries.len()),
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
        Span::styled("   • Daemons: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(
            format!("{} services", app.config.binaries.shell_daemons.len()),
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("  • UI Font: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(
            &app.config.gsettings.font_name,
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
        Span::styled("   • Icon Theme: ", Style::default().fg(COLOR_TEXT_MUTED)),
        Span::styled(
            &app.config.gsettings.icon_theme,
            Style::default().fg(COLOR_TEXT_WHITE),
        ),
    ]));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(COLOR_BORDER))
        .title(" ℹ️ Details & Configuration ")
        .title_style(Style::default().fg(COLOR_CYAN).add_modifier(Modifier::BOLD));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
