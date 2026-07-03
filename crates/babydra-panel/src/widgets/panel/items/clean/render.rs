use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use tokio::sync::mpsc;
use super::{clean_all_native, format_bytes, get_dir_size, get_journal_size, get_orphans_size, get_trash_size};

#[derive(Clone)]
enum CleanProgress {
    Log(String),
    ScanFinished { total_bytes: u64 },
    CleanFinished { total_bytes: u64 },
}

pub fn create_clean_tile(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-square-tile");
    btn.set_hexpand(true);
    btn.set_valign(gtk4::Align::Fill);
    btn.set_vexpand(true);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.set_valign(gtk4::Align::Center);
    main_box.set_halign(gtk4::Align::Center);

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_halign(gtk4::Align::Center);

    let icon_widget = babydra_common::icon::get_icon_colored("trash", 16, "rgba(255, 255, 255, 0.8)");
    icon_container.append(&icon_widget);

    let label = gtk4::Label::new(Some(&babydra_common::i18n::t("control.clean")));
    label.add_css_class("control-square-label");
    label.set_halign(gtk4::Align::Center);

    main_box.append(&icon_container);
    main_box.append(&label);
    btn.set_child(Some(&main_box));

    // Create the Popover container
    let popover = gtk4::Popover::new();
    popover.add_css_class("media-popover"); // Inherit media player popover styles
    popover.set_parent(&btn);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.set_has_arrow(false);

    setup_clean_popover(&popover);

    let on_popover_toggled_c = on_popover_toggled.clone();
    let popover_c = popover.clone();
    btn.connect_clicked(move |_| {
        popover_c.popup();
        if let Some(ref cb) = on_popover_toggled_c {
            cb(true);
        }
    });

    if let Some(ref cb) = on_popover_toggled {
        let cb_clone = cb.clone();
        popover.connect_closed(move |_| {
            cb_clone(false);
        });
    }

    btn
}

fn setup_clean_popover(popover: &gtk4::Popover) {
    let popover_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    popover_box.add_css_class("media-popover-box");
    popover_box.set_margin_start(4);
    popover_box.set_margin_end(4);

    // Header
    let popover_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    popover_header.add_css_class("media-popover-header");
    popover_header.set_valign(gtk4::Align::Center);
    let popover_app_icon = babydra_common::icon::get_icon_colored("trash", 14, "#ef4444");
    let popover_app_name = gtk4::Label::new(Some(&babydra_common::i18n::t("control.clean_my_linux")));
    popover_app_name.add_css_class("media-popover-app-name");
    popover_header.append(&popover_app_icon);
    popover_header.append(&popover_app_name);
    popover_box.append(&popover_header);

    // Status Title
    let status_title_label = gtk4::Label::new(Some("System Cleanup"));
    status_title_label.add_css_class("media-popover-title");
    status_title_label.set_halign(gtk4::Align::Center);
    status_title_label.set_justify(gtk4::Justification::Center);
    status_title_label.set_wrap(true);
    status_title_label.set_max_width_chars(25);
    popover_box.append(&status_title_label);

    // Status Desc
    let status_desc_label = gtk4::Label::new(Some("Scan for temporary & unused files"));
    status_desc_label.add_css_class("media-popover-artist");
    status_desc_label.set_halign(gtk4::Align::Center);
    status_desc_label.set_justify(gtk4::Justification::Center);
    status_desc_label.set_wrap(true);
    status_desc_label.set_max_width_chars(30);
    popover_box.append(&status_desc_label);

    // Pill Button Container
    let btn_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    btn_container.set_halign(gtk4::Align::Center);
    btn_container.set_margin_top(8);
    btn_container.set_margin_bottom(8);

    let action_btn = gtk4::Button::with_label(&babydra_common::i18n::t("control.scan"));
    action_btn.add_css_class("wifi-btn-primary");
    action_btn.set_size_request(120, -1);
    btn_container.append(&action_btn);
    popover_box.append(&btn_container);

    // Log label at the bottom
    let log_label = gtk4::Label::new(Some("Ready"));
    log_label.add_css_class("media-time-label");
    log_label.set_halign(gtk4::Align::Center);
    log_label.set_justify(gtk4::Justification::Center);
    popover_box.append(&log_label);

    popover.set_child(Some(&popover_box));

    // Shared State to hold current reclaimable bytes
    let total_reclaimable = Rc::new(RefCell::new(0u64));

    // Connect button click handler
    let action_btn_c = action_btn.clone();
    let status_title_label_c = status_title_label.clone();
    let status_desc_label_c = status_desc_label.clone();
    let log_label_c = log_label.clone();
    let total_reclaimable_c = total_reclaimable.clone();

    action_btn.connect_clicked(move |btn| {
        let label = btn.label().unwrap_or_default().to_string();
        let scan_label_str = babydra_common::i18n::t("control.scan");
        let free_label_str = babydra_common::i18n::t("control.free");

        if label == scan_label_str {
            // Trigger Scan
            btn.set_sensitive(false);
            btn.set_label(&babydra_common::i18n::t("control.scanning"));
            status_title_label_c.set_text("Scanning System...");
            status_desc_label_c.set_text("Analyzing disk space...");

            let (tx, mut rx) = mpsc::unbounded_channel::<CleanProgress>();
            
            std::thread::spawn(move || {
                let mut total = 0u64;

                let _ = tx.send(CleanProgress::Log("Analyzing user cache...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let cache_size = get_dir_size("~/.cache");
                total += cache_size;

                let _ = tx.send(CleanProgress::Log("Checking orphaned packages...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let orphan_size = get_orphans_size();
                total += orphan_size;

                let _ = tx.send(CleanProgress::Log("Calculating pacman package cache...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let pacman_cache = get_dir_size("/var/cache/pacman/pkg");
                total += pacman_cache;

                let _ = tx.send(CleanProgress::Log("Reading system log size...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let journal_size = get_journal_size();
                total += journal_size;

                let _ = tx.send(CleanProgress::Log("Checking trash bin...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let trash_size = get_trash_size();
                total += trash_size;

                let _ = tx.send(CleanProgress::ScanFinished { total_bytes: total });
            });

            let action_btn_inner = action_btn_c.clone();
            let status_title_inner = status_title_label_c.clone();
            let status_desc_inner = status_desc_label_c.clone();
            let log_inner = log_label_c.clone();
            let total_inner = total_reclaimable_c.clone();

            glib::spawn_future_local(async move {
                while let Some(progress) = rx.recv().await {
                    match progress {
                        CleanProgress::Log(msg) => {
                            log_inner.set_text(&msg);
                        }
                        CleanProgress::ScanFinished { total_bytes } => {
                            log_inner.set_text("Scan complete");
                            *total_inner.borrow_mut() = total_bytes;
                            action_btn_inner.set_sensitive(true);

                            if total_bytes > 0 {
                                status_title_inner.set_text("Scan Results");
                                let size_str = format_bytes(total_bytes);
                                let desc = babydra_common::i18n::t("control.bytes_can_be_freed")
                                    .replace("{}", &size_str);
                                status_desc_inner.set_text(&desc);
                                action_btn_inner.set_label(&babydra_common::i18n::t("control.free"));
                            } else {
                                status_title_inner.set_text("System Cleanup");
                                status_desc_inner.set_text(&babydra_common::i18n::t("control.nothing_to_free"));
                                action_btn_inner.set_label(&babydra_common::i18n::t("control.scan"));
                            }
                        }
                        _ => {}
                    }
                }
            });
        } else if label == free_label_str {
            // Trigger Clean
            btn.set_sensitive(false);
            btn.set_label("Cleaning...");
            status_title_label_c.set_text("Releasing Space...");
            status_desc_label_c.set_text("Running file cleanup...");

            let (tx, mut rx) = mpsc::unbounded_channel::<CleanProgress>();

            std::thread::spawn(move || {
                let _ = tx.send(CleanProgress::Log("Purging safe user caches...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));

                let _ = tx.send(CleanProgress::Log("Cleaning system logs & package caches...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));

                let _ = tx.send(CleanProgress::Log("Emptying trash bin...".to_string()));
                let freed = clean_all_native();
                std::thread::sleep(std::time::Duration::from_millis(300));

                let _ = tx.send(CleanProgress::CleanFinished { total_bytes: freed });
            });

            let action_btn_inner = action_btn_c.clone();
            let status_title_inner = status_title_label_c.clone();
            let status_desc_inner = status_desc_label_c.clone();
            let log_inner = log_label_c.clone();
            let total_inner = total_reclaimable_c.clone();

            glib::spawn_future_local(async move {
                while let Some(progress) = rx.recv().await {
                    match progress {
                        CleanProgress::Log(msg) => {
                            log_inner.set_text(&msg);
                        }
                        CleanProgress::CleanFinished { total_bytes } => {
                            log_inner.set_text("");
                            *total_inner.borrow_mut() = 0;
                            
                            status_title_inner.set_text("Cleanup Complete");
                            let size_str = format_bytes(total_bytes);
                            let success_msg = babydra_common::i18n::t("control.freed_success")
                                .replace("{}", &size_str);
                            status_desc_inner.set_text(&success_msg);

                            action_btn_inner.set_sensitive(true);
                            action_btn_inner.set_label(&babydra_common::i18n::t("control.scan"));
                        }
                        _ => {}
                    }
                }
            });
        }
    });
}
