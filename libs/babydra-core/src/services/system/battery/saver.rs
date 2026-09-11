//! Battery auto saver mode management.

use crate::models::system::battery::BatteryInfo;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static LAST_SAVER_CHECK: AtomicU64 = AtomicU64::new(0);

/// Check and apply auto battery saver when capacity drops below configured threshold.
pub fn apply_battery_saver(battery_info: &BatteryInfo) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let last = LAST_SAVER_CHECK.load(Ordering::Relaxed);
    if now - last < 60 {
        return;
    }
    LAST_SAVER_CHECK.store(now, Ordering::Relaxed);

    if battery_info.is_ac_only || battery_info.is_charging {
        return;
    }
    let conf = crate::config::load_babydra_config();
    if !conf.power.auto_saver_enabled {
        return;
    }
    if battery_info.percentage <= conf.power.saver_threshold {
        let cur_profile = crate::services::system::power::profile::get_current_profile();
        if cur_profile != crate::PerformanceProfile::Normal {
            if crate::services::system::power::profile::set_perf_profile(
                crate::PerformanceProfile::Normal,
            )
            .is_ok()
            {
                let title = crate::i18n::trans("settings.notif_auto_saver_title");
                let msg = crate::i18n::trans("settings.notif_auto_saver_msg")
                    .replace("{level}", &battery_info.percentage.to_string());
                crate::send_notification(&title, &msg);

                // Reduce screen brightness by 50% when auto saver mode activates
                let cur_b = crate::services::system::backlight::get_brightness();
                let target_b = (cur_b * 0.5).max(10.0);
                crate::services::system::backlight::set_brightness(target_b);
            }
        }
    }
}
