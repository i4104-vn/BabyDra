//! Shared "run a command job and stream its output" dialog used by both the
//! compress and decompress flows.

use gtk4::prelude::*;
use gtk4::{
    Align, Box, Button, Label, Orientation, ProgressBar, ScrolledWindow, Spinner, TextView, Window,
};
use std::path::PathBuf;
use std::rc::Rc;
use tokio::io::{AsyncBufReadExt, BufReader};

use babydra_core::i18n::trans;

/// All user-facing strings for one job run.
pub struct JobStrings {
    pub title: String,
    pub running: String,
    pub start_line: String,
    pub success: String,
    pub completed: String,
    pub failed: String,
    pub failed_detail: String,
}

/// Runs `spawn_job` (receiving the archive's parent directory) inside a modal
/// log window, streaming stdout/stderr into a text view with progress pulse.
/// Enables Close and refreshes via `nav_callback` when finished.
pub fn show_job_log(
    strings: JobStrings,
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<Window>>,
    spawn_job: impl FnOnce(&PathBuf) -> Result<tokio::process::Child, std::io::Error> + 'static,
) {
    let window = Window::builder()
        .title(&strings.title)
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
        .label(&strings.running)
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
    buffer.set_text(&format!("$ {}\n\n", strings.start_line));

    scroll.set_child(Some(&text_view));
    vbox.append(&scroll);

    let btn_close = Button::builder()
        .label(trans("explore.settings_close"))
        .sensitive(false)
        .halign(Align::End)
        .build();
    vbox.append(&btn_close);

    let win_c = window.clone();
    btn_close.connect_clicked(move |_| {
        win_c.close();
    });

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

    fn append_line(buffer: &gtk4::TextBuffer, view: &TextView, line: &str) {
        let mut end = buffer.end_iter();
        buffer.insert(&mut end, &format!("{}\n", line));
        let mark = view
            .buffer()
            .create_mark(None, &view.buffer().end_iter(), false);
        view.scroll_to_mark(&mark, 0.0, true, 0.0, 1.0);
        view.buffer().delete_mark(&mark);
    }

    fn fail(
        is_running: &Rc<std::cell::Cell<bool>>,
        spinner: &Spinner,
        pb: &ProgressBar,
        lbl: &Label,
        msg: &str,
    ) {
        is_running.set(false);
        spinner.stop();
        spinner.set_visible(false);
        pb.set_fraction(0.0);
        lbl.set_markup(&format!("<b><span foreground='#ef4444'>{}</span></b>", msg));
    }

    glib::spawn_future_local(async move {
        let parent_dir = match archive_path.parent() {
            Some(p) => p.to_path_buf(),
            None => {
                fail(
                    &is_running,
                    &spinner_c,
                    &pb_finish,
                    &lbl_status_c,
                    &strings.failed,
                );
                buffer_c.set_text(&trans("explore.invalid_parent_dir"));
                btn_close_c.set_sensitive(true);
                return;
            }
        };

        match spawn_job(&parent_dir) {
            Ok(mut child) => {
                let stdout = child.stdout.take().unwrap();
                let stderr = child.stderr.take().unwrap();

                let mut stdout_reader = BufReader::new(stdout).lines();
                let mut stderr_reader = BufReader::new(stderr).lines();

                loop {
                    tokio::select! {
                        res = stdout_reader.next_line() => {
                            if let Ok(Some(line)) = res {
                                append_line(&buffer_c, &text_view_c, &line);
                            }
                        }
                        res = stderr_reader.next_line() => {
                            if let Ok(Some(line)) = res {
                                append_line(&buffer_c, &text_view_c, &line);
                            }
                        }
                        status = child.wait() => {
                            is_running.set(false);
                            spinner_c.stop();
                            spinner_c.set_visible(false);

                            let success = status.map(|s| s.success()).unwrap_or(false);
                            if success {
                                pb_finish.set_fraction(1.0);
                                lbl_status_c.set_markup(&format!(
                                    "<b><span foreground='#22c55e'>{}</span></b>",
                                    strings.success
                                ));
                                append_line(&buffer_c, &text_view_c, &format!("\n✓ {}", strings.completed));
                            } else {
                                pb_finish.set_fraction(0.0);
                                lbl_status_c.set_markup(&format!(
                                    "<b><span foreground='#ef4444'>{}</span></b>",
                                    strings.failed
                                ));
                                append_line(&buffer_c, &text_view_c, &format!("\n✗ {}", strings.failed_detail));
                            }
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                fail(
                    &is_running,
                    &spinner_c,
                    &pb_finish,
                    &lbl_status_c,
                    &strings.failed,
                );
                append_line(
                    &buffer_c,
                    &text_view_c,
                    &trans("explore.spawn_compress_failed").replace("{}", &e.to_string()),
                );
            }
        }

        btn_close_c.set_sensitive(true);
        nav_callback(current_path);
    });

    window.present();
}
