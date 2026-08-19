use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, Label, Orientation, ProgressBar, ScrolledWindow, Spinner, TextView, Window,
};
use std::path::PathBuf;
use std::rc::Rc;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::components::explore::dialogs::shared::scroll_to_end;
use babydra_core::i18n::trans;
use babydra_core::services::explore::spawn_decompress;

/// Show decompress log dialog.
pub fn show_decompress_log(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    password: Option<String>,
    parent: Option<&gtk4::Window>,
) {
    let window = Window::builder()
        .title(&trans("explore.dialog_decompress_title"))
        .icon_name("babydra")
        .modal(true)
        .resizable(true)
        .default_width(500)
        .default_height(320)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let status_box = Box::new(Orientation::Horizontal, 10);
    let lbl_status = Label::builder()
        .label(&trans("explore.decompressing_running"))
        .halign(Align::Start)
        .hexpand(true)
        .build();
    let spinner = Spinner::new();
    spinner.start();

    status_box.append(&lbl_status);
    status_box.append(&spinner);
    vbox.append(&status_box);

    let progress_bar = ProgressBar::builder().hexpand(true).build();
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
    let fname = archive_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    buffer.set_text(&format!(
        "$ {}\n\n",
        trans("explore.extracting_archive").replace("{}", &fname)
    ));

    scroll.set_child(Some(&text_view));
    vbox.append(&scroll);

    let btn_close = Button::builder()
        .label(&trans("explore.settings_close"))
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
                lbl_status_c.set_markup(&format!(
                    "<b><span foreground='#ef4444'>{}</span></b>",
                    trans("explore.decompress_failed")
                ));
                buffer_c.set_text(&trans("explore.invalid_parent_dir"));
                btn_close_c.set_sensitive(true);
                return;
            }
        };

        let filename = archive_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        match spawn_decompress(&parent_dir, &filename, password.as_deref()) {
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
                                lbl_status_c.set_markup(&format!("<b><span foreground='#22c55e'>{}</span></b>", trans("explore.decompress_success")));
                                let mut end = buffer_c.end_iter();
                                buffer_c.insert(&mut end, &format!("\n✓ {}\n", trans("explore.decompress_completed")));
                            } else {
                                pb_finish.set_fraction(0.0);
                                lbl_status_c.set_markup(&format!("<b><span foreground='#ef4444'>{}</span></b>", trans("explore.decompress_failed")));
                                let mut end = buffer_c.end_iter();
                                buffer_c.insert(&mut end, &format!("\n✗ {}\n", trans("explore.decompress_failed_detail")));
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
                lbl_status_c.set_markup(&format!(
                    "<b><span foreground='#ef4444'>{}</span></b>",
                    trans("explore.decompress_failed")
                ));
                let mut end = buffer_c.end_iter();
                buffer_c.insert(
                    &mut end,
                    &format!(
                        "{}\n",
                        trans("explore.spawn_decompress_failed").replace("{}", &e.to_string())
                    ),
                );
                scroll_to_end(&text_view_c);
            }
        }

        btn_close_c.set_sensitive(true);
        nav_c(current_path_c);
    });

    window.present();
}
