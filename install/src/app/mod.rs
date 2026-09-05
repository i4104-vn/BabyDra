pub mod actions;
pub mod handlers;
pub mod state;
pub mod worker;

use crossterm::event::KeyEvent;

pub use handlers::handle_key_event;
pub use state::{App, BranchSwitchStatus};

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        handlers::handle_key_event(self, key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::WizardStep;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_execute_install_auto_confirm_modal() {
        let mut app = App::new();

        assert_eq!(app.current_step, WizardStep::Welcome);
        assert!(!app.show_confirm_dialog);

        // Jump or navigate to Step 5
        app.set_step(WizardStep::ExecuteInstall);
        assert_eq!(app.current_step, WizardStep::ExecuteInstall);
        assert!(
            app.show_confirm_dialog,
            "Modal should auto-open on ExecuteInstall step"
        );

        // Cancel modal with 'Esc' -> returns to VariantSelection
        app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(!app.show_confirm_dialog, "Modal should dismiss on Esc");
        assert_eq!(app.current_step, WizardStep::VariantSelection);

        // Navigate forward from VariantSelection to ExecuteInstall
        app.next_step();
        assert_eq!(app.current_step, WizardStep::ExecuteInstall);
        assert!(
            app.show_confirm_dialog,
            "Modal should auto-open when navigating forward into ExecuteInstall"
        );

        // Cancel modal with 'Left' / 'b' -> also returns to VariantSelection
        app.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        assert!(!app.show_confirm_dialog);
        assert_eq!(app.current_step, WizardStep::VariantSelection);
    }

    #[test]
    fn test_active_source_dir() {
        let mut app = App::new();
        if !app.branches.is_empty() {
            assert_eq!(
                app.active_source_dir(),
                app.workspace_root.join("branches").join(&app.selected_branch)
            );
        } else {
            assert_eq!(app.active_source_dir(), app.workspace_root);
        }

        app.selected_branch.clear();
        assert_eq!(app.active_source_dir(), app.workspace_root);

        app.selected_branch = "feature-test".to_string();
        assert_eq!(
            app.active_source_dir(),
            app.workspace_root.join("branches").join("feature-test")
        );
    }

    #[test]
    fn test_default_branch_release_selection() {
        let app = App::new();
        if app.branches.iter().any(|b| b.name == "release") {
            assert_eq!(app.selected_branch, "release");
            assert!(app.branches[app.branch_cursor].selected);
            assert_eq!(app.branches[app.branch_cursor].name, "release");
        }
    }
}
