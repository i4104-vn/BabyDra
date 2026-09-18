//! System Update settings card.

pub mod handler;
pub mod render;

use babydra_ui_kit::components::modals::PasswordDialog;
use gtk4::Box as GtkBox;

/// Builds the System Update collapsible card and mounts its password dialog overlay.
pub fn build_system_update_card(overlay: &gtk4::Overlay) -> GtkBox {
    let widgets = render::render_system_update_card();

    let auth_dialog = PasswordDialog::new(
        &babydra_core::i18n::trans("settings.auth_required"),
        &babydra_core::i18n::trans("settings.auth_enter_pwd"),
    );
    overlay.add_overlay(&auth_dialog.container);

    handler::wire_events(&widgets, &auth_dialog);

    widgets.container
}
