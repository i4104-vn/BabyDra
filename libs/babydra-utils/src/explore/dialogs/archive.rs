use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Entry, Button, Align, Window, CheckButton};
use std::path::PathBuf;
use std::rc::Rc;

use babydra_common::i18n::t;

/// Presents a dialog window to compress selected files/folders.
pub fn show_compress_dialog(
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    if target_paths.is_empty() {
        return;
    }
    
    let window = Window::builder()
        .title(&t("explore.dialog_archive_title"))
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(180)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label(&t("explore.dialog_archive_label"))
        .halign(Align::Start)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    let default_name = if target_paths.len() == 1 {
        target_paths[0].file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "archive".to_string())
    } else {
        "archive".to_string()
    };
    entry.set_text(&default_name);
    entry.set_hexpand(true);
    vbox.append(&entry);

    let format_box = Box::new(Orientation::Horizontal, 16);
    let opt_zip = CheckButton::builder().label("ZIP").active(true).build();
    let opt_tar = CheckButton::builder().label("TAR").group(&opt_zip).build();
    format_box.append(&opt_zip);
    format_box.append(&opt_tar);
    vbox.append(&format_box);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    let btn_create = Button::builder()
        .label(&t("explore.menu_compress"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_create);

    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    let win_create = window.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    let opt_zip_c = opt_zip.clone();
    
    btn_create.connect_clicked(move |_| {
        let name = entry_c.text().to_string();
        if !name.is_empty() {
            let is_zip = opt_zip_c.is_active();
            let ext = if is_zip { "zip" } else { "tar" };
            let archive_name = format!("{}.{}", name, ext);
            let archive_path = current_p.join(archive_name);
            
            let nav_c = nav.clone();
            let cp_c = current_p.clone();
            let paths = target_paths.clone();
            
            glib::spawn_future_local(async move {
                let success = perform_compress(paths, archive_path, is_zip).await;
                if !success {
                    eprintln!("Compression failed!");
                }
                nav_c(cp_c);
            });
        }
        win_create.close();
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_create.emit_clicked();
    });

    window.present();
    entry_trigger.grab_focus();
}

async fn perform_compress(paths: Vec<PathBuf>, archive_path: PathBuf, is_zip: bool) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p,
        None => return false,
    };
    
    let mut cmd = std::process::Command::new("sh");
    cmd.current_dir(parent_dir);
    
    let archive_filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
    
    let files: Vec<String> = paths.iter()
        .filter_map(|p| p.file_name().map(|f| f.to_string_lossy().to_string()))
        .collect();
        
    let cmd_str = if is_zip {
        format!("zip -r '{}' {}", archive_filename, files.iter().map(|f| format!("'{}'", f)).collect::<Vec<_>>().join(" "))
    } else {
        format!("tar -cf '{}' {}", archive_filename, files.iter().map(|f| format!("'{}'", f)).collect::<Vec<_>>().join(" "))
    };
    
    cmd.arg("-c").arg(&cmd_str);
    
    match cmd.spawn() {
        Ok(mut child) => {
            match tokio::task::spawn_blocking(move || child.wait()).await {
                Ok(Ok(status)) => status.success(),
                _ => false,
            }
        }
        Err(_) => false,
    }
}

pub fn perform_decompress_async(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let nav_c = nav_callback.clone();
    let cp_c = current_path.clone();
    let archive_path_c = archive_path.clone();
    
    glib::spawn_future_local(async move {
        let is_zip = archive_path_c.file_name()
            .map(|f| f.to_string_lossy().ends_with(".zip"))
            .unwrap_or(false);
            
        if is_zip && is_zip_encrypted(&archive_path_c).await {
            show_password_dialog(archive_path_c, cp_c, nav_c);
        } else {
            let success = perform_decompress(archive_path_c).await;
            if !success {
                eprintln!("Decompression failed!");
            }
            nav_c(cp_c);
        }
    });
}

async fn is_zip_encrypted(archive_path: &PathBuf) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return false,
    };
    
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
    
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .current_dir(parent_dir)
            .arg("-c")
            .arg(format!("unzip -t -P \"\" '{}'", filename))
            .output()
    }).await;
    
    if let Ok(Ok(out)) = output {
        let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
        let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
        
        stdout.contains("password") || stderr.contains("password") ||
        stdout.contains("incorrect password") || stderr.contains("incorrect password") ||
        stdout.contains("encrypted") || stderr.contains("encrypted") ||
        out.status.code() == Some(82) || out.status.code() == Some(81)
    } else {
        false
    }
}

async fn perform_decompress(archive_path: PathBuf) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p,
        None => return false,
    };
    
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
    let is_zip = filename.ends_with(".zip");
    let is_tar = filename.ends_with(".tar") || filename.ends_with(".tar.xz") || filename.ends_with(".tar.gz") || filename.ends_with(".tgz");
    
    if !is_zip && !is_tar {
        return false;
    }
    
    let mut cmd = std::process::Command::new("sh");
    cmd.current_dir(parent_dir);
    
    let cmd_str = if is_zip {
        format!("unzip -o '{}'", filename)
    } else {
        format!("tar -xf '{}'", filename)
    };
    
    cmd.arg("-c").arg(&cmd_str);
    
    match cmd.spawn() {
        Ok(mut child) => {
            match tokio::task::spawn_blocking(move || child.wait()).await {
                Ok(Ok(status)) => status.success(),
                _ => false,
            }
        }
        Err(_) => false,
    }
}

async fn perform_decompress_with_password(archive_path: PathBuf, password: &str) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p,
        None => return false,
    };
    
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
    let password_esc = password.replace("'", "'\\''");
    
    let mut cmd = std::process::Command::new("sh");
    cmd.current_dir(parent_dir);
    cmd.arg("-c").arg(format!("unzip -o -P '{}' '{}'", password_esc, filename));
    
    match cmd.spawn() {
        Ok(mut child) => {
            match tokio::task::spawn_blocking(move || child.wait()).await {
                Ok(Ok(status)) => status.success(),
                _ => false,
            }
        }
        Err(_) => false,
    }
}

pub fn show_password_dialog(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let window = Window::builder()
        .title(&t("explore.dialog_password_title"))
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(170)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label(&t("explore.dialog_password_label"))
        .halign(Align::Start)
        .wrap(true)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_visibility(false);
    entry.set_hexpand(true);
    entry.add_css_class("small-entry");
    vbox.append(&entry);

    let lbl_error = Label::builder()
        .halign(Align::Start)
        .use_markup(true)
        .build();
    lbl_error.add_css_class("error-label");
    vbox.append(&lbl_error);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&t("explore.settings_cancel"));
    let btn_extract = Button::builder()
        .label(&t("explore.menu_decompress"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_extract);

    let win_c = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_c.close();
    });

    let win_c2 = window.clone();
    let archive_path_c = archive_path.clone();
    let current_path_c = current_path.clone();
    let nav_c = nav_callback.clone();
    let entry_c = entry.clone();
    let lbl_error_c = lbl_error.clone();

    btn_extract.connect_clicked(move |_| {
        let password = entry_c.text().to_string();
        let archive_path_f = archive_path_c.clone();
        let current_path_f = current_path_c.clone();
        let nav_f = nav_c.clone();
        let win_f = win_c2.clone();
        let lbl_err_f = lbl_error_c.clone();

        glib::spawn_future_local(async move {
            let success = perform_decompress_with_password(archive_path_f, &password).await;
            if success {
                win_f.close();
                nav_f(current_path_f);
            } else {
                lbl_err_f.set_markup(&format!("<span color='#ef4444'>{}</span>", t("explore.dialog_password_incorrect")));
            }
        });
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_extract.emit_clicked();
    });

    window.present();
    entry_trigger.grab_focus();
}
