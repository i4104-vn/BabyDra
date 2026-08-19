//! Native Arch Linux settings manager built with GTK4 + Rust.

use babydra_core::{
    battery::apply_battery_saver, get_battery_info, get_current_profile, load_babydra_config,
    save_babydra_config, set_perf_profile, PerformanceProfile,
};
use gtk4::prelude::*;

mod layout;
mod widgets;

/// Parses CLI arguments and returns (should_exit, target_page).
fn handle_cli_args() -> (bool, Option<String>) {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return (false, None);
    }

    let mut target_page: Option<String> = None;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];

        if arg == "--apply-battery-saver" {
            let conf = load_babydra_config();
            if conf.power.auto_saver_enabled {
                let cur_profile = get_current_profile();
                if cur_profile != PerformanceProfile::Normal {
                    if set_perf_profile(PerformanceProfile::Normal).is_ok() {
                        let mut updated_conf = load_babydra_config();
                        updated_conf.power.profile = PerformanceProfile::Normal.key().to_string();
                        save_babydra_config(&updated_conf);

                        let bat_pct = get_battery_info()
                            .map(|b| b.percentage)
                            .unwrap_or(conf.power.saver_threshold);
                        let title = babydra_core::i18n::trans("settings.notif_auto_saver_title");
                        let msg = babydra_core::i18n::trans("settings.notif_auto_saver_msg")
                            .replace("{level}", &bat_pct.to_string());
                        babydra_core::send_settings_notif(&title, &msg);
                    }
                }
            }
            return (true, None);
        } else if arg == "--check-battery-saver" {
            if let Some(info) = get_battery_info() {
                apply_battery_saver(&info);
            }
            return (true, None);
        } else if arg == "--set-power-profile" {
            if let Some(key) = args.get(i + 1) {
                let prof = PerformanceProfile::from_key(key);
                if set_perf_profile(prof).is_ok() {
                    let mut updated_conf = load_babydra_config();
                    updated_conf.power.profile = prof.key().to_string();
                    save_babydra_config(&updated_conf);

                    let title = babydra_core::i18n::trans("settings.notif_power_title");
                    let msg = babydra_core::i18n::trans("settings.notif_power_msg")
                        .replace("{profile}", prof.label());
                    babydra_core::send_settings_notif(&title, &msg);
                }
            } else {
                println!(
                    "Usage: babydra-settings --set-power-profile <normal|balanced|performance>"
                );
            }
            return (true, None);
        } else if arg == "--apply-all-settings" {
            println!("Applying all saved BabyDra system settings (CPU Profile, Displays, Wallpaper, Battery)...");
            babydra_core::apply_saved_settings();
            println!("All saved settings applied successfully.");
            return (true, None);
        } else if arg == "--sync-greeter-wallpaper" {
            println!("Syncing greeter wallpaper to world-readable system path...");
            babydra_core::apply_greeter_wp();
            return (true, None);
        } else if arg == "--run-background-update" {
            use std::io::BufRead;
            let mut pwd = String::new();
            let stdin = std::io::stdin();
            let _ = stdin.lock().read_line(&mut pwd);
            let pwd_trimmed = pwd.trim();
            let pwd_opt = if pwd_trimmed.is_empty() {
                None
            } else {
                Some(pwd_trimmed)
            };

            babydra_core::services::system::updates::run_bg_update_loop(pwd_opt);
            return (true, None);
        } else if arg == "--help" || arg == "-h" {
            println!("BabyDra Settings CLI Options:");
            println!("  --page, -p <page>             Open settings directly to specified tab");
            println!("  --wallpaper, --appearance     Open Wallpaper & Appearance tab");
            println!("  --displays, --display         Open Display Configuration tab");
            println!("  --wifi, --bluetooth, --vpn    Open Network configuration tabs");
            println!("  --power, --keybinds, --apps   Open System configuration tabs");
            println!("  --apply-all-settings          Apply all saved system settings");
            println!("  --sync-greeter-wallpaper      Sync lock screen wallpaper");
            println!("  --apply-battery-saver         Switch to battery saver profile");
            println!("  --check-battery-saver         Check battery and apply auto-saver");
            println!(
                "  --set-power-profile <profile> Set CPU profile (normal|balanced|performance)"
            );
            println!("  --run-background-update       Run background update loop");
            return (true, None);
        } else if arg == "--page" || arg == "-p" || arg == "--tab" {
            if let Some(val) = args.get(i + 1) {
                target_page = Some(normalize_page_name(val));
                i += 1;
            }
        } else if arg.starts_with("--page=") {
            let val = &arg["--page=".len()..];
            target_page = Some(normalize_page_name(val));
        } else if arg.starts_with("--tab=") {
            let val = &arg["--tab=".len()..];
            target_page = Some(normalize_page_name(val));
        } else if arg.starts_with("--") {
            let flag = &arg[2..];
            let normalized = normalize_page_name(flag);
            if normalized != flag || is_valid_page(&normalized) {
                target_page = Some(normalized);
            }
        } else if !arg.starts_with('-') {
            let normalized = normalize_page_name(arg);
            if is_valid_page(&normalized) {
                target_page = Some(normalized);
            }
        }

        i += 1;
    }

    (false, target_page)
}

/// Normalizes page name aliases to standard internal page IDs.
fn normalize_page_name(name: &str) -> String {
    match name.trim().to_lowercase().as_str() {
        "wallpaper" | "wallpapers" | "theme" | "themes" | "appearance" => "appearance".to_string(),
        "display" | "displays" | "screen" | "screens" | "monitor" | "monitors" => {
            "displays".to_string()
        }
        "wifi" | "wlan" | "network" | "networks" => "wifi".to_string(),
        "bluetooth" | "bt" => "bluetooth".to_string(),
        "vpn" | "shield" => "vpn".to_string(),
        "power" | "battery" | "energy" => "power".to_string(),
        "keybind" | "keybinds" | "shortcuts" | "keys" => "keybinds".to_string(),
        "startup" | "autostart" | "startup_apps" => "startup".to_string(),
        "app" | "apps" | "applications" | "installed" => "apps".to_string(),
        "env" | "environment" | "vars" => "env".to_string(),
        "cert" | "certs" | "certificates" | "ssl" => "certificates".to_string(),
        "host" | "hosts" => "hosts".to_string(),
        "system_update" | "update" | "updates" => "system_update".to_string(),
        "system" | "about" | "info" => "system".to_string(),
        other => other.to_string(),
    }
}

/// Checks if a page identifier is valid.
fn is_valid_page(name: &str) -> bool {
    matches!(
        name,
        "wifi"
            | "bluetooth"
            | "vpn"
            | "certificates"
            | "hosts"
            | "displays"
            | "appearance"
            | "power"
            | "startup"
            | "apps"
            | "env"
            | "keybinds"
            | "system_update"
            | "system"
    )
}

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-settings", "babydra-settings.log");

    let (should_exit, target_page) = handle_cli_args();
    if should_exit {
        return;
    }

    let app = gtk4::Application::new(
        Some("com.babydra.settings"),
        gtk4::gio::ApplicationFlags::NON_UNIQUE,
    );

    let initial_page = target_page;
    app.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();

        layout::build_main_window(app, initial_page.as_deref());
    });

    app.run_with_args(&["babydra-settings"]);
}
