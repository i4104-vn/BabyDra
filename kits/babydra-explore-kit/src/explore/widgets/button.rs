use babydra_core::i18n::t;
use gtk4::prelude::*;
use gtk4::Button;

/// Dynamically morphs the button layout/icons between "New Folder" and "Empty Trash".
pub fn update_new_folder_button(btn: &Button, is_in_trash: bool) {
    if is_in_trash {
        if let Some(box_child) = btn.child().and_downcast::<gtk4::Box>() {
            if let Some(img) = box_child.first_child().and_downcast::<gtk4::Image>() {
                babydra_ui_kit::ui::icon::set_image_from_icon(&img, "user-trash-full-symbolic", 16);
            }
            if let Some(lbl) = box_child
                .first_child()
                .and_then(|w| w.next_sibling())
                .and_downcast::<gtk4::Label>()
            {
                lbl.set_text(&t("explore.empty_trash"));
            }
        }
        btn.add_css_class("empty-trash-btn");
    } else {
        if let Some(box_child) = btn.child().and_downcast::<gtk4::Box>() {
            if let Some(img) = box_child.first_child().and_downcast::<gtk4::Image>() {
                babydra_ui_kit::ui::icon::set_image_from_icon(&img, "folder-new-symbolic", 16);
            }
            if let Some(lbl) = box_child
                .first_child()
                .and_then(|w| w.next_sibling())
                .and_downcast::<gtk4::Label>()
            {
                lbl.set_text(&t("explore.new_folder"));
            }
        }
        btn.remove_css_class("empty-trash-btn");
    }
}
