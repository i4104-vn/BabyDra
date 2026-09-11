use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn run_scale_allocation() {
    if gtk4::init().is_err() {
        return;
    }
    std::env::set_var("BABYDRA_THEMES_DIR", repo_root().join("themes"));
    babydra_ui_kit::ui::theme::init_theme();

    let card =
        babydra_ui_kit::components::create_collapsible_card("Audio Output", None, None, true);
    let (vol_hdr, _val_label) = babydra_ui_kit::components::create_slider_header("Volume", 45.0);

    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);

    let mute_icon_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    mute_icon_box.set_valign(gtk4::Align::Center);
    let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored("volume", 16, "#ffffff");
    icon_widget.add_css_class("slider-icon");
    mute_icon_box.append(&icon_widget);

    let mute_btn = gtk4::Button::new();
    mute_btn.add_css_class("slider-overlay-mute-btn");
    mute_btn.set_child(Some(&mute_icon_box));
    mute_btn.set_halign(gtk4::Align::Start);
    mute_btn.set_valign(gtk4::Align::Center);
    mute_btn.set_margin_start(10);
    mute_btn.set_can_focus(false);

    let scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.set_value(45.0);
    scale.set_hexpand(true);
    scale.set_draw_value(false);
    scale.add_css_class("control-slider");

    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_child(Some(&scale));
    overlay.add_overlay(&mute_btn);

    row_box.append(&overlay);

    let vol_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    vol_box.append(&vol_hdr);
    vol_box.append(&row_box);

    card.content.append(&vol_box);

    let window = gtk4::Window::new();
    window.add_css_class("settings-window");
    let prov = gtk4::CssProvider::new();
    prov.load_from_data("window.settings-window { background-color: #12121c; } .settings-card { background-color: #1e1e2a; }");
    window
        .style_context()
        .add_provider(&prov, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 10);
    window.set_default_size(500, 300);
    window.set_child(Some(&card.container));
    window.present();

    let (w_min, w_nat, _, _) = scale.measure(gtk4::Orientation::Horizontal, -1);
    let (ov_w_min, ov_w_nat, _, _) = overlay.measure(gtk4::Orientation::Horizontal, -1);
    let (row_w_min, row_w_nat, _, _) = row_box.measure(gtk4::Orientation::Horizontal, -1);
    let (card_w_min, card_w_nat, _, _) = card.content.measure(gtk4::Orientation::Horizontal, -1);

    println!("SCALE MEASURE H: min={}, nat={}", w_min, w_nat);
    println!("OVERLAY MEASURE H: min={}, nat={}", ov_w_min, ov_w_nat);
    println!("ROW_BOX MEASURE H: min={}, nat={}", row_w_min, row_w_nat);
    println!("CARD MEASURE H: min={}, nat={}", card_w_min, card_w_nat);
    let ctx = gtk4::glib::MainContext::default();
    for _ in 0..10 {
        ctx.iteration(false);
    }

    println!(
        "AFTER ITERATION scale width: {}, height: {}",
        scale.width(),
        scale.height()
    );
    println!(
        "AFTER ITERATION overlay width: {}, height: {}",
        overlay.width(),
        overlay.height()
    );
    println!(
        "AFTER ITERATION row_box width: {}, height: {}",
        row_box.width(),
        row_box.height()
    );

    let snapshot = gtk4::Snapshot::new();
    let child = window.child().unwrap();
    window.snapshot_child(&child, &snapshot);
    if let Some(node) = snapshot.to_node() {
        let renderer = window.renderer().or_else(|| {
            window
                .surface()
                .and_then(|surface| gtk4::gsk::Renderer::for_surface(&surface))
        });
        println!("Renderer: {:?}", renderer.is_some());
        if let Some(r) = renderer {
            let texture = r.render_texture(&node, None);
            let _ = texture.save_to_png("/tmp/rendered_card.png");
            println!("Saved snapshot to /tmp/rendered_card.png");
        }
    }
}

fn run_gtk_css_parsing() {
    if gtk4::init().is_err() {
        println!("Skipping GTK test if display unavailable");
        return;
    }

    std::env::set_var("BABYDRA_THEMES_DIR", repo_root().join("themes"));

    let files = [
        "libs/babydra-ui-kit/src/styles/shared/panel/panel.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/workspaces.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/clock.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/status.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/system_monitor.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/tray.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/taskbar.css",
        "libs/babydra-ui-kit/src/styles/shared/control_center/control_center.css",
        "libs/babydra-ui-kit/src/styles/shared/control_center/power.css",
        "libs/babydra-ui-kit/src/styles/shared/island/system_island.css",
        "libs/babydra-ui-kit/src/styles/shared/island/notification.css",
        "libs/babydra-ui-kit/src/styles/shared/launcher/launcher.css",
        "libs/babydra-ui-kit/src/styles/shared/calendar/calendar.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/button.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/sidebar.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/screenshot.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/lock.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/preview.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/settings.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/switcher.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/window.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/header_bar.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/content_view.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/info_panel.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/status_bar.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/context_menu.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/dialogs.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/dialog.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/scrollbar.css",
        "themes/babydra-default/css/dark.css",
        "themes/babydra-default/css/light.css",
        "themes/babydra-default/css/theme.css",
        "themes/babydra-blue/css/dark.css",
        "themes/babydra-blue/css/light.css",
        "themes/babydra-blue/css/theme.css",
        "themes/babydra-green/css/dark.css",
        "themes/babydra-green/css/light.css",
        "themes/babydra-green/css/theme.css",
        "themes/babydra-purple/css/dark.css",
        "themes/babydra-purple/css/light.css",
        "themes/babydra-purple/css/theme.css",
        "themes/babydra-rose/css/dark.css",
        "themes/babydra-rose/css/light.css",
        "themes/babydra-rose/css/theme.css",
    ];

    let mut total_errors = 0;

    for rel_path in &files {
        let full_path = repo_root().join(rel_path);
        let content = std::fs::read_to_string(&full_path)
            .unwrap_or_else(|e| panic!("Could not read {}: {}", rel_path, e));
        let provider = gtk4::CssProvider::new();
        let errors = Rc::new(RefCell::new(Vec::new()));
        let errors_clone = errors.clone();

        let rel_path_str = rel_path.to_string();
        provider.connect_parsing_error(move |_prov, section, err| {
            let start = section.start_location();
            errors_clone.borrow_mut().push(format!(
                "  [{}:{}:{}] {}",
                rel_path_str,
                start.lines() + 1,
                start.line_chars() + 1,
                err
            ));
        });

        provider.load_from_data(&content);

        let errs = errors.borrow();
        if !errs.is_empty() {
            println!("Errors in {}:", rel_path);
            for e in errs.iter() {
                println!("{}", e);
            }
            total_errors += errs.len();
        }
    }

    // Now test resolved themes (concatenated layers)
    let theme_names = [
        "babydra-default",
        "babydra-blue",
        "babydra-green",
        "babydra-purple",
        "babydra-rose",
    ];
    for theme_name in &theme_names {
        let theme = babydra_theme::resolve_theme(theme_name).expect("Failed to resolve theme");

        for (mode, color_css) in [("dark", &theme.dark_css), ("light", &theme.light_css)] {
            // Concatenate all structural files + color layer + extra layer
            let mut full_css = String::new();
            for rel_path in &files[0..28] {
                let p = repo_root().join(rel_path);
                full_css.push_str(&std::fs::read_to_string(&p).unwrap());
                full_css.push('\n');
            }
            full_css.push_str(color_css);
            full_css.push('\n');
            full_css.push_str(&theme.css_layer);

            let provider = gtk4::CssProvider::new();
            let errors = Rc::new(RefCell::new(Vec::new()));
            let errors_clone = errors.clone();
            let context = format!("theme: {} (mode: {})", theme_name, mode);

            provider.connect_parsing_error(move |_prov, section, err| {
                let start = section.start_location();
                errors_clone.borrow_mut().push(format!(
                    "  [concatenated line {}:{}] {}",
                    start.lines() + 1,
                    start.line_chars() + 1,
                    err
                ));
            });

            provider.load_from_data(&full_css);

            let errs = errors.borrow();
            if !errs.is_empty() {
                println!("Errors in concatenated CSS for {}:", context);
                for e in errs.iter() {
                    println!("{}", e);
                }
                total_errors += errs.len();
            }
        }
    }

    println!("\nTotal CSS parsing errors found: {}", total_errors);
    assert_eq!(
        total_errors, 0,
        "Found {} CSS parsing errors in GTK stylesheets",
        total_errors
    );
}

fn run_render_fluid_cloud_capsule() {
    if gtk4::init().is_err() {
        return;
    }
    std::env::set_var("BABYDRA_THEMES_DIR", repo_root().join("themes"));
    babydra_ui_kit::ui::theme::init_theme();

    // Window with warm wallpaper background matching user's reference image
    let window = gtk4::Window::new();
    window.set_default_size(520, 200);

    let disp = gtk4::gdk::Display::default().unwrap();
    let theme_dark = std::fs::read_to_string(repo_root().join("themes/babydra-default/css/dark.css")).unwrap();
    let island_css = std::fs::read_to_string(repo_root().join("libs/babydra-ui-kit/src/styles/shared/island/system_island.css")).unwrap();
    let custom_css = "
        .wallpaper-mock {
            background: linear-gradient(135deg, #7c2d12 0%, #451a03 50%, #1c1917 100%);
            padding: 36px 40px;
        }
    ";
    let prov = gtk4::CssProvider::new();
    prov.load_from_data(&format!("{}\n{}\n{}", island_css, theme_dark, custom_css));
    gtk4::style_context_add_provider_for_display(&disp, &prov, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION + 100);

    let outer_center = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer_center.add_css_class("wallpaper-mock");
    outer_center.set_valign(gtk4::Align::Center);
    outer_center.set_halign(gtk4::Align::Center);

    // Pill Capsule matching NotificationPopover
    let capsule = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    capsule.add_css_class("notification-popup-box");
    capsule.set_valign(gtk4::Align::Center);
    capsule.set_halign(gtk4::Align::Center);

    let content_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    content_row.set_valign(gtk4::Align::Center);

    // Squircle Icon container
    let icon_box = gtk4::CenterBox::new();
    icon_box.add_css_class("notification-icon-box");
    icon_box.set_size_request(42, 42);
    icon_box.set_valign(gtk4::Align::Center);
    icon_box.set_halign(gtk4::Align::Center);

    let icon_inner = babydra_ui_kit::ui::icon::get_icon_colored("calendar", 22, "#fbbf24");
    icon_inner.add_css_class("notification-icon-img");
    icon_box.set_center_widget(Some(&icon_inner));
    content_row.append(&icon_box);

    // Text stack
    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    text_box.set_valign(gtk4::Align::Center);

    let title_lbl = gtk4::Label::new(Some("日程已添加"));
    title_lbl.add_css_class("notification-title");
    title_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&title_lbl);

    let body_lbl = gtk4::Label::new(Some("4月16日，不见不散"));
    body_lbl.add_css_class("notification-body");
    body_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&body_lbl);

    content_row.append(&text_box);
    capsule.append(&content_row);
    outer_center.append(&capsule);

    window.set_child(Some(&outer_center));
    window.present();

    let ctx = gtk4::glib::MainContext::default();
    for _ in 0..15 {
        ctx.iteration(false);
    }

    let snapshot = gtk4::Snapshot::new();
    let child = window.child().unwrap();
    window.snapshot_child(&child, &snapshot);
    if let Some(node) = snapshot.to_node() {
        let renderer = window.renderer().or_else(|| {
            window
                .surface()
                .and_then(|surface| gtk4::gsk::Renderer::for_surface(&surface))
        });
        if let Some(r) = renderer {
            let texture = r.render_texture(&node, None);
            let _ = texture.save_to_png("/tmp/rendered_fluid_cloud.png");
            println!("Saved Fluid Cloud snapshot to /tmp/rendered_fluid_cloud.png");
        }
    }
}

#[test]
fn test_theme_and_css() {
    run_gtk_css_parsing();
    run_scale_allocation();
    run_render_fluid_cloud_capsule();
}
