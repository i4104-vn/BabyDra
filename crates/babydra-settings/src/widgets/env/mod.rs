pub mod render;

use gtk4::prelude::*;
use gtk4::{Box, Entry, Widget};
use babydra_common::models::env_var::EnvVar;

pub fn create_env_widget() -> Widget {
    let vars = babydra_common::services::system::env::get_env_vars();
    let widget = render::build(&vars);

    let parent_card = widget.list_box.clone();
    widget.add_btn.connect_clicked(move |_| {
        let empty_var = EnvVar { id: 0, key: "".to_string(), value: "".to_string() };
        let row = render::create_env_row(&empty_var, parent_card.clone());
        parent_card.append(&row);
    });

    let parent_card_save = widget.list_box.clone();
    widget.save_btn.connect_clicked(move |_| {
        let mut save_list = Vec::new();
        let mut id = 1;
        let mut child = parent_card_save.first_child();
        while let Some(c) = child {
            if let Some(row_box) = c.downcast_ref::<Box>() {
                let mut key = String::new();
                let mut val = String::new();
                let mut count = 0;
                let mut sub_child = row_box.first_child();
                while let Some(sc) = sub_child {
                    if let Some(entry) = sc.downcast_ref::<Entry>() {
                        if count == 0 {
                            key = entry.text().to_string();
                            count += 1;
                        } else {
                            val = entry.text().to_string();
                        }
                    }
                    sub_child = sc.next_sibling();
                }
                if !key.trim().is_empty() {
                    save_list.push(EnvVar { id, key, value: val });
                    id += 1;
                }
            }
            child = c.next_sibling();
        }
        let _ = babydra_common::services::system::env::save_env_vars(&save_list);
    });

    widget.container.into()
}
