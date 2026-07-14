use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, FlowBoxChild, Image, Picture};
use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::{FileEntry, load_cropped_square_pixbuf};

pub fn create_flow_child(
    idx: usize,
    entry: &FileEntry,
    current_path: &PathBuf,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
) -> FlowBoxChild {
    let target_entry = entry.clone();
    let cp = current_path.clone();
    let nav = nav_callback.clone();

    baby_utils::components::create_grid_file_item(
        idx,
        entry,
        move |widget, x, y| {
            crate::widgets::context_menu::show_for_file(
                widget,
                x,
                y,
                target_entry.clone(),
                cp.clone(),
                nav.clone(),
            );
        },
    )
}
