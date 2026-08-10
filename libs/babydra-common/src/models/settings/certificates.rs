use serde::{Deserialize, Serialize};
use gtk4::{Box, Button, ListBox, Overlay};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertInfo {
    pub filename: String,
    pub path: String,
}

#[derive(Clone)]
pub struct CertificatesWidget {
    pub root: Overlay,
    pub container: Box,
    pub add_btn: Button,
    pub list_box: ListBox,
}
