use gtk4::prelude::*;
use babydra_common::desktop::DesktopApp;
use super::apps::{focus_window, close_window};
use super::render;

/// Populates a Popover widget containing a vertical list of window titles grouped by app
pub fn populate_popover_previews(
    popover: &gtk4::Popover,
    windows: &[DesktopApp],
    app_id: &str,
) {
    let (action_triggers, open_new_info, close_all_btn_opt) =
        render::render_popover_previews(popover, windows, app_id);

    // Set up click handlers for window items
    for (preview_btn, kill_btn, app) in action_triggers {
        let pop_close = popover.clone();
        let app_id_str = app.app_id.clone().unwrap_or_else(|| app_id.to_string());
        let title_str = app.window_title.clone().unwrap_or_default();

        let app_id_str_clone = app_id_str.clone();
        let title_str_clone = title_str.clone();
        preview_btn.connect_clicked(move |_| {
            focus_window(&app_id_str_clone, &title_str_clone);
            pop_close.popdown();
        });

        let pop_close_kill = popover.clone();
        let app_id_str_kill = app_id_str;
        let title_str_kill = title_str;
        kill_btn.connect_clicked(move |_| {
            close_window(&app_id_str_kill, &title_str_kill);
            pop_close_kill.popdown();
        });
    }

    // Set up click handler for open new button
    if let Some((open_new_btn, exec_cmd)) = open_new_info {
        let pop_open_new = popover.clone();
        open_new_btn.connect_clicked(move |_| {
            let parts: Vec<&str> = exec_cmd.split_whitespace().collect();
            if !parts.is_empty() {
                let program = parts[0];
                let args = &parts[1..];
                if let Err(e) = std::process::Command::new(program).args(args).spawn() {
                    eprintln!("Failed to spawn command {}: {}", exec_cmd, e);
                }
            }
            pop_open_new.popdown();
        });
    }

    // Set up click handler for close all button
    if let Some(close_all_btn) = close_all_btn_opt {
        let pop_close_all = popover.clone();
        let windows_clone: Vec<(String, String)> = windows
            .iter()
            .map(|app| {
                let app_id_str = app.app_id.as_deref().unwrap_or(app_id).to_string();
                let title_str = app.window_title.as_deref().unwrap_or("").to_string();
                (app_id_str, title_str)
            })
            .collect();

        close_all_btn.connect_clicked(move |_| {
            for (app_id_str, title_str) in &windows_clone {
                close_window(app_id_str, title_str);
            }
            pop_close_all.popdown();
        });
    }

    // Clear child on closed to free resources
    popover.connect_closed(|pop| {
        pop.set_child(None::<&gtk4::Widget>);
    });
}
