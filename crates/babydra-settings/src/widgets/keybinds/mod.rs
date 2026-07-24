pub mod render;

use gtk4::prelude::*;
use gtk4::Widget;
use babydra_common::models::keybind::Keybind;

pub fn create_keybinds_widget() -> Widget {
    let keybinds = babydra_common::services::system::keybinds::get_keybinds();
    let widget = render::build(&keybinds);

    let parent_card = widget.table_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let types = vec!["bind", "binde", "bindm", "bindl"];
        let mods = vec!["SUPER", "ALT", "CTRL", "SHIFT", "SUPER_SHIFT", "ALT_SHIFT"];
        let empty_kb = Keybind {
            id: 0,
            bind_type: "bind".to_string(),
            modifiers: "SUPER".to_string(),
            key: "".to_string(),
            dispatcher: "exec".to_string(),
            args: "".to_string(),
        };
        let row = render::create_keybind_row(&empty_kb, &types, &mods, parent_card.clone());
        parent_card.append(&row);
    });

    widget.save_btn.connect_clicked(move |_| {
        // Save logic
    });

    widget.container.into()
}
