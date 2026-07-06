use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use tokio::sync::mpsc;
use std::path::Path;
use std::fs;
use super::{clean_all_native, format_bytes, cache, logs, temp};

#[derive(Clone)]
enum CleanProgress {
    Log(String),
    ScanStep { step: usize, size: u64 },
    ScanFinished { total_bytes: u64 },
    CleanFinished { total_bytes: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CleanState {
    Idle,
    Scanning { angle: f64 },
    ScanFinished { total_bytes: u64 },
    Cleaning { progress: f64, total_bytes: u64, is_backend_done: bool, actual_freed: u64 },
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

    let icon_widget = babydra_common::icon::get_icon_colored("broom", 16, "rgba(255, 255, 255, 0.8)");
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

    let popover_box = setup_clean_popover(&popover);

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

    let popover_box_clone = popover_box.clone();
    popover.connect_map(move |_| {
        babydra_common::animation::slide_in(
            popover_box_clone.upcast_ref(),
            babydra_common::animation::SlideDirection::Down,
            15,
            450,
        );
    });

    btn
}

fn setup_clean_popover(popover: &gtk4::Popover) -> gtk4::Box {
    let popover_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    popover_box.add_css_class("media-popover-box");
    popover_box.set_margin_start(4);
    popover_box.set_margin_end(4);

    // Header
    let popover_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    popover_header.add_css_class("media-popover-header");
    popover_header.set_valign(gtk4::Align::Center);
    let popover_app_icon = babydra_common::icon::get_icon_colored("broom", 14, "#ef4444");
    let popover_app_name = gtk4::Label::new(Some(&babydra_common::i18n::t("control.clean_my_linux")));
    popover_app_name.add_css_class("media-popover-app-name");
    popover_header.append(&popover_app_icon);
    popover_header.append(&popover_app_name);
    popover_box.append(&popover_header);

    // Center Circular Progress and Text Overlay
    let overlay = gtk4::Overlay::new();
    overlay.set_halign(gtk4::Align::Center);
    overlay.set_margin_top(12);
    overlay.set_margin_bottom(12);

    let progress_drawing = gtk4::DrawingArea::new();
    progress_drawing.set_size_request(160, 160);
    overlay.set_child(Some(&progress_drawing));

    let center_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    center_box.set_valign(gtk4::Align::Center);
    center_box.set_halign(gtk4::Align::Center);

    let percent_label = gtk4::Label::new(Some("READY"));
    percent_label.add_css_class("clean-popover-percent");
    
    let size_label = gtk4::Label::new(Some("0 B"));
    size_label.add_css_class("clean-popover-size");

    center_box.append(&percent_label);
    center_box.append(&size_label);
    overlay.add_overlay(&center_box);
    popover_box.append(&overlay);

    // Target details list below circular progress bar
    let target_list_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    target_list_box.set_margin_start(16);
    target_list_box.set_margin_end(16);
    target_list_box.set_margin_top(8);
    target_list_box.set_margin_bottom(8);
    target_list_box.add_css_class("clean-target-list");

    let create_target_row = |name: &str| -> (gtk4::Box, gtk4::Label) {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.add_css_class("clean-target-row");
        row.set_hexpand(true);

        let bullet = gtk4::Label::new(Some("•"));
        bullet.add_css_class("clean-target-bullet");

        let name_lbl = gtk4::Label::new(Some(name));
        name_lbl.add_css_class("clean-target-name");
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.set_hexpand(true);

        let val_lbl = gtk4::Label::new(Some("-"));
        val_lbl.add_css_class("clean-target-value");
        val_lbl.set_halign(gtk4::Align::End);

        row.append(&bullet);
        row.append(&name_lbl);
        row.append(&val_lbl);
        (row, val_lbl)
    };

    let (row1, val1) = create_target_row("User Cache");
    let (row2, val2) = create_target_row("Package Cache");
    let (row3, val3) = create_target_row("System Logs");
    let (row4, val4) = create_target_row("Trash Bin");

    target_list_box.append(&row1);
    target_list_box.append(&row2);
    target_list_box.append(&row3);
    target_list_box.append(&row4);
    popover_box.append(&target_list_box);

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

    // Shared State to hold current reclaimable bytes and clean state
    let total_reclaimable = Rc::new(RefCell::new(0u64));
    let state = Rc::new(RefCell::new(CleanState::Idle));
    
    // Store size of each scanned category to animate their values down to 0
    let user_size = Rc::new(RefCell::new(0u64));
    let pacman_size = Rc::new(RefCell::new(0u64));
    let journal_size = Rc::new(RefCell::new(0u64));
    let trash_size = Rc::new(RefCell::new(0u64));

    // Set custom drawing logic on progress circle drawing area
    let state_draw = state.clone();
    progress_drawing.set_draw_func(move |_, cr, width, height| {
        let current_state = *state_draw.borrow();
        let cx = width as f64 / 2.0;
        let cy = height as f64 / 2.0;
        let radius = (width.min(height) as f64 / 2.0) - 8.0;

        if radius <= 0.0 {
            return;
        }

        // Draw background track circle
        cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.set_source_rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.06);
        cr.set_line_width(6.0);
        let _ = cr.stroke();

        let draw_neon = |cr: &gtk4::cairo::Context, start: f64, end: f64, r: f64, g: f64, b: f64| {
            // Outer faint glow
            cr.arc(cx, cy, radius, start, end);
            cr.set_source_rgba(r, g, b, 0.08);
            cr.set_line_width(16.0);
            let _ = cr.stroke();

            // Medium glow
            cr.arc(cx, cy, radius, start, end);
            cr.set_source_rgba(r, g, b, 0.22);
            cr.set_line_width(10.0);
            let _ = cr.stroke();

            // Core bright line
            cr.arc(cx, cy, radius, start, end);
            cr.set_source_rgba(r, g, b, 0.95);
            cr.set_line_width(6.0);
            let _ = cr.stroke();
        };

        match current_state {
            CleanState::Idle => {}
            CleanState::Scanning { angle } => {
                cr.set_line_cap(gtk4::cairo::LineCap::Round);
                
                // Pulsating arc length
                let arc_len = 1.2 + (angle * 3.0).sin().abs() * 0.6;
                // Main blue neon arc spinning clockwise
                draw_neon(cr, angle, angle + arc_len, 59.0 / 255.0, 130.0 / 255.0, 246.0 / 255.0);

                // Inner cyan neon arc spinning counter-clockwise
                let inner_radius = radius - 10.0;
                cr.arc(cx, cy, inner_radius, -angle, -angle + 1.0);
                cr.set_source_rgba(6.0 / 255.0, 182.0 / 255.0, 212.0 / 255.0, 0.9);
                cr.set_line_width(4.0);
                let _ = cr.stroke();

                // Orbiting glowing blue energy particles
                let num_scan_particles = 12;
                for i in 0..num_scan_particles {
                    let p_angle = angle + (i as f64 * (2.0 * std::f64::consts::PI / num_scan_particles as f64));
                    let wave_offset = (angle * 4.0 + i as f64).sin() * 5.0;
                    let pr = radius + wave_offset;
                    let px = cx + pr * p_angle.cos();
                    let py = cy + pr * p_angle.sin();
                    
                    let size = 2.0 + (angle * 6.0 + i as f64).sin().abs() * 1.5;
                    let alpha = 0.5 + (angle * 4.0 + i as f64).sin().abs() * 0.4;
                    
                    cr.arc(px, py, size, 0.0, 2.0 * std::f64::consts::PI);
                    cr.set_source_rgba(96.0 / 255.0, 165.0 / 255.0, 250.0 / 255.0, alpha);
                    let _ = cr.fill();
                }
            }
            CleanState::ScanFinished { total_bytes } => {
                if total_bytes > 0 {
                    draw_neon(cr, 0.0, 2.0 * std::f64::consts::PI, 59.0 / 255.0, 130.0 / 255.0, 246.0 / 255.0); // Blue Neon
                }
            }
            CleanState::Cleaning { progress, .. } => {
                cr.set_line_cap(gtk4::cairo::LineCap::Round);
                let start_angle = -std::f64::consts::FRAC_PI_2;
                let end_angle = start_angle + progress * 2.0 * std::f64::consts::PI;
                draw_neon(cr, start_angle, end_angle, 239.0 / 255.0, 68.0 / 255.0, 68.0 / 255.0); // Red Neon

                // Floating particles sucked into the center (Black Hole whirlpool effect)
                let num_particles = 30;
                for i in 0..num_particles {
                    let start_angle = (i as f64) * (2.0 * std::f64::consts::PI / num_particles as f64) + (i as f64 * 0.73);
                    let offset = (i as f64 * 0.13) % 1.0;
                    let p_time = (progress + offset) % 1.0;

                    // Spiraling inward from 2.2x radius down to 0
                    let R = radius * 2.2 * (1.0 - p_time);
                    let theta = start_angle + p_time * 7.0; // Spiraling whirlpool angle

                    let px = cx + R * theta.cos();
                    let py = cy + R * theta.sin();

                    let size = 2.5 * (1.0 - p_time);
                    let alpha = 0.9 * (1.0 - p_time);

                    cr.arc(px, py, size, 0.0, 2.0 * std::f64::consts::PI);
                    // Glowing reddish/orange particles
                    cr.set_source_rgba(251.0 / 255.0, 113.0 / 255.0, 133.0 / 255.0, alpha);
                    let _ = cr.fill();
                }
            }
            CleanState::CleanFinished { .. } => {
                draw_neon(cr, 0.0, 2.0 * std::f64::consts::PI, 16.0 / 255.0, 185.0 / 255.0, 129.0 / 255.0); // Green Neon
            }
        }
    });

    // Connect button click handler
    let action_btn_c = action_btn.clone();
    let percent_label_c = percent_label.clone();
    let size_label_c = size_label.clone();
    let log_label_c = log_label.clone();
    let total_reclaimable_c = total_reclaimable.clone();
    let state_c = state.clone();
    let progress_drawing_c = progress_drawing.clone();

    let val1_c = val1.clone();
    let val2_c = val2.clone();
    let val3_c = val3.clone();
    let val4_c = val4.clone();
    let user_size_c = user_size.clone();
    let pacman_size_c = pacman_size.clone();
    let journal_size_c = journal_size.clone();
    let trash_size_c = trash_size.clone();

    action_btn.connect_clicked(move |btn| {
        let label = btn.label().unwrap_or_default().to_string();
        let scan_label_str = babydra_common::i18n::t("control.scan");
        let free_label_str = babydra_common::i18n::t("control.free");

        if label == scan_label_str {
            // Trigger Scan
            btn.set_sensitive(false);
            btn.set_label(&babydra_common::i18n::t("control.scanning"));
            
            percent_label_c.remove_css_class("cleaning");
            percent_label_c.remove_css_class("finished");
            percent_label_c.add_css_class("scanning");
            percent_label_c.set_text("SCAN");
            size_label_c.set_text("...");
            log_label_c.set_text("Analyzing disk space...");

            val1_c.set_text("Scanning...");
            val2_c.set_text("Pending...");
            val3_c.set_text("Pending...");
            val4_c.set_text("Pending...");

            *state_c.borrow_mut() = CleanState::Scanning { angle: 0.0 };
            progress_drawing_c.queue_draw();

            // Start Scanning Animation Loop
            let state_loop = state_c.clone();
            let drawing_loop = progress_drawing_c.clone();
            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                let mut state_borrow = state_loop.borrow_mut();
                if let CleanState::Scanning { angle } = *state_borrow {
                    *state_borrow = CleanState::Scanning { angle: (angle + 0.08) % (2.0 * std::f64::consts::PI) };
                    drawing_loop.queue_draw();
                    gtk4::glib::ControlFlow::Continue
                } else {
                    gtk4::glib::ControlFlow::Break
                }
            });

            // Trigger Background Scan Thread
            let (tx, mut rx) = mpsc::unbounded_channel::<CleanProgress>();
            std::thread::spawn(move || {
                let mut total = 0u64;

                let _ = tx.send(CleanProgress::Log("Analyzing user cache...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let u_sz = cache::get_user_cache_size();
                total += u_sz;
                let _ = tx.send(CleanProgress::ScanStep { step: 0, size: u_sz });

                let _ = tx.send(CleanProgress::Log("Checking package cache...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let p_sz = cache::get_pacman_cache_size();
                total += p_sz;
                let _ = tx.send(CleanProgress::ScanStep { step: 1, size: p_sz });

                let _ = tx.send(CleanProgress::Log("Reading system log size...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let j_sz = logs::get_journal_logs_size();
                total += j_sz;
                let _ = tx.send(CleanProgress::ScanStep { step: 2, size: j_sz });

                let _ = tx.send(CleanProgress::Log("Checking trash bin...".to_string()));
                std::thread::sleep(std::time::Duration::from_millis(300));
                let t_sz = temp::get_trash_size();
                total += t_sz;
                let _ = tx.send(CleanProgress::ScanStep { step: 3, size: t_sz });

                let _ = tx.send(CleanProgress::ScanFinished { total_bytes: total });
            });

            let action_btn_inner = action_btn_c.clone();
            let percent_label_inner = percent_label_c.clone();
            let size_label_inner = size_label_c.clone();
            let log_inner = log_label_c.clone();
            let total_inner = total_reclaimable_c.clone();
            let state_inner = state_c.clone();
            let progress_drawing_inner = progress_drawing_c.clone();

            let val1_inner = val1_c.clone();
            let val2_inner = val2_c.clone();
            let val3_inner = val3_c.clone();
            let val4_inner = val4_c.clone();
            let user_size_inner = user_size_c.clone();
            let pacman_size_inner = pacman_size_c.clone();
            let journal_size_inner = journal_size_c.clone();
            let trash_size_inner = trash_size_c.clone();

            glib::spawn_future_local(async move {
                while let Some(progress) = rx.recv().await {
                    match progress {
                        CleanProgress::Log(msg) => {
                            log_inner.set_text(&msg);
                        }
                        CleanProgress::ScanStep { step, size } => {
                            match step {
                                0 => {
                                    *user_size_inner.borrow_mut() = size;
                                    val1_inner.set_text(&format_bytes(size));
                                    val2_inner.set_text("Scanning...");
                                }
                                1 => {
                                    *pacman_size_inner.borrow_mut() = size;
                                    val2_inner.set_text(&format_bytes(size));
                                    val3_inner.set_text("Scanning...");
                                }
                                2 => {
                                    *journal_size_inner.borrow_mut() = size;
                                    val3_inner.set_text(&format_bytes(size));
                                    val4_inner.set_text("Scanning...");
                                }
                                3 => {
                                    *trash_size_inner.borrow_mut() = size;
                                    val4_inner.set_text(&format_bytes(size));
                                }
                                _ => {}
                            }
                        }
                        CleanProgress::ScanFinished { total_bytes } => {
                            log_inner.set_text("Scan complete");
                            *total_inner.borrow_mut() = total_bytes;
                            action_btn_inner.set_sensitive(true);

                            percent_label_inner.remove_css_class("scanning");
                            
                            if total_bytes > 0 {
                                *state_inner.borrow_mut() = CleanState::ScanFinished { total_bytes };
                                percent_label_inner.set_text("READY");
                                size_label_inner.set_text(&format_bytes(total_bytes));
                                
                                let size_str = format_bytes(total_bytes);
                                let desc = babydra_common::i18n::t("control.bytes_can_be_freed")
                                    .replace("{}", &size_str);
                                log_inner.set_text(&desc);
                                action_btn_inner.set_label(&babydra_common::i18n::t("control.free"));
                            } else {
                                *state_inner.borrow_mut() = CleanState::Idle;
                                percent_label_inner.set_text("READY");
                                size_label_inner.set_text("0 B");
                                log_inner.set_text(&babydra_common::i18n::t("control.nothing_to_free"));
                                action_btn_inner.set_label(&babydra_common::i18n::t("control.scan"));
                            }
                            progress_drawing_inner.queue_draw();
                        }
                        _ => {}
                    }
                }
            });
        } else if label == free_label_str {
            // Trigger Clean
            btn.set_sensitive(false);
            btn.set_label("Cleaning...");
            
            percent_label_c.remove_css_class("scanning");
            percent_label_c.remove_css_class("finished");
            percent_label_c.add_css_class("cleaning");
            percent_label_c.set_text("0%");
            
            log_label_c.set_text("Running file cleanup...");

            let total_bytes = *total_reclaimable_c.borrow();
            *state_c.borrow_mut() = CleanState::Cleaning {
                progress: 0.0,
                total_bytes,
                is_backend_done: false,
                actual_freed: 0,
            };
            progress_drawing_c.queue_draw();

            // Background Clean Thread
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

            // Smooth Clean Animation Timer Loop
            let state_loop = state_c.clone();
            let drawing_loop = progress_drawing_c.clone();
            let percent_lbl_loop = percent_label_c.clone();
            let size_lbl_loop = size_label_c.clone();
            let action_btn_loop = action_btn_c.clone();
            let log_lbl_loop = log_label_c.clone();

            let val1_loop = val1_c.clone();
            let val2_loop = val2_c.clone();
            let val3_loop = val3_c.clone();
            let val4_loop = val4_c.clone();
            let user_size_loop = user_size_c.clone();
            let pacman_size_loop = pacman_size_c.clone();
            let journal_size_loop = journal_size_c.clone();
            let trash_size_loop = trash_size_c.clone();

            gtk4::glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
                let mut state_borrow = state_loop.borrow_mut();
                if let CleanState::Cleaning { progress, total_bytes, is_backend_done, actual_freed } = *state_borrow {
                    let new_progress = if is_backend_done {
                        (progress + 0.02).min(1.0)
                    } else {
                        (progress + 0.008).min(0.95)
                    };

                    if new_progress >= 1.0 {
                        *state_borrow = CleanState::CleanFinished { total_bytes: actual_freed };
                        
                        percent_lbl_loop.remove_css_class("cleaning");
                        percent_lbl_loop.add_css_class("finished");
                        percent_lbl_loop.set_text("DONE");
                        size_lbl_loop.set_text("0 B");

                        val1_loop.set_text("0 B");
                        val2_loop.set_text("0 B");
                        val3_loop.set_text("0 B");
                        val4_loop.set_text("0 B");

                        let size_str = format_bytes(actual_freed);
                        let success_msg = babydra_common::i18n::t("control.freed_success")
                            .replace("{}", &size_str);
                        log_lbl_loop.set_text(&success_msg);

                        action_btn_loop.set_sensitive(true);
                        action_btn_loop.set_label(&babydra_common::i18n::t("control.scan"));
                        
                        drawing_loop.queue_draw();
                        gtk4::glib::ControlFlow::Break
                    } else {
                        *state_borrow = CleanState::Cleaning {
                            progress: new_progress,
                            total_bytes,
                            is_backend_done,
                            actual_freed,
                        };

                        let pct = (new_progress * 100.0) as u32;
                        percent_lbl_loop.set_text(&format!("{}%", pct));

                        let remaining_bytes = ((1.0 - new_progress) * total_bytes as f64) as u64;
                        size_lbl_loop.set_text(&format_bytes(remaining_bytes));

                        // Dynamic countdown for each list item
                        let u_sz = *user_size_loop.borrow();
                        let p_sz = *pacman_size_loop.borrow();
                        let j_sz = *journal_size_loop.borrow();
                        let t_sz = *trash_size_loop.borrow();

                        val1_loop.set_text(&format_bytes(((1.0 - new_progress) * u_sz as f64) as u64));
                        val2_loop.set_text(&format_bytes(((1.0 - new_progress) * p_sz as f64) as u64));
                        val3_loop.set_text(&format_bytes(((1.0 - new_progress) * j_sz as f64) as u64));
                        val4_loop.set_text(&format_bytes(((1.0 - new_progress) * t_sz as f64) as u64));

                        drawing_loop.queue_draw();
                        gtk4::glib::ControlFlow::Continue
                    }
                } else {
                    gtk4::glib::ControlFlow::Break
                }
            });

            // Receive from Clean thread to update backend status
            let state_inner = state_c.clone();
            let log_inner = log_label_c.clone();
            glib::spawn_future_local(async move {
                while let Some(progress) = rx.recv().await {
                    match progress {
                        CleanProgress::Log(msg) => {
                            log_inner.set_text(&msg);
                        }
                        CleanProgress::CleanFinished { total_bytes } => {
                            let mut state_borrow = state_inner.borrow_mut();
                            if let CleanState::Cleaning { progress, total_bytes: tb, .. } = *state_borrow {
                                *state_borrow = CleanState::Cleaning {
                                    progress,
                                    total_bytes: tb,
                                    is_backend_done: true,
                                    actual_freed: total_bytes,
                                };
                            }
                        }
                        _ => {}
                    }
                }
            });
        }
    });

    popover_box
}
