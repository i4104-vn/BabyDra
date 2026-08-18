use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

#[test]
fn test_gtk_css_parsing() {
    if gtk4::init().is_err() {
        println!("Skipping GTK test if display unavailable");
        return;
    }

    std::env::set_var("BABYDRA_THEMES_DIR", repo_root().join("themes"));

    let files = [
        "libs/babydra-ui-kit/src/styles/shared/panel/panel.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/workspaces.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/clock.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/status.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/system_monitor.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/tray.css",
        "libs/babydra-ui-kit/src/styles/shared/panel/taskbar.css",
        "libs/babydra-ui-kit/src/styles/shared/control_center/control_center.css",
        "libs/babydra-ui-kit/src/styles/shared/control_center/power.css",
        "libs/babydra-ui-kit/src/styles/shared/island/system_island.css",
        "libs/babydra-ui-kit/src/styles/shared/island/notification.css",
        "libs/babydra-ui-kit/src/styles/shared/launcher/launcher.css",
        "libs/babydra-ui-kit/src/styles/shared/calendar/calendar.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/button.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/sidebar.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/screenshot.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/lock.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/preview.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/settings.css",
        "libs/babydra-ui-kit/src/styles/shared/apps/switcher.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/window.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/header_bar.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/content_view.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/info_panel.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/status_bar.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/context_menu.css",
        "libs/babydra-ui-kit/src/styles/shared/explore/dialogs.css",
        "libs/babydra-ui-kit/src/styles/shared/shared/scrollbar.css",
        "themes/babydra-default/css/dark.css",
        "themes/babydra-default/css/light.css",
        "themes/babydra-default/css/theme.css",
        "themes/babydra-blue/css/dark.css",
        "themes/babydra-blue/css/light.css",
        "themes/babydra-blue/css/theme.css",
        "themes/babydra-green/css/dark.css",
        "themes/babydra-green/css/light.css",
        "themes/babydra-green/css/theme.css",
        "themes/babydra-purple/css/dark.css",
        "themes/babydra-purple/css/light.css",
        "themes/babydra-purple/css/theme.css",
        "themes/babydra-rose/css/dark.css",
        "themes/babydra-rose/css/light.css",
        "themes/babydra-rose/css/theme.css",
    ];

    let mut total_errors = 0;

    for rel_path in &files {
        let full_path = repo_root().join(rel_path);
        let content = std::fs::read_to_string(&full_path).unwrap_or_else(|e| panic!("Could not read {}: {}", rel_path, e));
        let provider = gtk4::CssProvider::new();
        let errors = Rc::new(RefCell::new(Vec::new()));
        let errors_clone = errors.clone();

        let rel_path_str = rel_path.to_string();
        provider.connect_parsing_error(move |_prov, section, err| {
            let start = section.start_location();
            errors_clone.borrow_mut().push(format!(
                "  [{}:{}:{}] {}",
                rel_path_str,
                start.lines() + 1,
                start.line_chars() + 1,
                err
            ));
        });

        provider.load_from_data(&content);

        let errs = errors.borrow();
        if !errs.is_empty() {
            println!("Errors in {}:", rel_path);
            for e in errs.iter() {
                println!("{}", e);
            }
            total_errors += errs.len();
        }
    }

    // Now test resolved themes (concatenated layers)
    let theme_names = ["babydra-default", "babydra-blue", "babydra-green", "babydra-purple", "babydra-rose"];
    for theme_name in &theme_names {
        let theme = babydra_theme::resolve_theme(theme_name).expect("Failed to resolve theme");

        for (mode, color_css) in [("dark", &theme.dark_css), ("light", &theme.light_css)] {
            // Concatenate all structural files + color layer + extra layer
            let mut full_css = String::new();
            for rel_path in &files[0..28] {
                let p = repo_root().join(rel_path);
                full_css.push_str(&std::fs::read_to_string(&p).unwrap());
                full_css.push('\n');
            }
            full_css.push_str(color_css);
            full_css.push('\n');
            full_css.push_str(&theme.css_layer);

            let provider = gtk4::CssProvider::new();
            let errors = Rc::new(RefCell::new(Vec::new()));
            let errors_clone = errors.clone();
            let context = format!("theme: {} (mode: {})", theme_name, mode);

            provider.connect_parsing_error(move |_prov, section, err| {
                let start = section.start_location();
                errors_clone.borrow_mut().push(format!(
                    "  [concatenated line {}:{}] {}",
                    start.lines() + 1,
                    start.line_chars() + 1,
                    err
                ));
            });

            provider.load_from_data(&full_css);

            let errs = errors.borrow();
            if !errs.is_empty() {
                println!("Errors in concatenated CSS for {}:", context);
                for e in errs.iter() {
                    println!("{}", e);
                }
                total_errors += errs.len();
            }
        }
    }

    println!("\nTotal CSS parsing errors found: {}", total_errors);
    assert_eq!(total_errors, 0, "Found {} CSS parsing errors in GTK stylesheets", total_errors);
}
