pub mod global;
pub mod steps;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;

use super::state::App;

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    // 0. Branch Switching Modal
    if app.show_branch_switching_modal {
        match app.branch_switch_status {
            crate::app::BranchSwitchStatus::Done(Err(_)) => {
                if key.code == KeyCode::Esc
                    || key.code == KeyCode::Enter
                    || key.code == KeyCode::Char('q')
                {
                    app.show_branch_switching_modal = false;
                }
            }
            _ => {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    app.should_quit = true;
                }
            }
        }
        return;
    }

    // 1. Modal Help Popup
    if app.show_help {
        if key.code == KeyCode::Esc
            || key.code == KeyCode::Char('q')
            || key.code == KeyCode::Char('?')
            || key.code == KeyCode::Enter
        {
            app.show_help = false;
        }
        return;
    }

    // 2. Sudo Password Modal (masked input). Ctrl+C still quits — it must
    // not be captured as a password character.
    if app.show_sudo_modal {
        match key.code {
            KeyCode::Enter => app.submit_sudo(),
            KeyCode::Esc => app.cancel_sudo(),
            KeyCode::Backspace => {
                app.sudo_password.pop();
            }
            KeyCode::Char(_c) if key.modifiers.contains(KeyModifiers::CONTROL) => {
                app.should_quit = true;
            }
            KeyCode::Char(c) => {
                app.sudo_password.push(c);
                app.sudo_error = None;
            }
            _ => {}
        }
        return;
    }

    // 3. Modal Confirm Dialog
    if app.show_confirm_dialog {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            app.should_quit = true;
            return;
        }
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                app.show_confirm_dialog = false;
                app.begin_install();
            }
            KeyCode::Esc
            | KeyCode::Left
            | KeyCode::BackTab
            | KeyCode::Char('p')
            | KeyCode::Char('b')
            | KeyCode::Char('n')
            | KeyCode::Char('N')
            | KeyCode::Char('q') => {
                app.show_confirm_dialog = false;
                app.prev_step();
            }
            _ => {}
        }
        return;
    }

    // 4. Path Editing Mode
    if app.is_editing_path {
        match key.code {
            KeyCode::Enter => {
                app.source_binary_dir = PathBuf::from(&app.custom_path_input);
                app.is_editing_path = false;
                app.rescan_binaries();
            }
            KeyCode::Esc => {
                app.custom_path_input = app.source_binary_dir.to_string_lossy().to_string();
                app.is_editing_path = false;
            }
            KeyCode::Backspace => {
                app.custom_path_input.pop();
            }
            KeyCode::Char(c) => {
                app.custom_path_input.push(c);
            }
            _ => {}
        }
        return;
    }

    // 5. Global Navigation Keys
    if global::handle_global_keys(app, key) {
        return;
    }

    // 6. Step-Specific Interaction
    steps::handle_step_interaction(app, key);
}
