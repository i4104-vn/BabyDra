pub mod render;

use babydra_core::models::keybind::Keybind;
use gtk4::prelude::*;
use gtk4::Widget;

/// Creates a new `keybinds widget`.
pub fn create_keybinds() -> Widget {
    let keybinds = vec![
        Keybind {
            id: 1,
            bind_type: "bind".to_string(),
            modifiers: "SUPER".to_string(),
            key: "Q".to_string(),
            dispatcher: "exec".to_string(),
            args: "kitty".to_string(),
        },
        Keybind {
            id: 2,
            bind_type: "bind".to_string(),
            modifiers: "SUPER".to_string(),
            key: "C".to_string(),
            dispatcher: "killactive".to_string(),
            args: "".to_string(),
        },
        Keybind {
            id: 3,
            bind_type: "bind".to_string(),
            modifiers: "SUPER".to_string(),
            key: "E".to_string(),
            dispatcher: "exec".to_string(),
            args: "babydra-explore".to_string(),
        },
    ];

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

    widget.container.into()
}
