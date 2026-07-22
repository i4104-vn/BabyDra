use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::services::explore::execute_custom_command;
use crate::explore::context_menu::widgets::create_menu_button;

/// Appends user-defined custom context menu items to the vbox.
pub fn append_custom_context_items(
    vbox: &gtk4::Box,
    popover: &gtk4::Popover,
    target_paths: Vec<PathBuf>,
    is_dir_context: bool,
) {
    let settings = babydra_common::load_explore_settings();
    if settings.custom_context_items.is_empty() {
        return;
    }

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("menu-sep");
    vbox.append(&sep);

    for item in settings.custom_context_items {
        let icon_key = item.icon.as_deref().unwrap_or_else(|| {
            if item.name.to_lowercase().contains("terminal") {
                "terminal"
            } else {
                "settings"
            }
        });
        let btn_custom = create_menu_button(&item.name, icon_key);
        vbox.append(&btn_custom);

        let pop_c = popover.clone();
        let command_tmpl = item.command.clone();
        let target_paths_c = target_paths.clone();
        btn_custom.connect_clicked(move |_| {
            pop_c.popdown();
            let command_tmpl_c = command_tmpl.clone();
            let paths = target_paths_c.clone();
            for path in paths {
                let path_str = path.to_string_lossy().to_string();
                let parent_str = if is_dir_context {
                    path_str.clone()
                } else {
                    path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()
                };
                let name_str = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                let stem_str = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                let ext_str = path.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();

                let cmd_str = command_tmpl_c
                    .replace("{path}", &path_str)
                    .replace("{dir}", &parent_str)
                    .replace("{name}", &name_str)
                    .replace("{stem}", &stem_str)
                    .replace("{ext}", &ext_str);
                
                let _ = execute_custom_command(&cmd_str);
            }
        });
    }
}
