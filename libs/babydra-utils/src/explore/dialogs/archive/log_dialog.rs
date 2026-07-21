use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Align, Window, Spinner, TextView, ScrolledWindow, ProgressBar};
use std::path::PathBuf;
use std::rc::Rc;
use tokio::io::{AsyncBufReadExt, BufReader};

use babydra_common::i18n::t;
use babydra_common::services::explore::spawn_compress_command;
use crate::explore::dialogs::shared::scroll_to_end;

pub fn show_compress_log_dialog(
    target_paths: Vec<PathBuf>,
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    is_zip: bool,
) {
    let window = Window::builder()
        .title(&t("explore.dialog_archive_title"))
        .modal(true)
        .resizable(true)
        .default_width(500)
        .default_height(320)
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
        .label(&t("explore.compressing_running"))
        .halign(Align::Start)
        .hexpand(true)
        .build();
    let spinner = Spinner::new();
    spinner.start();

    status_box.append(&lbl_status);
    status_box.append(&spinner);
    vbox.append(&status_box);

    let progress_bar = ProgressBar::builder()
        .hexpand(true)
        .build();
    vbox.append(&progress_bar);

    let scroll = ScrolledWindow::builder()
        .hexpand(true)
        .vexpand(true)
        .min_content_height(200)
        .build();
    scroll.add_css_class("log-scroller");

    let text_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .monospace(true)
        .wrap_mode(gtk4::WrapMode::WordChar)
        .build();
    text_view.add_css_class("log-textview");
    
    let buffer = text_view.buffer();
    let fname = archive_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    buffer.set_text(&format!("$ Creating archive {}...\n\n", fname));
    
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

    let nav_c = nav_callback.clone();
    let current_path_c = current_path.clone();
    
    let is_running = Rc::new(std::cell::Cell::new(true));
    let is_running_c = is_running.clone();
    let pb_pulse = progress_bar.clone();
    glib::spawn_future_local(async move {
        while is_running_c.get() {
            pb_pulse.pulse();
            glib::timeout_future(std::time::Duration::from_millis(100)).await;
        }
    });

    let pb_finish = progress_bar.clone();
    let spinner_c = spinner.clone();
    let lbl_status_c = lbl_status.clone();
    let buffer_c = buffer.clone();
    let btn_close_c = btn_close.clone();
    let text_view_c = text_view.clone();

    glib::spawn_future_local(async move {
        let parent_dir = match archive_path.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                is_running.set(false);
                spinner_c.stop();
                spinner_c.set_visible(false);
                pb_finish.set_fraction(0.0);
                lbl_status_c.set_markup(&format!("<b><span foreground='#ef4444'>{}</span></b>", t("explore.compress_failed")));
                buffer_c.set_text("Invalid parent directory");
                btn_close_c.set_sensitive(true);
                return;
            }
        };
        
        let archive_filename = archive_path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let files: Vec<String> = target_paths.iter()
            .filter_map(|p| p.file_name().map(|f| f.to_string_lossy().to_string()))
            .collect();
            
        match spawn_compress_command(&parent_dir, &archive_filename, &files, is_zip) {
            Ok(mut child) => {
                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();
                
                let mut stdout_reader = BufReader::new(stdout).lines();
                let mut stderr_reader = BufReader::new(stderr).lines();
                
                loop {
                    tokio::select! {
                        res = stdout_reader.next_line() => {
                            if let Ok(Some(line)) = res {
                                let mut end = buffer_c.end_iter();
                                buffer_c.insert(&mut end, &format!("{}\n", line));
                                scroll_to_end(&text_view_c);
                            }
                        }
                        res = stderr_reader.next_line() => {
                            if let Ok(Some(line)) = res {
                                let mut end = buffer_c.end_iter();
                                buffer_c.insert(&mut end, &format!("{}\n", line));
                                scroll_to_end(&text_view_c);
                            }
                        }
                        status = child.wait() => {
                            is_running.set(false);
                            spinner_c.stop();
                            spinner_c.set_visible(false);
                            
                            let success = status.map(|s| s.success()).unwrap_or(false);
                            if success {
                                pb_finish.set_fraction(1.0);
                                lbl_status_c.set_markup(&format!("<b><span foreground='#22c55e'>{}</span></b>", t("explore.compress_success")));
                                let mut end = buffer_c.end_iter();
                                buffer_c.insert(&mut end, "\n✓ Compression completed successfully.\n");
                            } else {
                                pb_finish.set_fraction(0.0);
                                lbl_status_c.set_markup(&format!("<b><span foreground='#ef4444'>{}</span></b>", t("explore.compress_failed")));
                                let mut end = buffer_c.end_iter();
                                buffer_c.insert(&mut end, "\n✗ Compression failed.\n");
                            }
                            scroll_to_end(&text_view_c);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                is_running.set(false);
                spinner_c.stop();
                spinner_c.set_visible(false);
                pb_finish.set_fraction(0.0);
                lbl_status_c.set_markup(&format!("<b><span foreground='#ef4444'>{}</span></b>", t("explore.compress_failed")));
                let mut end = buffer_c.end_iter();
                buffer_c.insert(&mut end, &format!("Failed to spawn compress process: {}\n", e));
                scroll_to_end(&text_view_c);
            }
        }
        
        btn_close_c.set_sensitive(true);
        nav_c(current_path_c);
    });

    window.present();
}
