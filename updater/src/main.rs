mod actions;
mod config;
mod core;
mod ui;
mod utils;

use std::io::{stdout, Result};
use std::panic;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::actions::runner::LogMessage;
use crate::core::app::App;
use crate::core::events::handle_key_event;

fn main() -> Result<()> {
    // 1. Setup panic hook to restore terminal if crash occurs
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = stdout().execute(LeaveAlternateScreen);
        let _ = stdout().execute(Show);
        default_hook(info);
    }));

    // 2. Initialize terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(Hide)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // 3. Initialize application state and communication channels
    let mut app = App::new();
    let (tx, rx): (Sender<LogMessage>, Receiver<LogMessage>) = channel();

    // 4. Main application loop
    let tick_rate = Duration::from_millis(60);

    while !app.should_quit {
        // Draw TUI
        terminal.draw(|f| ui::draw(f, &app))?;

        // Drain background log messages
        while let Ok(msg) = rx.try_recv() {
            app.handle_log_message(msg);
        }

        if matches!(app.view_state, crate::core::state::ViewState::Running) {
            app.tick_spinner();
        }

        // Handle user input
        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_event(&mut app, key.code, &tx);
                }
            }
        }
    }

    // 5. Cleanup and restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(Show)?;

    Ok(())
}
