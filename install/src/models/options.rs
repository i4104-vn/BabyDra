#[derive(Debug, Clone)]
pub struct GenericOptionItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub detail: String,
    pub selected: bool,
    pub requires_root: bool,
}
