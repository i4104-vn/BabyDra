use gtk4::prelude::*;

/// Creates a fully generic icon button reusable across all crates.
pub fn create_icon_button(
    icon_name: &str,
    size: i32,
    css_classes: &[&str],
    tooltip: Option<&str>,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    for cls in css_classes {
        if !cls.is_empty() {
            btn.add_css_class(cls);
        }
    }
    if let Some(tip) = tooltip {
        btn.set_tooltip_text(Some(tip));
    }
    let icon = crate::ui::icon::get_icon(icon_name, size);
    btn.set_child(Some(&icon));
    btn.connect_clicked(move |_| on_click());
    btn
}

/// Creates a generic icon button using a **colored** icon.
pub fn create_colored_icon_button(
    icon_name: &str,
    size: i32,
    color: &str,
    css_classes: &[&str],
    tooltip: Option<&str>,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    for cls in css_classes {
        if !cls.is_empty() {
            btn.add_css_class(cls);
        }
    }
    if let Some(tip) = tooltip {
        btn.set_tooltip_text(Some(tip));
    }
    let icon = crate::ui::icon::get_icon_colored(icon_name, size, color);
    btn.set_child(Some(&icon));
    btn.connect_clicked(move |_| on_click());
    btn
}

/// Creates an icon + label button.
pub fn create_icon_label_button(icon_name: &str, label_text: &str, css_class: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    }
    let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let icon = crate::ui::icon::get_icon(icon_name, 16);
    let label = gtk4::Label::new(Some(label_text));
    content.append(&icon);
    content.append(&label);
    btn.set_child(Some(&content));
    btn
}

/// Creates a generic sidebar-style item button with an icon and label.
pub fn create_sidebar_item_button(
    name: &str,
    icon_name: &str,
    css_class: &str,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    hbox.set_margin_start(10);
    hbox.set_margin_end(10);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    let img = crate::ui::icon::get_icon(icon_name, 18);
    img.set_pixel_size(18);
    img.set_valign(gtk4::Align::Center);
    img.set_halign(gtk4::Align::Center);

    let lbl = gtk4::Label::builder()
        .label(name)
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .hexpand(true)
        .build();

    hbox.append(&img);
    hbox.append(&lbl);

    let btn = gtk4::Button::builder()
        .child(&hbox)
        .css_classes(vec![css_class.to_string(), "flat".to_string()])
        .build();

    btn.connect_clicked(move |_| on_click());
    btn
}

/// Creates a generic sidebar-style item button using a custom widget icon.
pub fn create_sidebar_item_button_with_widget(
    name: &str,
    icon_widget: &impl IsA<gtk4::Widget>,
    css_class: &str,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    hbox.set_margin_start(10);
    hbox.set_margin_end(10);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    icon_widget.set_valign(gtk4::Align::Center);
    icon_widget.set_halign(gtk4::Align::Center);

    let lbl = gtk4::Label::builder()
        .label(name)
        .halign(gtk4::Align::Start)
        .valign(gtk4::Align::Center)
        .hexpand(true)
        .build();

    hbox.append(icon_widget);
    hbox.append(&lbl);

    let btn = gtk4::Button::builder()
        .child(&hbox)
        .css_classes(vec![css_class.to_string(), "flat".to_string()])
        .build();

    btn.connect_clicked(move |_| on_click());
    btn
}

/// Dynamic Wi-Fi signal waves icon widget (0 to 4 wave bars).
pub fn create_wifi_signal_icon(size: i32) -> gtk4::Widget {
    crate::components::wifi::create_system_wifi_signal_icon(size, None)
}

/// Dynamic battery level % icon widget (matches Cairo battery card style).
pub fn create_battery_percentage_icon(_size: i32) -> gtk4::Widget {
    let (pct, is_charging) = if let Some(info) = babydra_common::services::system::battery::get_battery_info() {
        (info.percentage, info.is_charging)
    } else {
        (100, false)
    };

    crate::ui::battery::create_battery_drawing_area(pct, is_charging, 22, 12).upcast()
}

/// VPN shield icon widget with a small lock overlay at the bottom-right corner when connected.
pub fn create_vpn_shield_icon(size: i32) -> gtk4::Widget {
    let is_connected = babydra_common::services::system::vpn::get_vpn_connections()
        .iter()
        .any(|c| c.active);

    let shield_color = if is_connected { "#8B5CF6" } else { "#6B7280" };

    let shield_svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="{shield_color}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
        </svg>"##
    );

    let main_img = crate::ui::icon::get_icon_from_svg(&shield_svg, size);

    if is_connected {
        let overlay = gtk4::Overlay::new();
        overlay.set_child(Some(&main_img));

        let lock_svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 24 24" fill="#10B981" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
            </svg>"##
        );
        let lock_img = crate::ui::icon::get_icon_from_svg(&lock_svg, 10);
        lock_img.set_halign(gtk4::Align::End);
        lock_img.set_valign(gtk4::Align::End);
        lock_img.set_margin_end(0);
        lock_img.set_margin_bottom(0);
        overlay.add_overlay(&lock_img);
        overlay.upcast()
    } else {
        main_img.upcast()
    }
}

/// Small wallpaper thumbnail preview icon for Wallpaper & Themes section.
pub fn create_wallpaper_thumbnail_icon(size: i32) -> gtk4::Widget {
    if let Some(wp_path) = babydra_common::services::wallpaper::get_current_wallpaper() {
        if wp_path.exists() {
            if let Ok(orig) = gdk_pixbuf::Pixbuf::from_file(&wp_path) {
                let w = orig.width();
                let h = orig.height();
                let square_size = w.min(h);
                let x = (w - square_size) / 2;
                let y = (h - square_size) / 2;

                let cropped = orig.new_subpixbuf(x, y, square_size, square_size);
                if let Some(scaled) = cropped.scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear) {
                    if let Ok(circle_pb) = scaled.add_alpha(false, 0, 0, 0) {
                        let width = circle_pb.width();
                        let height = circle_pb.height();
                        let rowstride = circle_pb.rowstride() as usize;
                        let n_channels = circle_pb.n_channels() as usize;

                        let center = (width as f64 - 1.0) / 2.0;
                        let radius_sq = (width as f64 / 2.0) * (width as f64 / 2.0);

                        unsafe {
                            let pixels = circle_pb.pixels();
                            for py in 0..height {
                                let dy = py as f64 - center;
                                for px in 0..width {
                                    let dx = px as f64 - center;
                                    if dx * dx + dy * dy > radius_sq {
                                        let idx = (py as usize) * rowstride + (px as usize) * n_channels;
                                        if idx + 3 < pixels.len() {
                                            pixels[idx + 3] = 0;
                                        }
                                    }
                                }
                            }
                        }

                        let texture = gdk4::Texture::for_pixbuf(&circle_pb);
                        let img = gtk4::Image::from_paintable(Some(&texture));
                        img.set_pixel_size(size);
                        img.add_css_class("sidebar-wallpaper-thumb");
                        return img.upcast();
                    }
                }
            }
        }
    }

    // Fallback if no wallpaper or failed to load
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="#EC4899" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="9"/>
            <circle cx="8.5" cy="8.5" r="1.5"/>
            <path d="M20 15l-4-4-9 9"/>
        </svg>"##
    );
    let icon_w = crate::ui::icon::get_icon_from_svg(&svg, size);
    icon_w.add_css_class("sidebar-wallpaper-thumb");
    icon_w.upcast()
}

/// Generic colored SVG icon builder for standard icons using embedded theme assets.
pub fn create_colored_icon_widget(icon_name: &str, size: i32, color_hex: &str) -> gtk4::Widget {
    crate::ui::icon::get_icon_colored(icon_name, size, color_hex).upcast()
}


