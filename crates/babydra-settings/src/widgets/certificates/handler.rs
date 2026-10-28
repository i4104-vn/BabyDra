use crate::widgets::state::CertificatesWidget;
use babydra_core::services::system::certificates;
use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

type PendingFileCell = Rc<RefCell<Option<(String, String)>>>;

/// Reload cert list into responsive card grid.
pub fn reload_cert_list(
    flow_box: &gtk4::FlowBox,
    stack: &gtk4::Stack,
    auth_dialog: &Rc<PasswordDialog>,
    pending_file: &PendingFileCell,
) {
    crate::widgets::helpers::clear_flow_box(flow_box);

    let certs = certificates::list_ca_certificates();

    if certs.is_empty() {
        stack.set_visible_child_name("empty");
        return;
    }

    stack.set_visible_child_name("cards");

    for cert in certs {
        let card = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
        card.add_css_class("settings-card");
        card.add_css_class("device-card");
        card.add_css_class("cert-card");
        card.set_size_request(290, 136);
        card.set_halign(gtk4::Align::Start);
        card.set_valign(gtk4::Align::Start);

        // ── Top Row: Left icon badge, Right delete action button ──
        let top_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        top_row.set_valign(gtk4::Align::Center);

        let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_valign(gtk4::Align::Center);
        icon_badge.set_halign(gtk4::Align::Start);

        let shield_icon = babydra_ui_kit::ui::icon::get_icon("key", 18);
        shield_icon.set_pixel_size(18);
        shield_icon.set_valign(gtk4::Align::Center);
        shield_icon.set_halign(gtk4::Align::Center);
        shield_icon.set_vexpand(true);
        icon_badge.append(&shield_icon);
        top_row.append(&icon_badge);

        let spacer_top = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        spacer_top.set_hexpand(true);
        top_row.append(&spacer_top);

        // Delete Button
        let del_btn = gtk4::Button::new();
        del_btn.add_css_class("icon-btn");
        del_btn.add_css_class("circular");
        del_btn.add_css_class("delete-btn");
        del_btn.set_valign(gtk4::Align::Center);
        del_btn.set_cursor_from_name(Some("pointer"));

        let del_icon = babydra_ui_kit::ui::icon::get_icon("edit-delete", 16);
        del_icon.set_pixel_size(16);
        del_btn.set_child(Some(&del_icon));
        del_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.cert_delete")));

        let fname_del = cert.filename.clone();
        let auth_dialog_c = auth_dialog.clone();
        let pending_file_c = pending_file.clone();

        del_btn.connect_clicked(move |_| {
            let fn_rem = fname_del.clone();
            *pending_file_c.borrow_mut() = Some(("delete".to_string(), fn_rem.clone()));
            auth_dialog_c.show_for(
                "Delete Certificate",
                &format!(
                    "Enter sudo password to remove '{}' and run update-ca-trust:",
                    fn_rem
                ),
            );
        });
        top_row.append(&del_btn);

        card.append(&top_row);

        // Spacer to push info to bottom
        let mid_spacer = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        mid_spacer.set_vexpand(true);
        card.append(&mid_spacer);

        // ── Bottom Row: Filename + Path on Left, Trusted badge on Right ──
        let bottom_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        bottom_row.set_valign(gtk4::Align::End);

        let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
        text_box.set_valign(gtk4::Align::End);
        text_box.set_halign(gtk4::Align::Start);
        text_box.set_hexpand(true);

        let display_name = if cert.filename.chars().count() > 22 {
            let truncated: String = cert.filename.chars().take(22).collect();
            format!("{}...", truncated)
        } else {
            cert.filename.clone()
        };
        let name_lbl = gtk4::Label::new(Some(&display_name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.add_css_class("device-card-title");
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.set_tooltip_text(Some(&cert.filename));
        text_box.append(&name_lbl);

        let display_path = if cert.path.chars().count() > 28 {
            let truncated: String = cert.path.chars().take(28).collect();
            format!("{}...", truncated)
        } else {
            cert.path.clone()
        };
        let path_lbl = gtk4::Label::new(Some(&display_path));
        path_lbl.add_css_class("settings-row-desc");
        path_lbl.add_css_class("device-card-subtitle");
        path_lbl.set_halign(gtk4::Align::Start);
        path_lbl.set_tooltip_text(Some(&cert.path));
        text_box.append(&path_lbl);

        bottom_row.append(&text_box);

        let status_lbl = gtk4::Label::new(Some("Trusted"));
        status_lbl.add_css_class("device-card-status-on");
        status_lbl.set_valign(gtk4::Align::End);
        bottom_row.append(&status_lbl);

        card.append(&bottom_row);
        flow_box.append(&card);
    }
}

/// Wire events.
pub fn wire_events(widget: &CertificatesWidget, auth_dialog: PasswordDialog) {
    let auth_dialog_rc = Rc::new(auth_dialog);
    let pending_file = Rc::new(RefCell::new(None::<(String, String)>));

    reload_cert_list(
        &widget.flow_box,
        &widget.stack,
        &auth_dialog_rc,
        &pending_file,
    );

    let auth_dialog_add = auth_dialog_rc.clone();
    let pending_file_add = pending_file.clone();
    let container_c = widget.container.clone();

    widget.add_btn.connect_clicked(move |_| {
        let parent_win = container_c
            .root()
            .and_then(|r| r.downcast::<gtk4::Window>().ok());

        let file_dialog = gtk4::FileDialog::new();
        file_dialog.set_title("Select CA Certificate File");
        file_dialog.set_accept_label(Some("Select"));

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("Certificate Files (*.crt, *.pem, *.cer, *.der)"));
        filter.add_pattern("*.crt");
        filter.add_pattern("*.pem");
        filter.add_pattern("*.cer");
        filter.add_pattern("*.der");

        let filters = gtk4::gio::ListStore::new::<gtk4::FileFilter>();
        filters.append(&filter);
        file_dialog.set_filters(Some(&filters));
        file_dialog.set_default_filter(Some(&filter));

        let auth_dialog_cb = auth_dialog_add.clone();
        let pending_file_cb = pending_file_add.clone();

        file_dialog.open(
            parent_win.as_ref(),
            None::<&gtk4::gio::Cancellable>,
            move |result| {
                if let Ok(file) = result {
                    if let Some(src_path) = file.path() {
                        let fname = src_path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("cert.crt")
                            .to_string();
                        let src_str = src_path.to_string_lossy().to_string();

                        *pending_file_cb.borrow_mut() =
                            Some(("add".to_string(), format!("{}:::{}", src_str, fname)));

                        auth_dialog_cb.show_for(
                            "Add CA Certificate",
                            &format!(
                                "Enter sudo password to copy '{}' to /etc/ca-certificates/trust-source/anchors/ and run update-ca-trust:",
                                fname
                            ),
                        );
                    }
                }
            },
        );
    });

    // Wire PasswordDialog submit
    let flow_box_sub = widget.flow_box.clone();
    let stack_sub = widget.stack.clone();
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

        let (tx, rx) = mpsc::channel::<Result<(), babydra_core::CoreError>>();

        std::thread::spawn(move || {
            let res = if action_type == "add" {
                let parts: Vec<&str> = payload.split(":::").collect();
                if parts.len() == 2 {
                    certificates::add_ca_certificate(parts[0], parts[1], &password)
                } else {
                    Err(babydra_core::CoreError::msg("Invalid certificate path"))
                }
            } else {
                certificates::delete_ca_cert(&payload, &password)
            };

            let _ = tx.send(res);
        });

        let fb_ref = flow_box_sub.clone();
        let st_ref = stack_sub.clone();
        let auth_ref = auth_dialog_sub.clone();
        let pending_ref = pending_file_sub.clone();

        gtk4::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(_res) = rx.try_recv() {
                reload_cert_list(&fb_ref, &st_ref, &auth_ref, &pending_ref);
                gtk4::glib::ControlFlow::Break
            } else {
                gtk4::glib::ControlFlow::Continue
            }
        });
    });
}
