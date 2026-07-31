use gtk4::{Box, Button};

#[derive(Debug, Clone)]
pub struct Keybind {
    pub id: usize,
    pub bind_type: String,
    pub modifiers: String,
    pub key: String,
    pub dispatcher: String,
    pub args: String,
}

#[derive(Clone)]
pub struct KeybindsWidget {
    pub container: Box,
    pub table_box: Box,
    pub add_btn: Button,
    pub refresh_btn: Button,
    pub save_btn: Button,
}
