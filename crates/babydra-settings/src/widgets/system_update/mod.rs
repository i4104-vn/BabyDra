pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;

pub fn create_system_update_widget() -> Widget {
    let updates = babydra_common::services::system::updates::check_updates().unwrap_or_default();
    let widget = render::build(&updates);

    widget.update_all_btn.connect_clicked(move |_| {
        let _ = babydra_common::services::system::updates::update_system();
    });

    widget.container.into()
}
