pub use crate::models::cli::{CliAction, CliOptions, PageId};
use crate::config::load_babydra_config;
use crate::models::system::PerformanceProfile;
use crate::services::config::pages::{is_valid_page, normalize_page_name};
use crate::services::system::battery::{apply_battery_saver, get_battery_info};
use crate::services::system::power::set_perf_profile;
use crate::services::system::updates;
use crate::services::wallpaper;
use crate::{get_current_profile, save_babydra_config};
use std::io::BufRead;

pub fn parse_cli_args(args: &[String]) -> (bool, CliOptions) {
    if args.len() < 2 {
        return (false, CliOptions { page: None, action: None });
    }

    let mut target_page: Option<PageId> = None;
    let mut action: Option<CliAction> = None;
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];

        match arg.as_str() {
            "--apply-battery-saver" => {
                action = Some(CliAction::ApplyBatterySaver);
            }
            "--check-battery-saver" => {
                action = Some(CliAction::CheckBatterySaver);
            }
            "--set-power-profile" => {
                if let Some(key) = args.get(i + 1) {
                    let prof = PerformanceProfile::from_key(key);
                    action = Some(CliAction::SetPowerProfile(prof));
                    i += 1;
                } else {
                    eprintln!("Usage: --set-power-profile <normal|balanced|performance>");
                    return (true, CliOptions { page: None, action: None });
                }
            }
            "--apply-all-settings" => {
                action = Some(CliAction::ApplyAllSettings);
            }
            "--apply-displays" | "--apply-display" => {
                action = Some(CliAction::ApplyDisplays);
            }
            "--sync-greeter-wallpaper" => {
                action = Some(CliAction::SyncGreeterWallpaper);
            }
            "--run-background-update" => {
                let mut pwd = String::new();
                let stdin = std::io::stdin();
                let _ = stdin.lock().read_line(&mut pwd);
                let pwd_trimmed = pwd.trim();
                let pwd_opt = if pwd_trimmed.is_empty() { None } else { Some(pwd_trimmed.to_string()) };
                action = Some(CliAction::RunBackgroundUpdate(pwd_opt));
            }
            "--help" | "-h" => {
                action = Some(CliAction::Help);
            }
            "--page" | "-p" | "--tab" => {
                if let Some(val) = args.get(i + 1) {
                    target_page = parse_page_name(val);
                    i += 1;
                }
            }
            arg if arg.starts_with("--page=") => {
                let val = arg.strip_prefix("--page=").unwrap();
                target_page = parse_page_name(val);
            }
            arg if arg.starts_with("--tab=") => {
                let val = arg.strip_prefix("--tab=").unwrap();
                target_page = parse_page_name(val);
            }
            arg if arg.starts_with("--") => {
                let flag = arg.strip_prefix("--").unwrap();
                let normalized = normalize_page_name(flag);
                if is_valid_page(&normalized) {
                    target_page = Some(PageId::from_str(&normalized));
                }
            }
            arg if !arg.starts_with('-') => {
                let normalized = normalize_page_name(arg);
                if is_valid_page(&normalized) {
                    target_page = Some(PageId::from_str(&normalized));
                }
            }
            _ => {}
        }
        i += 1;
    }

    let should_exit = action.is_some();
    (should_exit, CliOptions { page: target_page, action })
}

fn parse_page_name(name: &str) -> Option<PageId> {
    let normalized = normalize_page_name(name);
    if is_valid_page(&normalized) {
        Some(PageId::from_str(&normalized))
    } else {
        None
    }
}

pub fn execute_cli_action(action: CliAction) {
    match action {
        CliAction::ApplyBatterySaver => {
            let conf = load_babydra_config();
            if conf.power.auto_saver_enabled
                && get_current_profile() != PerformanceProfile::Normal
                && set_perf_profile(PerformanceProfile::Normal).is_ok()
            {
                let mut updated_conf = load_babydra_config();
                updated_conf.power.profile = PerformanceProfile::Normal.key().to_string();
                save_babydra_config(&updated_conf);

                let bat_pct = get_battery_info()
                    .map(|b| b.percentage)
                    .unwrap_or(conf.power.saver_threshold);
                let title = crate::i18n::trans("settings.notif_auto_saver_title");
                let msg = crate::i18n::trans("settings.notif_auto_saver_msg")
                    .replace("{level}", &bat_pct.to_string());
                crate::send_settings_notif(&title, &msg);
            }
        }
        CliAction::CheckBatterySaver => {
            if let Some(info) = get_battery_info() {
                apply_battery_saver(&info);
            }
        }
        CliAction::SetPowerProfile(prof) => {
            if set_perf_profile(prof).is_ok() {
                let mut updated_conf = load_babydra_config();
                updated_conf.power.profile = prof.key().to_string();
                save_babydra_config(&updated_conf);

                let title = crate::i18n::trans("settings.notif_power_title");
                let msg = crate::i18n::trans("settings.notif_power_msg")
                    .replace("{profile}", prof.label());
                crate::send_settings_notif(&title, &msg);
            }
        }
        CliAction::ApplyAllSettings => {
            println!("Applying all saved BabyDra system settings (CPU Profile, Displays, Wallpaper, Battery)...");
            crate::apply_saved_settings();
            println!("All saved settings applied successfully.");
        }
        CliAction::ApplyDisplays => {
            println!("Applying saved display settings...");
            crate::services::system::display::apply_saved_displays();
            println!("Saved display settings applied successfully.");
        }
        CliAction::SyncGreeterWallpaper => {
            println!("Syncing greeter wallpaper to world-readable system path...");
            wallpaper::apply_greeter_wp();
        }
        CliAction::RunBackgroundUpdate(pwd_opt) => {
            updates::run_bg_update_loop(pwd_opt.as_deref());
        }
        CliAction::Help => {
            print_help();
        }
    }
}

fn print_help() {
    println!("BabyDra Settings CLI Options:");
    println!("  --page, -p <page>             Open settings directly to specified tab");
    println!("  --wallpaper, --appearance     Open Wallpaper & Appearance tab");
    println!("  --displays, --display         Open Display Configuration tab");
    println!("  --wifi, --bluetooth, --vpn    Open Network configuration tabs");
    println!("  --power, --keybinds, --apps   Open System configuration tabs");
    println!("  --recovery, --reset           Open Factory Reset / Recovery tab");
    println!("  --apply-all-settings          Apply all saved system settings");
    println!("  --sync-greeter-wallpaper      Sync lock screen wallpaper");
    println!("  --apply-battery-saver         Switch to battery saver profile");
    println!("  --check-battery-saver         Check battery and apply auto-saver");
    println!("  --set-power-profile <profile> Set CPU profile (normal|balanced|performance)");
    println!("  --run-background-update       Run background update loop");
}