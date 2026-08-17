pub mod layout;
pub mod modals;
pub mod steps;
pub mod theme;

pub use theme::THEME;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;
use crate::models::WizardStep;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Main layout: Header -> Body (No footer)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
        ])
        .split(size);

    layout::draw_header(f, app, main_chunks[0]);

    // Full screen width for installation progress and summary steps so logs are not cramped
    let is_full_width_step = matches!(
        app.current_step,
        WizardStep::ExecuteInstall | WizardStep::Summary
    );

    if is_full_width_step {
        draw_content(f, app, main_chunks[1]);
    } else {
        // Layout: Main Content (spacious) + Right Floating Shortcuts Box
        let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(45),
                Constraint::Length(25),
            ])
            .split(main_chunks[1]);

        draw_content(f, app, body_chunks[0]);
        layout::draw_shortcuts_panel(f, app, body_chunks[1]);
    }

    // Modals
    if app.show_branch_switching_modal {
        modals::draw_branch_switching_modal(f, app, size);
    } else if app.is_editing_path {
        modals::draw_edit_path_modal(f, app, size);
    } else if app.show_confirm_dialog {
        modals::draw_confirm_modal(f, app, size);
    } else if app.show_sudo_modal {
        modals::draw_sudo_modal(f, app, size);
    } else if app.show_help {
        modals::draw_help_modal(f, size);
    }
}

fn draw_content(f: &mut Frame, app: &App, area: Rect) {
    match app.current_step {
        WizardStep::Welcome => steps::draw_welcome_step(f, app, area),
        WizardStep::SourceBranch => steps::draw_branch_step(f, app, area),
        WizardStep::SystemPackages => steps::draw_system_packages_step(f, app, area),
        WizardStep::Binaries => steps::draw_binaries_step(f, app, area),
        WizardStep::VarLibBundle => steps::draw_varlib_bundle_step(f, app, area),
        WizardStep::ConfigsThemes => steps::draw_configs_themes_step(f, app, area),
        WizardStep::VariantSelection => steps::draw_variant_step(f, app, area),
        WizardStep::DisplayManager => steps::draw_display_manager_step(f, app, area),
        WizardStep::ExecuteInstall => steps::draw_execute_install_step(f, app, area),
        WizardStep::Summary => steps::draw_summary_step(f, app, area),
    }
}
use ratatui::layout::Rect;
