pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;

pub fn create_system_update_widget() -> Widget {
    let updates = babydra_common::services::system::updates::check_updates().unwrap_or_default();
    let widget = render::build(&updates);

    let list_box = widget.list_box.clone();
    let count_label = widget.count_label.clone();
    let refresh_btn = widget.refresh_btn.clone();
    refresh_btn.connect_clicked(move |_| {
        let updates = babydra_common::services::system::updates::check_updates().unwrap_or_default();
        count_label.set_text(&format!("{}", updates.len()));

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
    });

    widget.update_all_btn.connect_clicked(move |_| {
        let _ = babydra_common::services::system::updates::update_system();
    });

    widget.container.into()
}
