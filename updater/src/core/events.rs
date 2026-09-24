use crossterm::event::KeyCode;
use std::sync::mpsc::Sender;
use std::thread;

use crate::actions::runner::{CommandRunner, LogMessage};
use crate::actions::*;
use crate::core::app::App;
use crate::core::state::*;
use crate::utils::sudo::{check_sudo_cached, validate_sudo_password};

pub fn handle_key_event(app: &mut App, code: KeyCode, tx: &Sender<LogMessage>) {
    match app.view_state {
        ViewState::Menu => match code {
            KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Char('?') => app.view_state = ViewState::HelpModal,
            KeyCode::Up | KeyCode::Char('k') => app.menu_up(),
            KeyCode::Down | KeyCode::Char('j') => app.menu_down(),
            KeyCode::Enter => {
                let action_id = app.selected_action().id;
                match action_id {
                    ActionId::Quit => app.should_quit = true,
                    ActionId::FactoryReset => {
                        app.reset_mode_selected = 0;
                        app.view_state = ViewState::FactoryResetModal;
                    }
                    ActionId::ComponentRestart => {
                        app.component_selected = 0;
                        app.view_state = ViewState::ComponentModal;
                    }
                    _ => {
                        trigger_action_with_auth(app, action_id, tx.clone());
                    }
                }
            }
            _ => {}
        },
        ViewState::SudoModal => match code {
            KeyCode::Esc => {
                app.sudo_password_input.clear();
                app.sudo_error_message = None;
                app.pending_sudo_action = None;
                app.view_state = ViewState::Menu;
            }
            KeyCode::Backspace => {
                app.sudo_password_input.pop();
            }
            KeyCode::Char(c) => {
                app.sudo_password_input.push(c);
            }
            KeyCode::Enter => {
                if validate_sudo_password(&app.sudo_password_input) {
                    app.sudo_password_input.clear();
                    app.sudo_error_message = None;

                    if let Some(action) = app.pending_sudo_action.take() {
                        if action == ActionId::FactoryReset {
                            let mode = match app.reset_mode_selected {
                                0 => ResetMode::KeepPackages,
                                1 => ResetMode::RemoveShellPackages,
                                2 => ResetMode::DryRun,
                                _ => ResetMode::RemoveAllApps,
                            };
                            spawn_reset(app, mode, tx.clone());
                        } else {
                            spawn_action(app, action, tx.clone());
                        }
                    } else {
                        app.view_state = ViewState::Menu;
                    }
                } else {
                    app.sudo_password_input.clear();
                    app.sudo_error_message =
                        Some("Incorrect sudo password. Please try again.".to_string());
                }
            }
            _ => {}
        },
        ViewState::Running => handle_log_navigation(app, code),
        ViewState::Finished => match code {
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char(' ') => {
                app.return_to_menu_at = None;
                app.view_state = ViewState::Menu;
                app.status_message = if app.last_exit_code == Some(0) {
                    format!(
                        "{} completed successfully. Crates running in background.",
                        app.active_action_name.as_deref().unwrap_or("Operation")
                    )
                } else {
                    "Ready. Select an action.".to_string()
                };
            }
            _ => handle_log_navigation(app, code),
        },
        ViewState::FactoryResetModal => match code {
            KeyCode::Esc => {
                app.view_state = ViewState::Menu;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.reset_mode_selected > 0 {
                    app.reset_mode_selected -= 1;
                } else {
                    app.reset_mode_selected = 4;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.reset_mode_selected < 4 {
                    app.reset_mode_selected += 1;
                } else {
                    app.reset_mode_selected = 0;
                }
            }
            KeyCode::Enter => {
                if app.reset_mode_selected == 4 {
                    app.view_state = ViewState::Menu;
                } else {
                    let mode = match app.reset_mode_selected {
                        0 => ResetMode::KeepPackages,
                        1 => ResetMode::RemoveShellPackages,
                        2 => ResetMode::DryRun,
                        _ => ResetMode::RemoveAllApps,
                    };

                    if mode != ResetMode::DryRun && !check_sudo_cached() {
                        app.pending_sudo_action = Some(ActionId::FactoryReset);
                        app.view_state = ViewState::SudoModal;
                    } else {
                        spawn_reset(app, mode, tx.clone());
                    }
                }
            }
            _ => {}
        },
        ViewState::ComponentModal => match code {
            KeyCode::Esc => {
                app.view_state = ViewState::Menu;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if app.component_selected > 0 {
                    app.component_selected -= 1;
                } else {
                    app.component_selected = 6;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.component_selected < 6 {
                    app.component_selected += 1;
                } else {
                    app.component_selected = 0;
                }
            }
            KeyCode::Enter => match app.component_selected {
                0 => spawn_component(app, ComponentTarget::Panel, tx.clone()),
                1 => spawn_component(app, ComponentTarget::Desktop, tx.clone()),
                2 => spawn_component(app, ComponentTarget::Switcher, tx.clone()),
                3 => spawn_component(app, ComponentTarget::Keymap, tx.clone()),
                4 => spawn_component(app, ComponentTarget::ReconfigureLabwc, tx.clone()),
                5 => spawn_component(app, ComponentTarget::RefreshGtkFonts, tx.clone()),
                _ => app.view_state = ViewState::Menu,
            },
            _ => {}
        },
        ViewState::HelpModal => match code {
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                app.view_state = ViewState::Menu;
            }
            _ => {}
        },
    }
}

fn handle_log_navigation(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Char(' ') => app.autoscroll = !app.autoscroll,
        KeyCode::Up | KeyCode::Char('k') => app.scroll_logs_up(1),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_logs_down(1),
        KeyCode::PageUp => app.scroll_logs_up(15),
        KeyCode::PageDown => app.scroll_logs_down(15),
        _ => {}
    }
}

fn trigger_action_with_auth(app: &mut App, action_id: ActionId, tx: Sender<LogMessage>) {
    let requires_sudo = app.selected_action().requires_sudo;

    if requires_sudo && !check_sudo_cached() {
        app.pending_sudo_action = Some(action_id);
        app.sudo_password_input.clear();
        app.sudo_error_message = None;
        app.view_state = ViewState::SudoModal;
    } else {
        spawn_action(app, action_id, tx);
    }
}

pub fn spawn_action(app: &mut App, action_id: ActionId, tx: Sender<LogMessage>) {
    app.view_state = ViewState::Running;
    app.clear_logs();
    app.active_action_name = Some(app.selected_action().title.to_string());
    app.status_message = format!("Running: {}...", app.selected_action().title);

    if action_id == ActionId::UpdateReload {
        app.start_file_logging("update", "Hot Update & Reload");
    }

    let repo_root = app.repo_root.clone();
    let config = app.config.clone();

    thread::spawn(move || {
        let runner = CommandRunner::new(tx);
        let res = match action_id {
            ActionId::UpdateReload => execute_update(&runner, &repo_root, &config),
            ActionId::SafetyCheck => execute_check(&runner, &repo_root),
            ActionId::StartDesktop => execute_start(&runner, &repo_root, &config),
            ActionId::SyncConfigs => sync_all_configs(&runner, &repo_root, &config),
            ActionId::CleanWorkspace => {
                runner.step("Cleaning workspace build artifacts (cargo clean)...");
                let r = runner.run_cmd("cargo", &["clean"], Some(&repo_root));
                if r.is_ok() {
                    runner.success("Build artifacts cleaned.");
                }
                r
            }
            _ => Ok(()),
        };

        runner.done(if res.is_ok() { 0 } else { 1 });
    });
}

pub fn spawn_reset(app: &mut App, mode: ResetMode, tx: Sender<LogMessage>) {
    app.view_state = ViewState::Running;
    app.clear_logs();
    app.active_action_name = Some(format!("Factory Reset ({:?})", mode));
    app.status_message = "Executing factory reset...".to_string();

    thread::spawn(move || {
        let runner = CommandRunner::new(tx);
        let res = execute_factory_reset(&runner, mode);
        runner.done(if res.is_ok() { 0 } else { 1 });
    });
}

pub fn spawn_component(app: &mut App, target: ComponentTarget, tx: Sender<LogMessage>) {
    app.view_state = ViewState::Running;
    app.clear_logs();
    app.active_action_name = Some(format!("Restart {:?}", target));
    app.status_message = "Restarting component...".to_string();

    let config = app.config.clone();

    thread::spawn(move || {
        let runner = CommandRunner::new(tx);
        let res = execute_component_restart(&runner, target, &config);
        runner.done(if res.is_ok() { 0 } else { 1 });
    });
}
