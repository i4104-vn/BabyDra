mod app;
mod models;
mod system;
mod tasks;
mod ui;

use std::io;
use std::panic;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

fn main() -> Result<()> {
    setup_panic_hook();

    let args: Vec<String> = std::env::args().collect();
    let mut app = App::new();

    if args.len() > 1 {
        let arg = &args[1];
        if arg == "--help" || arg == "-h" {
            println!("BabyDra Desktop TUI Installer");
            println!("Usage: babydra-installer [SOURCE_BIN_DIR]");
            println!();
            println!("Arguments:");
            println!("  [SOURCE_BIN_DIR]   Optional path to folder containing prebuilt binaries (e.g. target/release)");
            println!();
            println!("Options:");
            println!("  -h, --help         Show this help message");
            println!("  -v, --version      Show version");
            return Ok(());
        } else if arg == "--version" || arg == "-v" {
            println!("babydra-installer v1.0.0");
            return Ok(());
        } else if !arg.starts_with('-') {
            app.source_binary_dir = std::path::PathBuf::from(arg);
            app.custom_path_input = arg.clone();
            app.rescan_binaries();
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Application error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }

        app.on_tick();

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
