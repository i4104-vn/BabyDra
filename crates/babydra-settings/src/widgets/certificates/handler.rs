use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc;
use babydra_utils::components::modal::PasswordDialog;
use babydra_common::services::system::certificates;
use super::render::CertificatesWidget;

type PendingFileCell = Rc<RefCell<Option<(String, String)>>>;

pub fn reload_cert_list(
    list_box: &gtk4::ListBox,
    auth_dialog: &Rc<PasswordDialog>,
    pending_file: &PendingFileCell,
) {
    // Clear list_box
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let certs = certificates::list_ca_certificates();

    if certs.is_empty() {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");
        row.set_selectable(false);

        let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        placeholder_box.set_valign(gtk4::Align::Center);
        placeholder_box.set_halign(gtk4::Align::Center);
        placeholder_box.set_margin_top(40);
        placeholder_box.set_margin_bottom(40);

        let icon = babydra_utils::ui::icon::get_icon("key", 24);
        icon.set_pixel_size(24);
        placeholder_box.append(&icon);

        let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.cert_no_items")));
        lbl.add_css_class("settings-row-title");
        placeholder_box.append(&lbl);

        row.set_child(Some(&placeholder_box));
        list_box.append(&row);
        return;
    }

    for cert in certs {
        let row = gtk4::ListBoxRow::new();
        row.add_css_class("settings-card-row");

        let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
        hbox.set_margin_top(4);
        hbox.set_margin_bottom(4);
        hbox.set_margin_start(8);
        hbox.set_margin_end(8);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Start);
        icon_badge.set_hexpand(false);

        let shield_icon = babydra_utils::ui::icon::get_icon("shield", 18);
        shield_icon.set_pixel_size(18);
        shield_icon.set_valign(gtk4::Align::Center);
        shield_icon.set_halign(gtk4::Align::Center);
        shield_icon.set_vexpand(true);
        icon_badge.append(&shield_icon);
        hbox.append(&icon_badge);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_valign(gtk4::Align::Center);
        text_box.set_hexpand(true);

        let name_lbl = gtk4::Label::new(Some(&cert.filename));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&name_lbl);

        let path_lbl = gtk4::Label::new(Some(&cert.path));
        path_lbl.add_css_class("settings-row-desc");
        path_lbl.set_halign(gtk4::Align::Start);
        text_box.append(&path_lbl);

        hbox.append(&text_box);

        // Delete Button matching Wi-Fi row icon-btn-circle style
        let del_btn = gtk4::Button::new();
        del_btn.add_css_class("icon-btn-circle");
        del_btn.set_valign(gtk4::Align::Center);
        del_btn.set_cursor_from_name(Some("pointer"));

        let del_icon = babydra_utils::ui::icon::get_icon("trash", 14);
        del_icon.set_pixel_size(14);
        del_btn.set_child(Some(&del_icon));
        del_btn.set_tooltip_text(Some(&babydra_common::i18n::t("settings.cert_delete")));

        let fname_del = cert.filename.clone();
        let auth_dialog_c = auth_dialog.clone();
        let pending_file_c = pending_file.clone();

        del_btn.connect_clicked(move |_| {
            let fn_rem = fname_del.clone();
            *pending_file_c.borrow_mut() = Some(("delete".to_string(), fn_rem.clone()));
            auth_dialog_c.show_for(
                "Delete Certificate",
                &format!("Enter sudo password to remove '{}' and run update-ca-trust:", fn_rem),
            );
        });

        hbox.append(&del_btn);
        row.set_child(Some(&hbox));
        list_box.append(&row);
    }
}

pub fn wire_events(widget: &CertificatesWidget, auth_dialog: PasswordDialog) {
    let auth_dialog_rc = Rc::new(auth_dialog);
    let pending_file = Rc::new(RefCell::new(None::<(String, String)>));

    // Load initial certificate list
    reload_cert_list(&widget.list_box, &auth_dialog_rc, &pending_file);

    let auth_dialog_add = auth_dialog_rc.clone();
    let pending_file_add = pending_file.clone();
    let container_c = widget.container.clone();

    widget.add_btn.connect_clicked(move |_| {
        let parent_win = container_c.root().and_then(|r| r.downcast::<gtk4::Window>().ok());

        let chooser = gtk4::FileChooserNative::new(
            Some("Select CA Certificate File"),
            parent_win.as_ref(),
            gtk4::FileChooserAction::Open,
            Some("Select"),
            Some("Cancel"),
        );

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Certificate Files (*.crt, *.pem, *.cer, *.der)"));
        filter.add_pattern("*.crt");
        filter.add_pattern("*.pem");
        filter.add_pattern("*.cer");
        filter.add_pattern("*.der");
        chooser.add_filter(&filter);

        let auth_dialog_cb = auth_dialog_add.clone();
        let pending_file_cb = pending_file_add.clone();

        chooser.connect_response(move |dialog, response| {
            if response == gtk4::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(src_path) = file.path() {
                        let fname = src_path.file_name().and_then(|n| n.to_str()).unwrap_or("cert.crt").to_string();
                        let src_str = src_path.to_string_lossy().to_string();

                        *pending_file_cb.borrow_mut() = Some(("add".to_string(), format!("{}:::{}", src_str, fname)));

                        auth_dialog_cb.show_for(
                            "Add CA Certificate",
                            &format!("Enter sudo password to copy '{}' to /etc/ca-certificates/trust-source/anchors/ and run update-ca-trust:", fname),
                        );
                    }
                }
            }
            dialog.destroy();
        });

        chooser.show();
    });

    // Wire PasswordDialog submit
    let list_box_sub = widget.list_box.clone();
    let auth_dialog_sub = auth_dialog_rc.clone();
    let pending_file_sub = pending_file.clone();

    auth_dialog_rc.connect_submit(move |password_opt| {
        let password = match password_opt {
            Some(p) => p,
            None => return,
        };

        let pending = pending_file_sub.borrow_mut().take();
        let (action_type, payload) = match pending {
            Some(p) => p,
            None => return,
        };

        let (tx, rx) = mpsc::channel::<Result<(), String>>();

        std::thread::spawn(move || {
            let res = if action_type == "add" {
                let parts: Vec<&str> = payload.split(":::").collect();
                if parts.len() == 2 {
                    certificates::add_ca_certificate(parts[0], parts[1], &password)
                } else {
                    Err("Invalid certificate path".to_string())
                }
            } else {
                certificates::delete_ca_certificate(&payload, &password)
            };

            let _ = tx.send(res);
        });

        let lb_ref = list_box_sub.clone();
        let auth_ref = auth_dialog_sub.clone();
        let pending_ref = pending_file_sub.clone();

        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(_res) = rx.try_recv() {
                reload_cert_list(&lb_ref, &auth_ref, &pending_ref);
                gtk4::glib::ControlFlow::Break
            } else {
                gtk4::glib::ControlFlow::Continue
            }
        });
    });
}
