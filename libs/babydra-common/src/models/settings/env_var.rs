use gtk4::{Box, Button};

#[derive(Debug, Clone)]
pub struct EnvVar {
    pub id: usize,
    pub key: String,
    pub value: String,
}

#[derive(Clone)]
pub struct EnvWidget {
    pub container: Box,
    pub list_box: Box,
    pub add_btn: Button,
    pub save_btn: Button,
}
