use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::PathBuf;

use super::App;
use crate::models::{InstallState, PresetProfile, WizardStep};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
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

        // Direct Number Jumping (1-9, 0 = summary)
        KeyCode::Char('1') => app.current_step = WizardStep::Welcome,
        KeyCode::Char('2') => app.current_step = WizardStep::SourceBranch,
        KeyCode::Char('3') => app.current_step = WizardStep::SystemPackages,
        KeyCode::Char('4') => app.current_step = WizardStep::Binaries,
        KeyCode::Char('5') => app.current_step = WizardStep::VarLibBundle,
        KeyCode::Char('6') => app.current_step = WizardStep::ConfigsThemes,
        KeyCode::Char('7') => app.current_step = WizardStep::VariantSelection,
        KeyCode::Char('8') => app.current_step = WizardStep::DisplayManager,
        KeyCode::Char('9') => app.current_step = WizardStep::ExecuteInstall,
        KeyCode::Char('0') => app.current_step = WizardStep::Summary,

        // Step Navigation
        KeyCode::Tab | KeyCode::Char('n') => app.next_step(),
        KeyCode::BackTab | KeyCode::Char('p') => app.prev_step(),

        // Step-Specific Interaction
        _ => handle_step_interaction(app, key),
    }
}

fn handle_step_interaction(app: &mut App, key: KeyEvent) {
    match app.current_step {
        WizardStep::Welcome => match key.code {
            KeyCode::Char('c') | KeyCode::Char('C') => {
                app.toggle_channel();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.current_profile = match app.current_profile {
                    PresetProfile::FullDesktop => PresetProfile::Custom,
                    PresetProfile::BinariesAndBundle => PresetProfile::FullDesktop,
                    PresetProfile::Custom => PresetProfile::BinariesAndBundle,
                };
                app.apply_profile(app.current_profile);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.current_profile = match app.current_profile {
                    PresetProfile::FullDesktop => PresetProfile::BinariesAndBundle,
                    PresetProfile::BinariesAndBundle => PresetProfile::Custom,
                    PresetProfile::Custom => PresetProfile::FullDesktop,
                };
                app.apply_profile(app.current_profile);
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::SourceBranch => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.branch_cursor > 0 {
                    app.branch_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.branch_cursor <= app.branches.len() {
                    app.branch_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                select_branch_at_cursor(app);
                app.current_profile = PresetProfile::Custom;
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::SystemPackages => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.package_cursor > 0 {
                    app.package_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.package_cursor + 1 < app.package_options.len() {
                    app.package_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(item) = app.package_options.get_mut(app.package_cursor) {
                    item.selected = !item.selected;
                    app.current_profile = PresetProfile::Custom;
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let all_sel = app.package_options.iter().all(|o| o.selected);
                for o in &mut app.package_options {
                    o.selected = !all_sel;
                }
                app.current_profile = PresetProfile::Custom;
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::Binaries => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.binary_cursor > 0 {
                    app.binary_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.binary_cursor + 1 < app.binaries.len() {
                    app.binary_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(item) = app.binaries.get_mut(app.binary_cursor) {
                    item.selected = !item.selected;
                    app.current_profile = PresetProfile::Custom;
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let all_sel = app.binaries.iter().all(|b| b.selected);
                for b in &mut app.binaries {
                    b.selected = !all_sel;
                }
                app.current_profile = PresetProfile::Custom;
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::VarLibBundle => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.varlib_cursor > 0 {
                    app.varlib_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.varlib_cursor + 1 < app.varlib_options.len() {
                    app.varlib_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(item) = app.varlib_options.get_mut(app.varlib_cursor) {
                    item.selected = !item.selected;
                    app.current_profile = PresetProfile::Custom;
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let all_sel = app.varlib_options.iter().all(|o| o.selected);
                for o in &mut app.varlib_options {
                    o.selected = !all_sel;
                }
                app.current_profile = PresetProfile::Custom;
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::ConfigsThemes => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.configs_themes_cursor > 0 {
                    app.configs_themes_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.configs_themes_cursor + 1 < app.configs_themes_options.len() {
                    app.configs_themes_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(item) = app
                    .configs_themes_options
                    .get_mut(app.configs_themes_cursor)
                {
                    item.selected = !item.selected;
                    app.current_profile = PresetProfile::Custom;
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let all_sel = app.configs_themes_options.iter().all(|o| o.selected);
                for o in &mut app.configs_themes_options {
                    o.selected = !all_sel;
                }
                app.current_profile = PresetProfile::Custom;
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::VariantSelection => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.variant_cursor > 0 {
                    app.variant_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.variant_cursor + 1 < app.variant_options.len() {
                    app.variant_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if app.variant_cursor < app.variant_options.len() {
                    for v in &mut app.variant_options {
                        v.selected = false;
                    }
                    if let Some(selected) = app.variant_options.get_mut(app.variant_cursor) {
                        selected.selected = true;
                        app.selected_variant = selected.name.clone();
                    }
                    app.current_profile = PresetProfile::Custom;
                }
            }
            KeyCode::Enter => app.next_step(),
            _ => {}
        },

        WizardStep::DisplayManager => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.display_manager_cursor > 0 {
                    app.display_manager_cursor -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.display_manager_cursor + 1 < app.display_manager_options.len() {
                    app.display_manager_cursor += 1;
                }
            }
            KeyCode::Char(' ') => {
                if let Some(item) = app
                    .display_manager_options
                    .get_mut(app.display_manager_cursor)
                {
                    item.selected = !item.selected;
                    app.current_profile = PresetProfile::Custom;
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                let all_sel = app.display_manager_options.iter().all(|o| o.selected);
                for o in &mut app.display_manager_options {
                    o.selected = !all_sel;
                }
                app.current_profile = PresetProfile::Custom;
            }
            KeyCode::Enter => {
                app.show_confirm_dialog = true;
            }
            _ => {}
        },

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
