pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;

pub fn create_displays_widget() -> Widget {
    let monitors = babydra_common::services::system::display::get_displays();
    let widget = render::build(&monitors);

    let monitors_clone = monitors.clone();
    widget.save_btn.connect_clicked(move |_| {
        let _ = babydra_common::services::system::display::save_displays(&monitors_clone);
    });

    widget.container.into()
}
