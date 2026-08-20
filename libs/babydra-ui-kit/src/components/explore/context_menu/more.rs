//! Extended "More..." submenu displaying actions provided by other applications.

use crate::components::context_menu::ContextMenuBuilder;
use babydra_core::i18n::trans;
use gtk4::gio::prelude::*;
use std::collections::HashSet;
use std::path::Path;

/// Queries system desktop applications registered or suitable for a given file or directory.
pub fn get_apps_for_path(path: &Path) -> Vec<gtk4::gio::AppInfo> {
    let (content_type, _) = gtk4::gio::content_type_guess(Some(path), &[]);
    let is_dir = path.is_dir();
    let mut result = Vec::new();
    let mut seen_ids = HashSet::new();

    let mut try_add = |app: gtk4::gio::AppInfo| {
        if app.should_show() {
            let id = app.id().map(|s| s.to_string()).unwrap_or_else(|| app.name().to_string());
            if seen_ids.insert(id) {
                result.push(app);
            }
        }
    };

    // 1. Primary registered handlers for this content type
    for app in gtk4::gio::AppInfo::all_for_type(&content_type) {
        try_add(app);
    }

    // 2. Generic text handlers for text-based file types
    if !is_dir && content_type != "text/plain" && gtk4::gio::content_type_is_a(&content_type, "text/plain") {
        for app in gtk4::gio::AppInfo::all_for_type("text/plain") {
            try_add(app);
        }
    }

    // 3. Other installed apps declaring matching supported types or directory arguments
    for app in gtk4::gio::AppInfo::all() {
        if !app.supports_files() && !app.supports_uris() {
            continue;
        }

        let supported = app.supported_types();
        let matches_type = supported.iter().any(|st| {
            st.as_str() == &content_type
                || gtk4::gio::content_type_is_a(&content_type, st)
                || gtk4::gio::content_type_is_a(st, &content_type)
        });
        let matches_dir = is_dir && (supported.is_empty() || supported.iter().any(|st| st == "inode/directory" || st.starts_with("text/")));

        if matches_type || matches_dir {
            try_add(app);
        }
    }

    result.sort_by(|a, b| a.name().to_lowercase().cmp(&b.name().to_lowercase()));
    result
}

/// Launches an application with the given path.
pub fn launch_app(app: &gtk4::gio::AppInfo, path: &Path) {
    let uri = format!("file://{}", path.to_string_lossy());
    if app.launch_uris(&[&uri], gtk4::gio::AppLaunchContext::NONE).is_err() {
        if let Some(cmd) = app.commandline() {
            let clean_cmd = cmd
                .to_string_lossy()
                .split_whitespace()
                .filter(|w| !w.starts_with('%'))
                .collect::<Vec<&str>>()
                .join(" ");
            let _ = std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("{} \"{}\" &", clean_cmd, path.to_string_lossy()))
                .spawn();
        }
    }
}

/// Appends the "More..." submenu containing registered external apps for `path` if any exist.
pub fn append_more_submenu(builder: ContextMenuBuilder, path: &Path) -> ContextMenuBuilder {
    let apps = get_apps_for_path(path);
    if apps.is_empty() {
        return builder;
    }

    let path_buf = path.to_path_buf();
    builder.submenu(&trans("desktop.more"), Some("apps"), move |mut sub| {
        for app in apps {
            let app_name = app.name().to_string();
            let app_c = app.clone();
            let p_c = path_buf.clone();
            let on_click = move || launch_app(&app_c, &p_c);

            let raw_icon = app
                .downcast_ref::<gtk4::gio::DesktopAppInfo>()
                .and_then(|d| d.string("Icon"))
                .map(|s| s.to_string());

            if let Some(ref icon_str) = raw_icon {
                sub = sub.item_with_icon_name(&app_name, icon_str, on_click);
            } else if let Some(icon) = app.icon() {
                sub = sub.item_with_gicon(&app_name, &icon, on_click);
            } else {
                sub = sub.item_with_icon_name(&app_name, "application-x-executable", on_click);
            }
        }
        sub
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_apps_for_directory() {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let apps = get_apps_for_path(Path::new(&home));
        assert!(!apps.is_empty());
        let names: Vec<String> = apps.iter().map(|a| a.name().to_string()).collect();
        assert!(names.iter().any(|n| n.contains("Explore") || n.contains("Studio") || n.contains("IDE") || n.contains("Dolphin")));
    }
}
