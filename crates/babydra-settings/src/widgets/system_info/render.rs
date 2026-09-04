//! System specifications UI layout generator matching reference design Image 1.

use babydra_ui_kit::components::{ChangeHostnameDialog, ChangeNameDialog, ChangePasswordDialog};
use gtk4::prelude::*;

#[derive(Clone)]
pub struct SystemInfoLabels {
    pub os_label: gtk4::Label,
    pub sub_label: gtk4::Label,
    pub uptime_lbl: gtk4::Label,
    pub kernel_lbl: gtk4::Label,
    pub cpu_lbl: gtk4::Label,
    pub mem_lbl: gtk4::Label,
    pub gpu_lbl: gtk4::Label,
}

pub struct SystemInfoWidgets {
    pub root: gtk4::Overlay,
    pub labels: SystemInfoLabels,
    pub edit_host_btn: gtk4::Button,
    pub display_name_lbl: gtk4::Label,
    pub change_name_btn: gtk4::Button,
    pub change_pwd_btn: gtk4::Button,
    pub change_name_dialog: ChangeNameDialog,
    pub change_hostname_dialog: ChangeHostnameDialog,
    pub change_password_dialog: ChangePasswordDialog,
}

/// Builds the system information settings page UI.
pub fn build_system_ui(
    hostname: &str,
    os_name: &str,
    kernel_version: &str,
    cpu_model: &str,
    gpu_info: &str,
    memory_text: &str,
    uptime_text: &str,
    cpu_arch: &str,
) -> SystemInfoWidgets {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    // Page Title
    let page_title = gtk4::Label::new(Some(&babydra_core::i18n::trans("settings.about_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(gtk4::Align::Start);
    main_box.append(&page_title);

    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);

    // ── Card 1: Top Hero Card (Avatar, OS Title, Uptime Badge) ──
    let hero_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 20);
    hero_card.add_css_class("glass-panel");
    hero_card.set_margin_top(4);
    hero_card.set_margin_bottom(2);
    hero_card.set_margin_start(4);
    hero_card.set_margin_end(4);

    // Mascot / Avatar Container
    let avatar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    avatar_box.add_css_class("hero-avatar-box");
    avatar_box.set_size_request(80, 80);
    avatar_box.set_valign(gtk4::Align::Center);

    let avatar_img = babydra_ui_kit::ui::icon::get_icon("logo", 80);
    avatar_img.set_pixel_size(80);
    avatar_img.set_valign(gtk4::Align::Center);
    avatar_img.set_halign(gtk4::Align::Center);
    avatar_box.append(&avatar_img);
    hero_card.append(&avatar_box);

    // Right Side Information
    let info_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    info_box.set_hexpand(true);
    info_box.set_valign(gtk4::Align::Center);

    // Left Column: Hostname + Subtitle
    let text_column = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    text_column.set_hexpand(true);
    text_column.set_valign(gtk4::Align::Center);

    let display_host = if !hostname.is_empty() && hostname != "localhost" {
        hostname
    } else {
        "BabyDra Linux"
    };

    let host_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    host_box.set_valign(gtk4::Align::Center);

    let os_label = gtk4::Label::new(Some(display_host));
    os_label.add_css_class("hero-hostname");
    os_label.set_halign(gtk4::Align::Start);
    host_box.append(&os_label);

    let edit_host_btn = gtk4::Button::from_icon_name("document-edit-symbolic");
    edit_host_btn.add_css_class("connect-pill-btn");
    edit_host_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.change_hostname")));
    edit_host_btn.set_cursor_from_name(Some("pointer"));
    edit_host_btn.set_valign(gtk4::Align::Center);
    host_box.append(&edit_host_btn);

    text_column.append(&host_box);

    // Subtitle Row: OS Name (Architecture) • Kernel Version
    let sub_title = format!("{} ({}) • Kernel {}", os_name, cpu_arch, kernel_version);
    let sub_label = gtk4::Label::new(Some(&sub_title));
    sub_label.add_css_class("hero-subtitle");
    sub_label.set_halign(gtk4::Align::Start);
    text_column.append(&sub_label);

    info_box.append(&text_column);

    // Right Column: Uptime Badge (Top) + EN/VN Button (Bottom, below uptime!)
    let badge_column = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    badge_column.set_valign(gtk4::Align::Center);
    badge_column.set_halign(gtk4::Align::End);

    // Sleek Uptime Badge (e.g. "Up 2h 15m")
    let uptime_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    uptime_badge.add_css_class("hero-uptime-badge");
    uptime_badge.set_valign(gtk4::Align::Center);

    let clock_icon = babydra_ui_kit::ui::icon::get_icon("history", 14);
    clock_icon.set_pixel_size(14);
    clock_icon.set_valign(gtk4::Align::Center);
    uptime_badge.append(&clock_icon);

    let formatted_uptime = babydra_core::i18n::trans("settings.up_time").replace("{}", uptime_text);
    let uptime_lbl = gtk4::Label::new(Some(&formatted_uptime));
    uptime_lbl.add_css_class("hero-uptime-label");
    uptime_lbl.set_valign(gtk4::Align::Center);
    uptime_badge.append(&uptime_lbl);

    badge_column.append(&uptime_badge);

    // Language Segmented Control Pill (EN / VN) below uptime
    let lang_segmented_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 2);
    lang_segmented_box.add_css_class("lang-segmented-control");
    lang_segmented_box.set_halign(gtk4::Align::End);

    let btn_en = gtk4::Button::with_label("EN");
    btn_en.set_cursor_from_name(Some("pointer"));

    let btn_vn = gtk4::Button::with_label("VN");
    btn_vn.set_cursor_from_name(Some("pointer"));

    let current_locale = babydra_core::i18n::get_locale();
    if current_locale == "vi" {
        btn_vn.add_css_class("lang-seg-active");
        btn_en.add_css_class("lang-seg-inactive");
    } else {
        btn_en.add_css_class("lang-seg-active");
        btn_vn.add_css_class("lang-seg-inactive");
    }

    let btn_en_c = btn_en.clone();
    let btn_vn_c = btn_vn.clone();
    btn_en.connect_clicked(move |b| {
        babydra_core::i18n::set_locale("en");
        babydra_core::i18n::persist_locale("en");
        btn_en_c.remove_css_class("lang-seg-inactive");
        btn_en_c.add_css_class("lang-seg-active");
        btn_vn_c.remove_css_class("lang-seg-active");
        btn_vn_c.add_css_class("lang-seg-inactive");
        let _ = b.activate_action("win.rebuild-ui", None);
    });

    let btn_en_c2 = btn_en.clone();
    let btn_vn_c2 = btn_vn.clone();
    btn_vn.connect_clicked(move |b| {
        babydra_core::i18n::set_locale("vi");
        babydra_core::i18n::persist_locale("vi");
        btn_vn_c2.remove_css_class("lang-seg-inactive");
        btn_vn_c2.add_css_class("lang-seg-active");
        btn_en_c2.remove_css_class("lang-seg-active");
        btn_en_c2.add_css_class("lang-seg-inactive");
        let _ = b.activate_action("win.rebuild-ui", None);
    });

    lang_segmented_box.append(&btn_en);
    lang_segmented_box.append(&btn_vn);
    badge_column.append(&lang_segmented_box);
    info_box.append(&badge_column);

    hero_card.append(&info_box);
    content_box.append(&hero_card);

    // ── Card 2: User Account Card (Avatar, Name, Username, Change Name, Change Password) ──
    let user_info = babydra_core::services::system::account::get_user_account_info();

    let account_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 16);
    account_card.add_css_class("glass-panel");
    account_card.set_margin_start(4);
    account_card.set_margin_end(4);

    let user_avatar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    user_avatar_box.add_css_class("blue-icon-badge");
    user_avatar_box.set_size_request(48, 48);
    user_avatar_box.set_valign(gtk4::Align::Center);

    let user_icon_img = babydra_ui_kit::ui::icon::get_icon("user", 22);
    user_icon_img.set_pixel_size(22);
    user_icon_img.set_vexpand(true);
    user_icon_img.set_hexpand(true);
    user_icon_img.set_valign(gtk4::Align::Center);
    user_icon_img.set_halign(gtk4::Align::Center);
    user_avatar_box.append(&user_icon_img);
    account_card.append(&user_avatar_box);

    let user_text_col = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    user_text_col.set_hexpand(true);
    user_text_col.set_valign(gtk4::Align::Center);

    let display_name_lbl = gtk4::Label::new(Some(&user_info.display_name));
    display_name_lbl.add_css_class("settings-row-title");
    display_name_lbl.set_halign(gtk4::Align::Start);
    user_text_col.append(&display_name_lbl);

    let username_sub = format!("@{}", user_info.username);
    let username_lbl = gtk4::Label::new(Some(&username_sub));
    username_lbl.add_css_class("settings-row-desc");
    username_lbl.set_halign(gtk4::Align::Start);
    user_text_col.append(&username_lbl);

    account_card.append(&user_text_col);

    let acc_actions = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    acc_actions.set_valign(gtk4::Align::Center);
    acc_actions.set_halign(gtk4::Align::End);

    let change_name_btn =
        gtk4::Button::with_label(&babydra_core::i18n::trans("settings.change_name"));
    change_name_btn.add_css_class("connect-pill-btn");
    change_name_btn.set_cursor_from_name(Some("pointer"));
    acc_actions.append(&change_name_btn);

    let change_pwd_btn =
        gtk4::Button::with_label(&babydra_core::i18n::trans("settings.change_password"));
    change_pwd_btn.add_css_class("connect-pill-btn");
    change_pwd_btn.set_cursor_from_name(Some("pointer"));
    acc_actions.append(&change_pwd_btn);

    account_card.append(&acc_actions);
    content_box.append(&account_card);

    // ── 2x2 Grid of Hardware Spec Cards ─────────────────────────
    let grid = gtk4::Grid::new();
    grid.set_column_spacing(20);
    grid.set_row_spacing(20);
    grid.set_column_homogeneous(true);

    let kernel_lbl = gtk4::Label::new(Some(kernel_version));
    let cpu_lbl = gtk4::Label::new(Some(cpu_model));
    let mem_lbl = gtk4::Label::new(Some(memory_text));
    let gpu_lbl = gtk4::Label::new(Some(gpu_info));

    let specs: Vec<(&str, String, &gtk4::Label)> = vec![
        (
            "cog",
            babydra_core::i18n::trans("settings.spec_kernel"),
            &kernel_lbl,
        ),
        (
            "sliders",
            babydra_core::i18n::trans("settings.spec_processor"),
            &cpu_lbl,
        ),
        (
            "history",
            babydra_core::i18n::trans("settings.spec_memory"),
            &mem_lbl,
        ),
        (
            "palette",
            babydra_core::i18n::trans("settings.spec_graphics"),
            &gpu_lbl,
        ),
    ];

    for (idx, (icon_name, label, value_widget)) in specs.into_iter().enumerate() {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
        card.add_css_class("spec-card");
        card.set_halign(gtk4::Align::Fill);
        card.set_valign(gtk4::Align::Fill);

        // Centered Blue Icon Badge
        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_halign(gtk4::Align::Center);
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_size_request(44, 44);

        let icon_img = babydra_ui_kit::ui::icon::get_icon(icon_name, 22);
        icon_img.set_pixel_size(22);
        icon_img.set_vexpand(true);
        icon_img.set_hexpand(true);
        icon_img.set_valign(gtk4::Align::Center);
        icon_img.set_halign(gtk4::Align::Center);
        icon_badge.append(&icon_img);
        card.append(&icon_badge);

        // Label
        let label_widget = gtk4::Label::new(Some(label.as_str()));
        label_widget.add_css_class("spec-label");
        label_widget.set_halign(gtk4::Align::Center);
        card.append(&label_widget);

        // Value
        value_widget.add_css_class("spec-value");
        value_widget.set_halign(gtk4::Align::Center);
        value_widget.set_justify(gtk4::Justification::Center);
        value_widget.set_wrap(true);
        card.append(value_widget);

        let col = (idx % 2) as i32;
        let row = (idx / 2) as i32;
        grid.attach(&card, col, row, 1, 1);
    }

    content_box.append(&grid);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&content_box));

    main_box.append(&scroll);

    // Modal dialogs
    let change_name_dialog = ChangeNameDialog::new();
    let change_hostname_dialog = ChangeHostnameDialog::new();
    let change_password_dialog = ChangePasswordDialog::new();

    let root = gtk4::Overlay::new();
    root.set_child(Some(&main_box));
    root.add_overlay(&change_name_dialog.container);
    root.add_overlay(&change_hostname_dialog.container);
    root.add_overlay(&change_password_dialog.container);

    let labels = SystemInfoLabels {
        os_label,
        sub_label,
        uptime_lbl,
        kernel_lbl,
        cpu_lbl,
        mem_lbl,
        gpu_lbl,
    };

    SystemInfoWidgets {
        root,
        labels,
        edit_host_btn,
        display_name_lbl,
        change_name_btn,
        change_pwd_btn,
        change_name_dialog,
        change_hostname_dialog,
        change_password_dialog,
    }
}
