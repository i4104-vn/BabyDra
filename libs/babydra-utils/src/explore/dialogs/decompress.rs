use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Entry, Button, Align, Window, Spinner, TextView, ScrolledWindow};
use std::path::PathBuf;
use std::rc::Rc;

use babydra_common::i18n::t;

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
            show_decompress_log_dialog(archive_path_c, cp_c, nav_c, None);
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

async fn check_password_correct(archive_path: &PathBuf, password: &str) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return false,
    };
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
    let password_esc = password.replace("'", "'\\''");
    
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .current_dir(parent_dir)
            .arg("-c")
            .arg(format!("unzip -t -P '{}' '{}'", password_esc, filename))
            .output()
    }).await;
    
    if let Ok(Ok(out)) = output {
        out.status.success()
    } else {
        false
    }
}

async fn perform_decompress_get_logs(archive_path: PathBuf, password: Option<String>) -> (bool, String) {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return (false, "Invalid parent directory path".to_string()),
    };
    
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
    let is_zip = filename.ends_with(".zip");
    
    let cmd_str = if is_zip {
        if let Some(pass) = password {
            let pass_esc = pass.replace("'", "'\\''");
            format!("unzip -o -P '{}' '{}'", pass_esc, filename)
        } else {
            format!("unzip -o '{}'", filename)
        }
    } else {
        format!("tar -xvf '{}'", filename)
    };
    
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .current_dir(parent_dir)
            .arg("-c")
            .arg(&cmd_str)
            .output()
    }).await;
    
    match output {
        Ok(Ok(out)) => {
            let success = out.status.success();
            let mut logs = String::from_utf8_lossy(&out.stdout).to_string();
            let errs = String::from_utf8_lossy(&out.stderr).to_string();
            if !errs.is_empty() {
                logs.push_str("\n--- Errors ---\n");
                logs.push_str(&errs);
            }
            (success, logs)
        }
        _ => (false, "Failed to spawn decompression command".to_string()),
    }
}

pub fn show_decompress_log_dialog(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    password: Option<String>,
) {
    let window = Window::builder()
        .title(&t("explore.dialog_decompress_title"))
        .modal(true)
        .resizable(true)
        .default_width(450)
        .default_height(300)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let status_box = Box::new(Orientation::Horizontal, 10);
    let lbl_status = Label::builder()
        .label(&t("explore.decompressing_running"))
        .halign(Align::Start)
        .hexpand(true)
        .build();
    let spinner = Spinner::new();
    spinner.start();

    status_box.append(&lbl_status);
    status_box.append(&spinner);
    vbox.append(&status_box);

    let scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .min_content_height(180)
        .build();
    scroll.add_css_class("log-scroller");

    let text_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .wrap_mode(gtk4::WrapMode::WordChar)
        .build();
    text_view.add_css_class("log-textview");
    
    let buffer = text_view.buffer();
    buffer.set_text(&format!("$ Decompressing {}...\n", archive_path.file_name().unwrap().to_string_lossy()));
    
    scroll.set_child(Some(&text_view));
    vbox.append(&scroll);

    let btn_close = Button::builder()
        .label(&t("explore.settings_close"))
        .sensitive(false)
        .halign(Align::End)
        .build();
    vbox.append(&btn_close);

    let win_c = window.clone();
    btn_close.connect_clicked(move |_| {
        win_c.close();
    });

    let archive_path_c = archive_path.clone();
    let current_path_c = current_path.clone();
    let nav_c = nav_callback.clone();
    let spinner_c = spinner.clone();
    let lbl_status_c = lbl_status.clone();
    let buffer_c = buffer.clone();
    let btn_close_c = btn_close.clone();

    glib::spawn_future_local(async move {
        let (success, logs) = perform_decompress_get_logs(archive_path_c, password).await;
        
        spinner_c.stop();
        spinner_c.set_visible(false);
        
        let current_text = buffer_c.text(&buffer_c.start_iter(), &buffer_c.end_iter(), false);
        if success {
            lbl_status_c.set_markup(&format!("<b><span color='#22c55e'>{}</span></b>", t("explore.decompress_success")));
            buffer_c.set_text(&format!("{}{}\n\nDecompression completed successfully!", current_text, logs));
        } else {
            lbl_status_c.set_markup(&format!("<b><span color='#ef4444'>{}</span></b>", t("explore.decompress_failed")));
            buffer_c.set_text(&format!("{}{}\n\nDecompression failed with error:\n{}", current_text, logs, logs));
        }
        
        btn_close_c.set_sensitive(true);
        nav_c(current_path_c);
    });

    window.present();
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
            let correct = check_password_correct(&archive_path_f, &password).await;
            if correct {
                win_f.close();
                show_decompress_log_dialog(archive_path_f, current_path_f, nav_f, Some(password));
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
