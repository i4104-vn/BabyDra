pub mod widgets;

use babydra_core::SessionState;
use gtk4::prelude::*;
use gtk4::Application;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-explore", "babydra-explore.log");

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("babydra-explore - BabyDra File Manager");
        println!();
        println!("USAGE:");
        println!("    babydra-explore [OPTIONS] [PATH]");
        println!();
        println!("ARGS:");
        println!("    <PATH>    File or directory path to open and highlight");
        println!();
        println!("OPTIONS:");
        println!("    -p, --path <PATH>      File or directory to open");
        println!("    -s, --select <PATH>    Select/highlight file in parent folder");
        println!("    -n, --new-window       Force opening in a new window");
        println!("    -h, --help             Print this help message");
        std::process::exit(0);
    }

    let force_new_window = args.iter().any(|a| a == "-n" || a == "--new-window");
    if !force_new_window {
        let mut target_candidate: Option<std::path::PathBuf> = None;
        let mut i = 1;
        while i < args.len() {
            let arg = &args[i];
            if let Some(val) = arg.strip_prefix("-p=").or_else(|| arg.strip_prefix("--path=")) {
                target_candidate = Some(std::path::PathBuf::from(val));
                break;
            }
            if let Some(val) = arg.strip_prefix("-s=").or_else(|| arg.strip_prefix("--select=")) {
                target_candidate = Some(std::path::PathBuf::from(val));
                break;
            }
            if arg == "-p" || arg == "--path" || arg == "-s" || arg == "--select" || arg == "--show-items" || arg == "--show-item" {
                if i + 1 < args.len() {
                    target_candidate = Some(std::path::PathBuf::from(&args[i + 1]));
                }
                break;
            }
            if !arg.starts_with('-') {
                target_candidate = Some(std::path::PathBuf::from(arg));
                break;
            }
            i += 1;
        }

        if let Some(ref path) = target_candidate {
            if babydra_core::services::explore::try_open_in_running_instance(path) {
                std::process::exit(0);
            }
        }
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("org.babydra.explore")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE | gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_activate(|app| {
        let (target_dir, focus_item) = babydra_ui_kit::components::explore::parse_target_dir();

        let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(target_dir)));
        let main_window = crate::widgets::window::create_explore_win(app, session, focus_item);
        main_window.present();
    });

    app.connect_open(|app, files, _hint| {
        let mut target_dir = glib::home_dir();
        let mut focus_item = None;

        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                let (dir, focus) = babydra_core::services::explore::resolve_target_from_path(&path);
                target_dir = dir;
                focus_item = focus;
            } else {
                let uri = file.uri();
                let (dir, focus) =
                    babydra_core::services::explore::resolve_target_from_uri(uri.as_str());
                target_dir = dir;
                focus_item = focus;
            }
        }

        let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(target_dir)));
        let main_window = crate::widgets::window::create_explore_win(app, session, focus_item);
        main_window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
