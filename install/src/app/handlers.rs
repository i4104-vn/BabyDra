use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;

use super::App;
use crate::models::{InstallState, WizardStep};

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
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
                app.show_confirm_dialog = false;
                app.begin_install();
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Char('q') => {
                app.show_confirm_dialog = false;
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
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
        }
        KeyCode::Char('?') => {
            app.show_help = true;
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.is_editing_path = true;
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.rescan_binaries();
        }
        KeyCode::Char('i') | KeyCode::Char('I') => {
            if app.install_state != InstallState::Installing {
                app.show_confirm_dialog = true;
            }
        }

        // Direct Number Jumping (1-5, 0 = summary)
        KeyCode::Char('1') => app.current_step = WizardStep::Welcome,
        KeyCode::Char('2') => app.current_step = WizardStep::SourceBranch,
        KeyCode::Char('3') => app.current_step = WizardStep::Binaries,
        KeyCode::Char('4') => app.current_step = WizardStep::VariantSelection,
        KeyCode::Char('5') => app.current_step = WizardStep::ExecuteInstall,
        KeyCode::Char('0') => app.current_step = WizardStep::Summary,

        // Step Navigation
        KeyCode::Tab | KeyCode::Char('n') => {
            if app.current_step == WizardStep::SourceBranch && app.is_build_from_source() {
                app.start_branch_switch();
            } else {
                app.next_step();
            }
        }
        KeyCode::BackTab | KeyCode::Char('p') => app.prev_step(),

        // Step-Specific Interaction
        _ => handle_step_interaction(app, key),
    }
}

/// Action derived from a key press on a checkbox-list step.
enum ListAction {
    None,
    /// Space on the cursor row.
    Toggle,
    /// 'a' — the argument is the new "all selected" state.
    ToggleAll,
    Enter,
}

/// Shared Up/Down/Space/'a'/Enter handling for the checkbox-list steps.
///
/// Moves `cursor` within `len` and classifies the key; the caller applies
/// the action to its own item list.
fn list_action(key: KeyEvent, len: usize, cursor: &mut usize) -> ListAction {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            *cursor = cursor.saturating_sub(1);
            ListAction::None
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if *cursor + 1 < len {
                *cursor += 1;
            }
            ListAction::None
        }
        KeyCode::Char(' ') => ListAction::Toggle,
        KeyCode::Char('a') | KeyCode::Char('A') => ListAction::ToggleAll,
        KeyCode::Enter => ListAction::Enter,
        _ => ListAction::None,
    }
}

fn handle_step_interaction(app: &mut App, key: KeyEvent) {
    match app.current_step {
        WizardStep::Welcome => {
            if key.code == KeyCode::Enter {
                app.next_step();
            }
        }

        WizardStep::SourceBranch => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.branch_cursor > 0 {
                    app.branch_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // Row 0 = pre-built only; rows 1..=N map to branches, so the
                // cursor max is branches.len() (not len - 1).
                if app.branch_cursor < app.branches.len() {
                    app.branch_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                select_branch_at_cursor(app);
            }
            KeyCode::Enter => {
                if app.is_build_from_source() {
                    app.start_branch_switch();
                } else {
                    app.next_step();
                }
            }
            _ => {}
        },

        WizardStep::Binaries => {
            let len = app.binaries.len();
            match list_action(key, len, &mut app.binary_cursor) {
                ListAction::Toggle => {
                    if let Some(item) = app.binaries.get_mut(app.binary_cursor) {
                        item.selected = !item.selected;
                    }
                }
                ListAction::ToggleAll => {
                    let all_sel = app.binaries.iter().all(|b| b.selected);
                    for b in &mut app.binaries {
                        b.selected = !all_sel;
                    }
                }
                ListAction::Enter => app.next_step(),
                ListAction::None => {}
            }
        }

        WizardStep::VariantSelection => {
            let len = app.variant_options.len();
            match list_action(key, len, &mut app.variant_cursor) {
                ListAction::Toggle => {
                    if app.variant_cursor < app.variant_options.len() {
                        for v in &mut app.variant_options {
                            v.selected = false;
                        }
                        if let Some(selected) = app.variant_options.get_mut(app.variant_cursor) {
                            selected.selected = true;
                            app.selected_variant = selected.name.clone();
                        }
                    }
                }
                ListAction::Enter => app.next_step(),
                _ => {}
            }
        }

        WizardStep::ExecuteInstall => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.auto_scroll_logs = false;
                if app.log_scroll > 0 {
                    app.log_scroll -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.log_scroll + 1 < app.logs.len() {
                    app.log_scroll += 1;
                }
            }
            KeyCode::Char('c') => {
                app.logs.clear();
                app.log_scroll = 0;
            }
            KeyCode::Char('g') => {
                app.log_scroll = 0;
                app.auto_scroll_logs = false;
            }
            KeyCode::Char('G') => {
                app.auto_scroll_logs = true;
                app.log_scroll = app.logs.len().saturating_sub(12);
            }
            KeyCode::Enter if app.install_state != InstallState::Installing => {
                app.show_confirm_dialog = true;
            }
            _ => {}
        },

        WizardStep::Summary => match key.code {
            KeyCode::Enter | KeyCode::Char('q') => {
                app.should_quit = true;
            }
            _ => {}
        },
    }
}

/// Row 0 = pre-built only; rows 1..=N map to `app.branches[cursor - 1]`.
fn select_branch_at_cursor(app: &mut App) {
    for b in &mut app.branches {
        b.selected = false;
    }
    if app.branch_cursor == 0 {
        app.selected_branch.clear();
    } else if let Some(branch) = app.branches.get(app.branch_cursor - 1) {
        app.selected_branch = branch.name.clone();
        app.branches[app.branch_cursor - 1].selected = true;
    }
}
