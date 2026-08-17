use gtk4::prelude::*;
use std::path::PathBuf;

/// Dynamically updates the `.cut-item` CSS class on current GTK view items without re-scanning the directory.
pub fn apply_cut_dimming(root_widget: &impl IsA<gtk4::Widget>, cut_paths: &[PathBuf]) {
    apply_cut_class_recursive(root_widget.upcast_ref(), cut_paths);
}

/// Applies cut dimming across all active top-level windows in the application.
pub fn apply_cut_dimming_global(cut_paths: &[PathBuf]) {
    let toplevels = gtk4::Window::toplevels();
    for i in 0..toplevels.n_items() {
        if let Some(item) = toplevels.item(i) {
            if let Ok(widget) = item.downcast::<gtk4::Widget>() {
                apply_cut_class_recursive(&widget, cut_paths);
            }
        }
    }
}

fn apply_cut_class_recursive(widget: &gtk4::Widget, cut_paths: &[PathBuf]) {
    if let Ok(flowbox) = widget.clone().downcast::<gtk4::FlowBox>() {
        let mut child_opt = flowbox.first_child();
        while let Some(child) = child_opt {
            let widget_name = child.widget_name();
            let path = PathBuf::from(widget_name.as_str());
            let is_cut = cut_paths.contains(&path);
            if let Ok(flow_child) = child.clone().downcast::<gtk4::FlowBoxChild>() {
                if let Some(inner) = flow_child.child() {
                    if is_cut {
                        inner.add_css_class("cut-item");
                    } else {
                        inner.remove_css_class("cut-item");
                    }
                }
            }
            child_opt = child.next_sibling();
        }
        return;
    }

    if let Ok(listbox) = widget.clone().downcast::<gtk4::ListBox>() {
        let mut child_opt = listbox.first_child();
        while let Some(child) = child_opt {
            let widget_name = child.widget_name();
            let path = PathBuf::from(widget_name.as_str());
            let is_cut = cut_paths.contains(&path);
            if let Ok(list_row) = child.clone().downcast::<gtk4::ListBoxRow>() {
                if let Some(inner) = list_row.child() {
                    if is_cut {
                        inner.add_css_class("cut-item");
                    } else {
                        inner.remove_css_class("cut-item");
                    }
                }
            }
            child_opt = child.next_sibling();
        }
        return;
    }

    let mut child_opt = widget.first_child();
    while let Some(child) = child_opt {
        apply_cut_class_recursive(&child, cut_paths);
        child_opt = child.next_sibling();
    }
}
