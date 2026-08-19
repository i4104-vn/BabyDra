use babydra_core::services::apps::DesktopApp;
use gtk4::prelude::*;
use std::path::Path;
use std::rc::Rc;

mod render;

/// Launches an application with a given file path.
pub fn launch_app_with_file(app: &DesktopApp, path: &Path) {
    if let Some(desktop_file) = &app.file_path {
        if let Some(app_info) = gtk4::gio::DesktopAppInfo::from_filename(desktop_file) {
            let uri = format!("file://{}", path.to_string_lossy());
            if app_info
                .launch_uris(&[&uri], gtk4::gio::AppLaunchContext::NONE)
                .is_ok()
            {
                return;
            }
        }
    }

    let clean_exec = app
        .exec
        .split_whitespace()
        .filter(|w| !w.starts_with('%'))
        .collect::<Vec<&str>>()
        .join(" ");

    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("{} \"{}\" &", clean_exec, path.to_string_lossy()))
        .spawn();
}

/// Sets a desktop application as the default handler for a file's MIME type.
pub fn set_default_app_for_file(app: &DesktopApp, path: &Path) {
    let (content_type, _) = gtk4::gio::content_type_guess(Some(path), &[]);
    let mime_type = gtk4::gio::content_type_get_mime_type(&content_type)
        .map(|s| s.to_string())
        .unwrap_or_else(|| content_type.to_string());

    if let Some(desktop_file) = &app.file_path {
        let desktop_name = desktop_file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();

        if let Some(app_info) = gtk4::gio::DesktopAppInfo::from_filename(desktop_file) {
            let _ = app_info.set_as_default_for_type(&mime_type);
        }

        let _ = std::process::Command::new("xdg-mime")
            .arg("default")
            .arg(&*desktop_name)
            .arg(&mime_type)
            .spawn();
    }
}

/// Presents the Open With application chooser dialog for a file.
pub fn show_open_with_dialog(path: &Path, parent: Option<&impl IsA<gtk4::Window>>) {
    let widgets = render::build_open_with_dialog(path, parent);
    let window = widgets.window;
    let search_entry = widgets.search_entry;
    let listbox = widgets.listbox;
    let check_always = widgets.check_always;
    let apps = Rc::new(widgets.apps);

    // Escape key to close
    let key_controller = gtk4::EventControllerKey::new();
    let win_esc = window.clone();
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            win_esc.close();
            glib::Propagation::Stop
        } else {
            glib::Propagation::Proceed
        }
    });
    window.add_controller(key_controller);

    // Real-time search filter
    let apps_filter = apps.clone();
    let search_entry_for_filter = search_entry.clone();
    listbox.set_filter_func(move |row| {
        let text = search_entry_for_filter.text().to_lowercase();
        if text.is_empty() {
            return true;
        }

        let idx = row.index() as usize;
        if let Some(app) = apps_filter.get(idx) {
            app.name.to_lowercase().contains(&text) || app.exec.to_lowercase().contains(&text)
        } else {
            true
        }
    });

    let lb_search = listbox.clone();
    search_entry.connect_search_changed(move |_| {
        lb_search.invalidate_filter();
    });

    // Clicking / activating an app in the list immediately opens it
    let window_c = window.clone();
    let path_buf = path.to_path_buf();
    let apps_c = apps.clone();
    let check_always_c = check_always.clone();

    listbox.connect_row_activated(move |_, row| {
        let idx = row.index() as usize;
        if let Some(app) = apps_c.get(idx) {
            if check_always_c.is_active() {
                set_default_app_for_file(app, &path_buf);
            }
            launch_app_with_file(app, &path_buf);
            window_c.close();
        }
    });

    // Enter in search bar activates first visible matching row
    let listbox_c = listbox.clone();
    let window_enter = window.clone();
    let path_enter = path.to_path_buf();
    let apps_enter = apps.clone();
    let check_always_enter = check_always.clone();
    search_entry.connect_activate(move |_| {
        let mut row_opt = listbox_c.first_child();
        while let Some(widget) = row_opt {
            if let Ok(row) = widget.clone().downcast::<gtk4::ListBoxRow>() {
                if row.is_child_visible() {
                    let idx = row.index() as usize;
                    if let Some(app) = apps_enter.get(idx) {
                        if check_always_enter.is_active() {
                            set_default_app_for_file(app, &path_enter);
                        }
                        launch_app_with_file(app, &path_enter);
                        window_enter.close();
                        break;
                    }
                }
            }
            row_opt = widget.next_sibling();
        }
    });

    window.present();
    search_entry.grab_focus();
}

/// Attempts to launch a file using the system default handler, or opens the App Picker dialog if none exists.
pub fn launch_file_or_open_with(path: &Path, parent: Option<&impl IsA<gtk4::Window>>) {
    let (content_type, _) = gtk4::gio::content_type_guess(Some(path), &[]);
    let mime_type = gtk4::gio::content_type_get_mime_type(&content_type)
        .map(|s| s.to_string())
        .unwrap_or_else(|| content_type.to_string());

    let uri = format!("file://{}", path.to_string_lossy());

    if let Some(app_info) = gtk4::gio::AppInfo::default_for_type(&mime_type, false)
        .or_else(|| gtk4::gio::AppInfo::default_for_type(&content_type, false))
    {
        if app_info
            .launch_uris(&[&uri], gtk4::gio::AppLaunchContext::NONE)
            .is_ok()
        {
            return;
        }
    }

    if gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE).is_ok() {
        return;
    }

    if let Ok(_child) = std::process::Command::new("xdg-open")
        .arg(path.to_string_lossy().as_ref())
        .spawn()
    {
        return;
    }

    show_open_with_dialog(path, parent);
}
