pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;
use babydra_common::models::system_update::PackageUpdate;

pub fn create_system_update_widget() -> Widget {
    let updates = babydra_common::services::system::updates::check_updates().unwrap_or_default();
    let widget = render::build(&updates);

    let list_box = widget.list_box.clone();
    let count_badge = widget.count_badge.clone();
    let spinner = widget.spinner.clone();
    let refresh_btn = widget.refresh_btn.clone();

    widget.refresh_btn.connect_clicked(move |_| {
        spinner.set_visible(true);
        spinner.start();
        refresh_btn.set_sensitive(false);

        let list_box = list_box.clone();
        let count_badge = count_badge.clone();
        let spinner = spinner.clone();
        let refresh_btn = refresh_btn.clone();

        let (tx, rx) = std::sync::mpsc::channel::<Vec<PackageUpdate>>();

        std::thread::spawn(move || {
            let updates = babydra_common::services::system::updates::check_updates().unwrap_or_default();
            let _ = tx.send(updates);
        });

        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Ok(updates) = rx.try_recv() {
                let count_text = if updates.is_empty() {
                    babydra_common::i18n::t("settings.up_to_date")
                } else {
                    format!("{} {}", updates.len(), babydra_common::i18n::t("settings.updates_available"))
                };
                count_badge.set_text(&count_text);

                while let Some(child) = list_box.first_child() {
                    list_box.remove(&child);
                }

                if updates.is_empty() {
                    list_box.append(&render::create_empty_up_to_date_row());
                } else {
                    for pkg in &updates {
                        list_box.append(&render::create_update_row(pkg));
                    }
                }

                spinner.stop();
                spinner.set_visible(false);
                refresh_btn.set_sensitive(true);

                glib::ControlFlow::Break
            } else {
                glib::ControlFlow::Continue
            }
        });
    });

    widget.update_all_btn.connect_clicked(move |_| {
        let _ = babydra_common::services::system::updates::update_system();
    });

    widget.container.into()
}
