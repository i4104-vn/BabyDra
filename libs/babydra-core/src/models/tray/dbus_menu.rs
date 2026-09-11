use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Type};

/// Represents a raw layout item returned from the DBus menu's GetLayout call.
/// The tuple signature is: (id, properties, children).
#[derive(Debug, serde::Serialize, serde::Deserialize, Type)]
#[zvariant(signature = "(ia{sv}av)")]
pub struct LayoutItem(
    pub i32,
    pub HashMap<String, OwnedValue>,
    pub Vec<OwnedValue>,
);

/// A parsed, hierarchical representation of a menu item.
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub id: i32,
    pub label: String,
    pub enabled: bool,
    pub visible: bool,
    pub is_separator: bool,
    pub toggle_type: Option<String>,
    pub toggle_state: Option<i32>,
    pub children: Vec<MenuItem>,
}
