use gtk4::prelude::*;

/// Creates a standardized styled Switch widget.
pub fn create_switch(
    initial_active: bool,
    on_changed: impl Fn(bool) + 'static,
) -> gtk4::Switch {
    let sw = gtk4::Switch::new();
    sw.set_valign(gtk4::Align::Center);
    sw.add_css_class("baby-switch");
    sw.set_active(initial_active);
    
    sw.connect_state_set(move |_, state| {
        on_changed(state);
        glib::Propagation::Proceed
    });
    
    sw
}
