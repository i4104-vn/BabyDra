//! Native Arch Linux settings manager built with GTK4 + Rust.

use gtk4::prelude::*;
use babydra_common::{
    PerformanceProfile, get_current_profile, set_performance_profile,
    get_battery_info, load_babydra_config, save_babydra_config,
    battery::check_and_apply_auto_battery_saver,
};

mod layout;
mod widgets;

fn handle_cli_args() -> bool {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return false;
    }

    match args[1].as_str() {
        "--apply-battery-saver" => {
            let conf = load_babydra_config();
            if conf.power.auto_saver_enabled {
                let cur_profile = get_current_profile();
                if cur_profile != PerformanceProfile::Normal {
                    if set_performance_profile(PerformanceProfile::Normal).is_ok() {
                        let mut updated_conf = load_babydra_config();
                        updated_conf.power.profile = PerformanceProfile::Normal.key().to_string();
                        save_babydra_config(&updated_conf);

                        let bat_pct = get_battery_info().map(|b| b.percentage).unwrap_or(conf.power.saver_threshold);
                        let title = babydra_common::i18n::t("settings.notif_auto_saver_title");
                        let msg = babydra_common::i18n::t("settings.notif_auto_saver_msg").replace("{level}", &bat_pct.to_string());
                        babydra_common::send_settings_notification(&title, &msg);
                    }
                }
            }
            true
        }
        "--check-battery-saver" => {
            if let Some(info) = get_battery_info() {
                check_and_apply_auto_battery_saver(&info);
            }
            true
        }
        "--set-power-profile" => {
            if let Some(key) = args.get(2) {
                let prof = PerformanceProfile::from_key(key);
                if set_performance_profile(prof).is_ok() {
                    let mut updated_conf = load_babydra_config();
                    updated_conf.power.profile = prof.key().to_string();
                    save_babydra_config(&updated_conf);

                    let title = babydra_common::i18n::t("settings.notif_power_title");
                    let msg = babydra_common::i18n::t("settings.notif_power_msg").replace("{profile}", prof.label());
                    babydra_common::send_settings_notification(&title, &msg);
                }
            } else {
                println!("Usage: babydra-settings --set-power-profile <normal|balanced|performance>");
            }
            true
        }
        "--run-background-update" => {
            use std::io::BufRead;
            let mut pwd = String::new();
            let stdin = std::io::stdin();
            let _ = stdin.lock().read_line(&mut pwd);
            let pwd_trimmed = pwd.trim();
            let pwd_opt = if pwd_trimmed.is_empty() { None } else { Some(pwd_trimmed) };

            babydra_common::services::system::updates::run_background_update_loop(pwd_opt);
            true
        }
        "--help" | "-h" => {
            println!("BabyDra Settings CLI Options:");
            println!("  --apply-battery-saver         Switch to battery saver profile if auto saver is enabled");
            println!("  --check-battery-saver         Check system battery and apply saver if below threshold");
            println!("  --set-power-profile <profile> Set CPU performance profile (normal, balanced, performance)");
            println!("  --run-background-update       Run sequential updates in detached background process");
            true
        }
        _ => false,
    }
}

fn main() {
    if handle_cli_args() {
        return;
    }

    let app = gtk4::Application::new(
        Some("com.babydra.settings"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        // Load custom styles
        babydra_utils::ui::theme::init_theme();

        // Build and display main settings window
        layout::build_main_window(app);
    });

    app.run_with_args(&["babydra-settings"]);
}
