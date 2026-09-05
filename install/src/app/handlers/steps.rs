use crossterm::event::{KeyCode, KeyEvent};

use crate::models::{InstallState, LogLevel, WizardStep};

use super::super::state::App;

/// Action derived from a key press on a checkbox-list step.
pub enum ListAction {
    None,
    /// Space on the cursor row.
    Toggle,
    /// 'a' — the argument is the new "all selected" state.
    ToggleAll,
    Enter,
}

/// Shared Up/Down/Space/'a'/Enter handling for the checkbox-list steps.
pub fn list_action(key: KeyEvent, len: usize, cursor: &mut usize) -> ListAction {
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
        KeyCode::PageUp => {
            *cursor = cursor.saturating_sub(5);
            ListAction::None
        }
        KeyCode::PageDown => {
            if len > 0 {
                *cursor = (*cursor + 5).min(len - 1);
            }
            ListAction::None
        }
        KeyCode::Home => {
            *cursor = 0;
            ListAction::None
        }
        KeyCode::End => {
            if len > 0 {
                *cursor = len - 1;
            }
            ListAction::None
        }
        KeyCode::Char(' ') => ListAction::Toggle,
        KeyCode::Char('a') | KeyCode::Char('A') => ListAction::ToggleAll,
        KeyCode::Enter => ListAction::Enter,
        _ => ListAction::None,
    }
}

pub fn handle_step_interaction(app: &mut App, key: KeyEvent) {
    match app.current_step {
        WizardStep::Welcome => {
            if key.code == KeyCode::Enter
                || key.code == KeyCode::Char(' ')
                || key.code == KeyCode::Right
            {
                app.next_step();
            }
        }

        WizardStep::SourceBranch => {
            if app.branches.is_empty() {
                return;
            }
            let max_idx = app.branches.len().saturating_sub(1);
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if app.branch_cursor > 0 {
                        app.branch_cursor -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if app.branch_cursor < max_idx {
                        app.branch_cursor += 1;
                    }
                }
                KeyCode::PageUp => {
                    app.branch_cursor = app.branch_cursor.saturating_sub(5);
                }
                KeyCode::PageDown => {
                    app.branch_cursor = (app.branch_cursor + 5).min(max_idx);
                }
                KeyCode::Home => {
                    app.branch_cursor = 0;
                }
                KeyCode::End => {
                    app.branch_cursor = max_idx;
                }
                KeyCode::Char(' ') => {
                    select_branch_at_cursor(app);
                }
                KeyCode::Enter => {
                    if app.branches.is_empty() {
                        app.add_log(
                            LogLevel::Error,
                            "No installation releases available to continue.",
                        );
                        return;
                    }
                    select_branch_at_cursor(app);
                    if app.is_build_from_source() {
                        app.start_branch_switch();
                    } else {
                        app.next_step();
                    }
                }
                _ => {}
            }
        }

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
                    select_variant_at_cursor(app);
                }
                ListAction::Enter => {
                    select_variant_at_cursor(app);
                    app.next_step();
                }
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
            KeyCode::PageUp => {
                app.auto_scroll_logs = false;
                app.log_scroll = app.log_scroll.saturating_sub(10);
            }
            KeyCode::PageDown => {
                if app.log_scroll + 10 < app.logs.len() {
                    app.log_scroll += 10;
                } else {
                    app.auto_scroll_logs = true;
                    app.log_scroll = app.logs.len().saturating_sub(12);
                }
            }
            KeyCode::Char('c') => {
                app.logs.clear();
                app.log_scroll = 0;
            }
            KeyCode::Home | KeyCode::Char('g') => {
                app.log_scroll = 0;
                app.auto_scroll_logs = false;
            }
            KeyCode::End | KeyCode::Char('G') => {
                app.auto_scroll_logs = true;
                app.log_scroll = app.logs.len().saturating_sub(12);
            }
            KeyCode::Enter | KeyCode::Char('i') | KeyCode::Char('I')
                if app.install_state != InstallState::Installing =>
            {
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

pub fn select_variant_at_cursor(app: &mut App) {
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

pub fn select_branch_at_cursor(app: &mut App) {
    if app.branches.is_empty() {
        app.selected_branch.clear();
        return;
    }
    for b in &mut app.branches {
        b.selected = false;
    }
    if let Some(branch) = app.branches.get_mut(app.branch_cursor) {
        branch.selected = true;
        app.selected_branch = branch.name.clone();
    }
}
