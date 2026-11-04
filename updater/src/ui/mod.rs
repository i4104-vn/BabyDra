pub mod components;
pub mod layout;
pub mod modals;
pub mod theme;
pub mod views;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use crate::core::app::App;
use crate::core::state::ViewState;
use crate::ui::components::{render_footer, render_header};
use crate::ui::layout::make_app_layout;
use crate::ui::modals::{
    render_component_modal, render_help_modal, render_reset_modal, render_sudo_modal,
};
use crate::ui::views::{render_action_details, render_action_list, render_execution};

pub fn draw(frame: &mut Frame, app: &App) {
    let layout = make_app_layout(frame.area());

    render_header(frame, layout.header, app);

    match app.view_state {
        ViewState::Menu => {
            render_main_split(frame, layout.content, app);
        }
        ViewState::Running | ViewState::Finished => {
            render_execution(frame, layout.content, app);
        }
        ViewState::SudoModal => {
            render_main_split(frame, layout.content, app);
            render_sudo_modal(frame, frame.area(), app);
        }
        ViewState::FactoryResetModal => {
            render_main_split(frame, layout.content, app);
            render_reset_modal(frame, frame.area(), app);
        }
        ViewState::ComponentModal => {
            render_main_split(frame, layout.content, app);
            render_component_modal(frame, frame.area(), app);
        }
        ViewState::HelpModal => {
            render_main_split(frame, layout.content, app);
            render_help_modal(frame, frame.area());
        }
    }

    render_footer(frame, layout.footer, app);
}

fn render_main_split(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(area);

    render_action_list(frame, chunks[0], app);
    render_action_details(frame, chunks[1], app);
}
