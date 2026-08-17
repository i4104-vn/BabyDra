//! Integration tests: kits public API / prelude.
//!
//! Verifies the one-stop `prelude` re-exports of `babydra-ui-kit` and
//! `babydra-explore-kit` stay wired up — any rename or removal of a public
//! item breaks this test, so the documented API surface cannot drift silently.

use babydra_explore_kit::prelude as explore_prelude;
use babydra_ui_kit::prelude as ui_prelude;

/// Asserts the ui-kit prelude exposes the core builders & helpers.
///
/// Concrete fn pointers pin the exact signatures for stable API; items with
/// `impl Trait` params are referenced as values (compiles ⇒ still exists).
#[test]
fn ui_kit_prelude_exposes_components_and_helpers() {
    // Component builders.
    let _: fn(&str) -> gtk4::Button = ui_prelude::create_button;
    let _: fn(&str) -> gtk4::Button = ui_prelude::create_accent_button;
    let _: fn(&str) -> gtk4::Button = ui_prelude::create_fab;
    let _: fn(gtk4::Orientation, i32) -> gtk4::Box = ui_prelude::create_card;
    let _: fn(&str) -> gtk4::Label = ui_prelude::create_title;

    // impl-Trait builders — instantiated with concrete callable types.
    let _: fn(&str, i32, &[&str], Option<&str>, fn()) -> gtk4::Button =
        ui_prelude::create_icon_button;
    let _: fn(&gtk4::Box, &str) = ui_prelude::set_tooltip;
    let _: fn(&str, &str) -> (gtk4::Box, ui_prelude::CustomSwitch) = ui_prelude::create_switch_card;
    let _: fn(ui_prelude::PlaceholderState) -> gtk4::ListBoxRow =
        ui_prelude::create_placeholder_row;
    let _: fn(bool, fn(bool)) -> ui_prelude::CustomSwitch = ui_prelude::create_switch;

    // UI helpers.
    let _: fn(&str, i32) -> gtk4::Image = ui_prelude::get_icon;
    let _: fn(&str, i32, &str) -> gtk4::Image = ui_prelude::get_icon_colored;
    let _: fn(u32, bool) -> &'static str = ui_prelude::get_battery_color_hex;
    let _: fn() = ui_prelude::init_theme;
    let _: fn(bool) = ui_prelude::set_dark_mode;
    let _: fn(&gtk4::ApplicationWindow) = ui_prelude::apply_theme_class;
    let _: fn(f64) -> f64 = ui_prelude::ease_out_cubic;
}

/// Asserts the explore-kit prelude exposes the feature API.
#[test]
fn explore_kit_prelude_exposes_features() {
    // Dialogs — `impl IsA<gtk4::Window>` instantiates to ApplicationWindow.
    let _: fn(
        std::path::PathBuf,
        std::rc::Rc<dyn Fn(std::path::PathBuf)>,
        Option<&gtk4::ApplicationWindow>,
    ) = explore_prelude::show_new_folder_dialog;
    let _: fn(&str, &str, Option<&gtk4::ApplicationWindow>) = explore_prelude::show_alert_dialog;
    let _: fn(
        &std::path::PathBuf,
        std::path::PathBuf,
        std::rc::Rc<dyn Fn(std::path::PathBuf)>,
        Option<&gtk4::ApplicationWindow>,
    ) = explore_prelude::show_rename_dialog;
    let _: fn(std::vec::Vec<std::path::PathBuf>, Option<&gtk4::ApplicationWindow>) =
        explore_prelude::show_properties_dialog;

    // Drag & drop / selection / items.
    let _: fn(std::path::PathBuf) -> gtk4::DropTarget = explore_prelude::create_dir_drop_target;
    let _: fn(&gtk4::Widget, gtk4::Box, gtk4::Fixed, gtk4::Box) =
        explore_prelude::wire_rubberband_grid;
    let _: fn(
        usize,
        &babydra_core::FileEntry,
        std::rc::Rc<std::cell::RefCell<Vec<std::path::PathBuf>>>,
        fn(&gtk4::Widget, f64, f64),
    ) -> gtk4::FlowBoxChild = explore_prelude::create_grid_file_item;

    // Helpers.
    let _: fn(u64) -> String = explore_prelude::format_size;
    let _: fn(std::time::SystemTime) -> String = explore_prelude::format_date;
    let _: fn(&std::path::Path) -> bool = explore_prelude::is_archive_file;
    let _: fn(&std::path::Path) -> std::path::PathBuf = explore_prelude::sanitize_path;
    let _: fn(&std::path::Path) -> bool = explore_prelude::is_in_trash;
}
