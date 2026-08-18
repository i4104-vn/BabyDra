//! Controller wrappers for individual launcher application row and grid item buttons.

use babydra_core::DesktopApp;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::process::Command;

mod render;

/// Attaches a DragSource to an application button so users can drag & drop shortcuts
/// directly onto the desktop or explore windows.
fn attach_app_drag_source(btn: &gtk4::Button, app: &DesktopApp, window: &gtk4::ApplicationWindow) {
    let drag_source = gtk4::DragSource::new();
    drag_source.set_actions(gtk4::gdk::DragAction::COPY);

    let app_c = app.clone();
    let win_c = window.clone();

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

    let icon_name = app.icon.as_deref().unwrap_or("application-x-executable");
    let img = babydra_ui_kit::ui::icon::get_fallback_icon(icon_name, "application-x-executable");
    img.set_pixel_size(48);
    if let Some(paintable) = img.paintable() {
        drag_source.set_icon(Some(&paintable), 24, 24);
    }

    drag_source.connect_drag_end(move |_, _, _| {
        win_c.close();
    });

    btn.add_controller(drag_source);
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

    btn
}

/// Strip any field code specifiers from Exec fields (e.g. %u, %U, %f, %F).
fn program_part(raw: &str) -> &str {
    raw.trim_end_matches('%')
}
