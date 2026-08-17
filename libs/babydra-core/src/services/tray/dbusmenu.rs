use zbus::zvariant::Value;

use crate::models::{LayoutItem, MenuItem};

#[zbus::proxy(interface = "com.canonical.dbusmenu")]
pub trait DbusMenu {
    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: &[&str],
    ) -> zbus::Result<(u32, LayoutItem)>;

    fn event(
        &self,
        id: i32,
        event_id: &str,
        data: &zbus::zvariant::Value<'_>,
        timestamp: u32,
    ) -> zbus::Result<()>;
}

/// Parses `layout item`.
pub fn parse_layout_item(item: &LayoutItem) -> MenuItem {
    let id = item.0;
    let props = &item.1;
    let children_vals = &item.2;

    let mut label = String::new();
    let mut enabled = true;
    let mut visible = true;
    let mut is_separator = false;

    if let Some(v) = props.get("label") {
        if let Value::Str(s) = &**v {
            label = s.as_str().replace("_", "");
        }
    }

    if let Some(v) = props.get("enabled") {
        if let Value::Bool(b) = &**v {
            enabled = *b;
        }
    }

    if let Some(v) = props.get("visible") {
        if let Value::Bool(b) = &**v {
            visible = *b;
        }
    }

    if let Some(v) = props.get("type") {
        if let Value::Str(s) = &**v {
            is_separator = s.as_str() == "separator";
        }
    }

    let mut children = Vec::new();
    for child_val in children_vals {
        if let Some(child_menu) = parse_zval(&**child_val) {
            children.push(child_menu);
        }
    }

    MenuItem {
        id,
        label,
        enabled,
        visible,
        is_separator,
        children,
    }
}

fn parse_zval(val: &Value<'_>) -> Option<MenuItem> {
    let struct_val = match val {
        Value::Value(v) => match &**v {
            Value::Structure(s) => s,
            _ => return None,
        },
        Value::Structure(s) => s,
        _ => return None,
    };

    let fields = struct_val.fields();
    if fields.len() != 3 {
        return None;
    }

    let id = match &fields[0] {
        Value::I32(i) => *i,
        _ => return None,
    };

    let mut label = String::new();
    let mut enabled = true;
    let mut visible = true;
    let mut is_separator = false;

    if let Value::Dict(d) = &fields[1] {
        if let Ok(Some(v)) = d.get::<_, Value<'_>>(&"label") {
            if let Value::Value(var) = v {
                if let Value::Str(s) = &*var {
                    label = s.as_str().replace("_", "");
                }
            } else if let Value::Str(s) = v {
                label = s.as_str().replace("_", "");
            }
        }

        if let Ok(Some(v)) = d.get::<_, Value<'_>>(&"enabled") {
            if let Value::Value(var) = v {
                if let Value::Bool(b) = &*var {
                    enabled = *b;
                }
            } else if let Value::Bool(b) = v {
                enabled = b;
            }
        }

        if let Ok(Some(v)) = d.get::<_, Value<'_>>(&"visible") {
            if let Value::Value(var) = v {
                if let Value::Bool(b) = &*var {
                    visible = *b;
                }
            } else if let Value::Bool(b) = v {
                visible = b;
            }
        }

        if let Ok(Some(v)) = d.get::<_, Value<'_>>(&"type") {
            if let Value::Value(var) = v {
                if let Value::Str(s) = &*var {
                    is_separator = s.as_str() == "separator";
                }
            } else if let Value::Str(s) = v {
                is_separator = s.as_str() == "separator";
            }
        }
    }

    let mut children = Vec::new();
    if let Value::Array(a) = &fields[2] {
        for i in 0..a.len() {
            if let Ok(Some(child_val)) = a.get::<Value<'_>>(i) {
                if let Some(child_menu) = parse_zval(&child_val) {
                    children.push(child_menu);
                }
            }
        }
    }

    Some(MenuItem {
        id,
        label,
        enabled,
        visible,
        is_separator,
        children,
    })
}
