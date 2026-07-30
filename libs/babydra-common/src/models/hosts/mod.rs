use gtk4::{Box, Button, Label, Overlay, TextBuffer, TextView};

pub struct HostsWidget {
    pub root: Overlay,
    pub container: Box,
    pub title_label: Label,
    pub status_badge: Label,
    pub save_btn: Button,
    pub reload_btn: Button,
    pub glass_card: Box,
    pub text_view: TextView,
    pub text_buffer: TextBuffer,
    pub auth_overlay: Box,
}
