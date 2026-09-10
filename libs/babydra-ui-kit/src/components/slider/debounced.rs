use gtk4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Binds a `gtk4::Scale` to update a percentage value label immediately while dragging,
/// and debounces the callback invocation by `delay_ms` to avoid flooding hardware/services.
pub fn bind_debounced_slider<F>(
    scale: &gtk4::Scale,
    value_label: &gtk4::Label,
    delay_ms: u64,
    on_change: F,
) where
    F: Fn(f64) + 'static,
{
    let last_source: Rc<Cell<Option<gtk4::glib::SourceId>>> = Rc::new(Cell::new(None));
    let on_change = Rc::new(on_change);

    let val_label_c = value_label.clone();
    let last_source_c = last_source.clone();
    let on_change_c = on_change.clone();

    scale.connect_value_changed(move |s| {
        let val = s.value();
        val_label_c.set_text(&format!("{:.0}%", val));

        if let Some(id) = last_source_c.take() {
            id.remove();
        }

        let last_source_inner = last_source_c.clone();
        let on_change_inner = on_change_c.clone();
        let new_id = gtk4::glib::timeout_add_local_once(
            std::time::Duration::from_millis(delay_ms),
            move || {
                last_source_inner.set(None);
                on_change_inner(val);
            },
        );
        last_source_c.set(Some(new_id));
    });
}
