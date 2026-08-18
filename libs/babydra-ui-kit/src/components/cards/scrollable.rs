use gtk4::prelude::*;

/// Creates a ScrolledWindow + ListBox combo.
pub fn create_scroll_list(css_class: &str) -> (gtk4::ScrolledWindow, gtk4::ListBox) {
    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    if !css_class.is_empty() {
        list_box.add_css_class(css_class);
    }
    scroll.set_child(Some(&list_box));

    (scroll, list_box)
}
