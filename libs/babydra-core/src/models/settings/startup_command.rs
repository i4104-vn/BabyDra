use gtk4::{Box, Button, Entry};

#[derive(Debug, Clone)]
pub struct StartupCommand {
    pub id: u32,
    pub command: String,
}

#[derive(Clone)]
pub struct StartupWidget {
    pub container: Box,
    pub list_box: Box,
    pub add_btn: Button,
    pub save_btn: Button,
    pub entries: Vec<Entry>,
}
