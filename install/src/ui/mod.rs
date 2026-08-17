pub mod layout;
pub mod modals;
pub mod steps;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;
use crate::models::WizardStep;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Main layout: Header -> (Sidebar + Content) -> Footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(size);

    layout::draw_header(f, app, main_chunks[0]);

    // Body split: Left Sidebar (32 chars) + Right Content
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Min(40)])
        .split(main_chunks[1]);

    layout::draw_sidebar(f, app, body_chunks[0]);
    draw_content(f, app, body_chunks[1]);

    layout::draw_footer(f, app, main_chunks[2]);

    // Modals
    if app.is_editing_path {
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
