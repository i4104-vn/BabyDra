use gtk4::{Box, Button, Label, ListBox, Overlay, ScrolledWindow, Spinner, TextBuffer, TextView};

#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
}

#[derive(Clone)]
pub struct SystemUpdateWidget {
    pub root: Overlay,
    pub container: Box,
    pub count_badge: Label,
    pub spinner: Spinner,
    pub update_all_btn: Button,
    pub refresh_btn: Button,
    pub glass_card: Box,
    pub list_box: ListBox,
    pub console_card: Box,
    pub console_title_lbl: Label,
    pub console_close_btn: Button,
    pub text_view: TextView,
    pub text_buffer: TextBuffer,
    pub console_scroll: ScrolledWindow,
}
