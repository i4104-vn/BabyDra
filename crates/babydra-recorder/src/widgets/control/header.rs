//! Header widget for the recorder control window.

use babydra_ui_kit::prelude::*;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Orientation};

/// Builds the header section displaying the application icon, title, and subtitle.
pub fn build_header() -> GtkBox {
    let header_box = GtkBox::new(Orientation::Horizontal, 12);
    header_box.set_valign(Align::Center);

    let icon = babydra_ui_kit::ui::icon::get_icon("camera-video", 32);
    icon.set_pixel_size(32);
    header_box.append(&icon);

    let title_vbox = GtkBox::new(Orientation::Vertical, 2);
    let title = create_title(&babydra_core::i18n::trans("recorder.title"));
    let subtitle = create_subtitle(&babydra_core::i18n::trans("recorder.subtitle"));
    title_vbox.append(&title);
    title_vbox.append(&subtitle);
    header_box.append(&title_vbox);

    header_box
}
