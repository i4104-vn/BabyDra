/// A variant selectable during installation (read from `variants/*/variant.toml`).
#[derive(Debug, Clone)]
pub struct VariantItem {
    pub name: String,
    pub theme: String,
    pub apps: Vec<String>,
    pub selected: bool,
}

impl VariantItem {
    pub fn apps_preview(&self) -> String {
        if self.apps.is_empty() {
            "all apps (variant has no explicit app list)".to_string()
        } else {
            self.apps.join(", ")
        }
    }
}
