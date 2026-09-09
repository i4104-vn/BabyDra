use babydra_workspace::switcher::build_workspace_switcher_ui;
use gtk4::prelude::*;

/// Runs one-shot UI instance when daemon is not running.
pub fn run_oneshot_ui() {
    let app = gtk4::Application::new(Some("org.babydra.workspace.oneshot"), Default::default());
    app.connect_activate(|app| {
        let controller = build_workspace_switcher_ui(app);
        (controller.show_fn)();
    });
    app.run_with_args(&["babydra-workspace"]);
}
