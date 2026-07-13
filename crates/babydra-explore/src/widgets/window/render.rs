use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box, Orientation, Paned};
use babydra_common::MainWindowWidgets;

/// Builds the MainWindow container and basic layouts grid.
pub fn build_window_ui(app: &gtk4::Application) -> MainWindowWidgets {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("BabyDra Explore")
        .default_width(1000)
        .default_height(700)
        .build();
    window.add_css_class("explore-window");

    // Apply Windows 11 dark theme
    babydra_common::apply_explore_theme();

    let vbox = Box::new(Orientation::Vertical, 0);
    window.set_child(Some(&vbox));

    // Create Split Paned container for left/right file panes
    let split_paned = Paned::new(Orientation::Horizontal);
    split_paned.set_hexpand(true);
    split_paned.set_vexpand(true);

    // Paned (Sidebar + Main Split Content View Area)
    let main_paned = Paned::new(Orientation::Horizontal);
    main_paned.set_hexpand(true);
    main_paned.set_vexpand(true);
    main_paned.set_position(220); // Allocate sidebar space
    vbox.append(&main_paned);

    // Content Area VBox (contains SplitPaned + InfoPanel side-by-side)
    let content_vbox = Box::new(Orientation::Vertical, 0);
    content_vbox.set_hexpand(true);
    content_vbox.set_vexpand(true);
    main_paned.set_end_child(Some(&content_vbox));

    // Horizontal Paned to show InfoPanel resizable next to split view
    let layout_paned = Paned::new(Orientation::Horizontal);
    layout_paned.set_hexpand(true);
    layout_paned.set_vexpand(true);
    layout_paned.set_position(530); // Allocate space for InfoPanel
    layout_paned.set_resize_end_child(false);  // Info panel keeps fixed width
    layout_paned.set_shrink_end_child(false);  // Info panel cannot be shrunk
    content_vbox.append(&layout_paned);

    layout_paned.set_start_child(Some(&split_paned));

    MainWindowWidgets {
        window,
        vbox,
        split_paned,
        main_paned,
        content_vbox,
        layout_paned,
    }
}
