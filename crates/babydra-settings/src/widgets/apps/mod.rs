pub mod handler;
pub mod render;

use gtk4::Widget;
use std::cell::RefCell;
use std::rc::Rc;

/// Creates a new `apps widget`.
pub fn create_apps_widget() -> Widget {
    // Build the initial UI layout instantly (0ms main-thread blocking)
    let (widget, auth_dialog) = render::build();
    let auth_dialog_rc = Rc::new(auth_dialog);
    let pending_action = Rc::new(RefCell::new(None::<handler::PendingAction>));

    let apps_data = Rc::new(RefCell::new(Vec::<render::AppItemData>::new()));
    let pkgs_data = Rc::new(RefCell::new(Vec::<render::PkgItemData>::new()));
    let is_loading = Rc::new(RefCell::new(true));

    // Show initial loading placeholder animation on both tabs, identical to other settings tabs
    render::render_apps_list(
        &widget.apps_list_box,
        &[],
        true,
        &auth_dialog_rc,
        pending_action.clone(),
    );
    render::render_pkgs_list(
        &widget.pkgs_list_box,
        &[],
        true,
        &auth_dialog_rc,
        pending_action.clone(),
    );

    // Wire main event handlers (tabs switching, search, console close, refresh button)
    handler::wire_main_events(
        &widget,
        &auth_dialog_rc,
        pending_action.clone(),
        apps_data.clone(),
        pkgs_data.clone(),
        is_loading.clone(),
    );

    // Trigger async data scan in the background
    handler::fetch_apps_and_pkgs_async(
        &widget,
        &auth_dialog_rc,
        pending_action.clone(),
        apps_data.clone(),
        pkgs_data.clone(),
        is_loading.clone(),
    );

    widget.root.into()
}
