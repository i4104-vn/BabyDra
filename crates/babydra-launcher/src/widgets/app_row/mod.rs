//! Controller wrappers for individual launcher application row and grid item buttons.

use babydra_core::DesktopApp;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::process::Command;

mod render;

/// Helper to create a properly-sized (48x48 max) drag texture icon preview.
fn create_drag_icon_texture(app: &DesktopApp) -> Option<gtk4::gdk::Texture> {
    let icon_str = app.icon.as_deref().unwrap_or("application-x-executable");

    // 1. Direct file path
    if icon_str.starts_with('/') {
        if let Ok(pb) = gdk_pixbuf::Pixbuf::from_file_at_scale(icon_str, 48, 48, true) {
            return Some(gtk4::gdk::Texture::for_pixbuf(&pb));
        }
    }

    // 2. Resolved path via ui_kit
    if let Some(resolved_path) = babydra_ui_kit::ui::icon::resolver::get_resolved_icon(icon_str) {
        if let Ok(pb) = gdk_pixbuf::Pixbuf::from_file_at_scale(&resolved_path, 48, 48, true) {
            return Some(gtk4::gdk::Texture::for_pixbuf(&pb));
        }
    }

    // 3. Icon theme lookup
    if let Some(display) = gtk4::gdk::Display::default() {
        let theme = gtk4::IconTheme::for_display(&display);
        let paintable = theme.lookup_icon(
            icon_str,
            &[],
            48,
            1,
            gtk4::TextDirection::Ltr,
            gtk4::IconLookupFlags::empty(),
        );
        if let Some(file) = paintable.file() {
            if let Some(path) = file.path() {
                if let Ok(pb) = gdk_pixbuf::Pixbuf::from_file_at_scale(path, 48, 48, true) {
                    return Some(gtk4::gdk::Texture::for_pixbuf(&pb));
                }
            }
        }
    }

    // 4. Default fallback icon
    if let Some(display) = gtk4::gdk::Display::default() {
        let theme = gtk4::IconTheme::for_display(&display);
        let paintable = theme.lookup_icon(
            "application-x-executable",
            &[],
            48,
            1,
            gtk4::TextDirection::Ltr,
            gtk4::IconLookupFlags::empty(),
        );
        if let Some(file) = paintable.file() {
            if let Some(path) = file.path() {
                if let Ok(pb) = gdk_pixbuf::Pixbuf::from_file_at_scale(path, 48, 48, true) {
                    return Some(gtk4::gdk::Texture::for_pixbuf(&pb));
                }
            }
        }
    }

    None
}

/// Pins the specified application to ~/Desktop as a shortcut.
pub fn pin_app_to_desktop(app: &DesktopApp) {
    let desktop_dir = dirs::desktop_dir().unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join("Desktop")
    });
    let _ = std::fs::create_dir_all(&desktop_dir);

    let final_path = if let Some(ref path) = app.file_path {
        if path.exists() {
            Some(path.clone())
        } else {
            None
        }
    } else {
        None
    }
    .or_else(|| {
        let exec_bin = app
            .exec
            .split_whitespace()
            .next()?
            .split('/')
            .last()?;
        let sys_path = PathBuf::from(format!("/usr/share/applications/{}.desktop", exec_bin));
        if sys_path.exists() {
            Some(sys_path)
        } else {
            let user_path = dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("applications")
                .join(format!("{}.desktop", exec_bin));
            if user_path.exists() {
                Some(user_path)
            } else {
                None
            }
        }
    });

    if let Some(src_path) = final_path {
        let file_name = src_path
            .file_name()
            .map(|n| n.to_os_string())
            .unwrap_or_else(|| {
                let sanitized = app.name.to_lowercase().replace(' ', "-");
                format!("{}.desktop", sanitized).into()
            });

        let dest_path = desktop_dir.join(&file_name);
        if src_path != dest_path {
            let _ = std::fs::copy(&src_path, &dest_path);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = std::fs::metadata(&dest_path) {
                    let mut perms = metadata.permissions();
                    perms.set_mode(perms.mode() | 0o755);
                    let _ = std::fs::set_permissions(&dest_path, perms);
                }
            }
        }
    } else {
        let sanitized = app.name.to_lowercase().replace(' ', "-");
        let dest_path = desktop_dir.join(format!("{}.desktop", sanitized));
        let icon_line = app
            .icon
            .as_ref()
            .map(|ic| format!("Icon={}\n", ic))
            .unwrap_or_default();
        let content = format!(
            "[Desktop Entry]\nType=Application\nName={}\nExec={}\n{}Terminal=false\n",
            app.name, app.exec, icon_line
        );
        let _ = std::fs::write(&dest_path, content);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&dest_path) {
                let mut perms = metadata.permissions();
                perms.set_mode(perms.mode() | 0o755);
                let _ = std::fs::set_permissions(&dest_path, perms);
            }
        }
    }
}

/// Attaches a DragSource to an application button so users can drag & drop shortcuts
/// directly onto the desktop or explore windows.
fn attach_app_drag_source(btn: &gtk4::Button, app: &DesktopApp, window: &gtk4::ApplicationWindow) {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::COPY);

    let app_c = app.clone();
    let win_c = window.clone();
    let win_drag_begin = window.clone();

    drag_source.connect_prepare(move |_, _, _| {
        let file_path = if let Some(ref path) = app_c.file_path {
            if path.exists() {
                Some(path.clone())
            } else {
                None
            }
        } else {
            None
        };

        let final_path = file_path.or_else(|| {
            let exec_bin = app_c
                .exec
                .split_whitespace()
                .next()?
                .split('/')
                .last()?;
            let sys_path = PathBuf::from(format!("/usr/share/applications/{}.desktop", exec_bin));
            if sys_path.exists() {
                Some(sys_path)
            } else {
                let user_path = dirs::data_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("applications")
                    .join(format!("{}.desktop", exec_bin));
                if user_path.exists() {
                    Some(user_path)
                } else {
                    None
                }
            }
        });

        if let Some(path) = final_path {
            let file = gtk4::gio::File::for_path(&path);
            let file_list = gtk4::gdk::FileList::from_array(&[file]);
            Some(gtk4::gdk::ContentProvider::for_value(&file_list.to_value()))
        } else {
            None
        }
    });

    if let Some(texture) = create_drag_icon_texture(app) {
        drag_source.set_icon(Some(&texture), 24, 24);
    }

    drag_source.connect_drag_begin(move |_, _| {
        win_drag_begin.set_visible(false);
    });

    drag_source.connect_drag_end(move |_, _, _| {
        win_c.close();
    });

    btn.add_controller(drag_source);
}

/// Attaches a right-click context menu to an application button.
fn attach_app_right_click(btn: &gtk4::Button, app: &DesktopApp, window: &gtk4::ApplicationWindow) {
    let right_click = gtk4::GestureClick::new();
    right_click.set_button(3); // Right click

    let app_c = app.clone();
    let win_c = window.clone();
    let btn_c = btn.clone();

    right_click.connect_pressed(move |_, _, x, y| {
        let app_launch = app_c.clone();
        let win_launch = win_c.clone();
        let app_pin = app_c.clone();
        let win_pin = win_c.clone();

        babydra_ui_kit::components::context_menu::ContextMenuBuilder::new(&btn_c)
            .at_coords(x, y)
            .item("Mở ứng dụng", "media-playback-start", move || {
                let parts: Vec<&str> = app_launch.exec.split_whitespace().collect();
                if !parts.is_empty() {
                    let program = program_part(parts[0]);
                    let args = &parts[1..];
                    let _ = Command::new(program).args(args).spawn();
                }
                win_launch.close();
            })
            .item("Ghim ra Màn hình chính", "bookmark-new", move || {
                pin_app_to_desktop(&app_pin);
                win_pin.close();
            })
            .popup();
    });

    btn.add_controller(right_click);
}

/// Creates a grid layout application button widget, binding its click event to launch the app.
pub fn create_grid_app(app: &DesktopApp, window: &gtk4::ApplicationWindow) -> gtk4::Button {
    let (btn, _, _) = render::build_grid_app_ui(app);

    let exec_cmd = app.exec.clone();
    let win_to_close = window.clone();
    btn.connect_clicked(move |_| {
        let parts: Vec<&str> = exec_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let program = parts[0];
            let args = &parts[1..];
            let _ = Command::new(program).args(args).spawn();
        }

        win_to_close.close();
    });

    let motion = gtk4::EventControllerMotion::new();
    let btn_clone = btn.clone();
    motion.connect_enter(move |_, _, _| {
        btn_clone.grab_focus();
    });
    btn.add_controller(motion);

    attach_app_drag_source(&btn, app, window);
    attach_app_right_click(&btn, app, window);

    btn
}

/// Creates a list row application button widget, binding its click event to launch the app.
pub fn create_list_app(app: &DesktopApp, window: &gtk4::ApplicationWindow) -> gtk4::Button {
    let (btn, _, _) = render::build_list_app_ui(app);

    let exec_cmd = app.exec.clone();
    let win_to_close = window.clone();
    btn.connect_clicked(move |_| {
        let parts: Vec<&str> = exec_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let program = program_part(parts[0]);
            let args = &parts[1..];
            let _ = Command::new(program).args(args).spawn();
        }

        win_to_close.close();
    });

    let motion = gtk4::EventControllerMotion::new();
    let btn_clone = btn.clone();
    motion.connect_enter(move |_, _, _| {
        btn_clone.grab_focus();
    });
    btn.add_controller(motion);

    attach_app_drag_source(&btn, app, window);
    attach_app_right_click(&btn, app, window);

    btn
}

/// Strip any field code specifiers from Exec fields (e.g. %u, %U, %f, %F).
fn program_part(raw: &str) -> &str {
    raw.trim_end_matches('%')
}
