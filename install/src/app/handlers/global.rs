use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::models::{InstallState, LogLevel, WizardStep};

use super::super::state::App;

pub fn handle_global_keys(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
            true
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true;
            true
        }
        KeyCode::Char('?') => {
            app.show_help = true;
            true
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.is_editing_path = true;
            true
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.rescan_binaries();
            true
        }
        KeyCode::Char('i') | KeyCode::Char('I') => {
            if app.install_state != InstallState::Installing {
                app.show_confirm_dialog = true;
            }
            true
        }

        // Direct Number Jumping (1-6, 0 = summary)
        KeyCode::Char('1') => {
            app.set_step(WizardStep::Welcome);
            true
        }
        KeyCode::Char('2') => {
            app.set_step(WizardStep::SourceBranch);
            true
        }
        KeyCode::Char('3') => {
            app.set_step(WizardStep::Binaries);
            true
        }
        KeyCode::Char('4') => {
            app.set_step(WizardStep::VariantSelection);
            true
        }
        KeyCode::Char('5') => {
            app.set_step(WizardStep::ExecuteInstall);
            true
        }
        KeyCode::Char('6') | KeyCode::Char('0') => {
            app.set_step(WizardStep::Summary);
            true
        }

        // Step Navigation (Tab / n / Right arrow = Next, BackTab / p / Left arrow = Prev)
        KeyCode::Right | KeyCode::Tab | KeyCode::Char('n') => {
            if app.current_step == WizardStep::SourceBranch {
                if app.branches.is_empty() {
                    app.add_log(
                        LogLevel::Error,
                        "No installation releases available to continue.",
                    );
                } else if app.is_build_from_source() {
                    app.start_branch_switch();
                } else {
                    app.next_step();
                }
            } else {
                app.next_step();
            }
            true
        }
        KeyCode::Left | KeyCode::BackTab | KeyCode::Char('p') => {
            if app.current_step != WizardStep::ExecuteInstall
                || app.install_state != InstallState::Installing
            {
                app.prev_step();
            }
            true
        }

        _ => false,
    }
}
