//! Default Applications UI builder.

use babydra_core::i18n::trans;
use babydra_core::services::system::default_apps::{
    get_available_browsers, get_available_file_managers, get_available_terminals,
    get_default_browser, get_default_file_manager, get_default_terminal, AppChoice,
};
use babydra_ui_kit::components::cards::create_collapsible_card;
use babydra_ui_kit::components::create_list_row;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, DropDown, StringList};

/// Holds widget references for Default Applications card.
pub struct DefaultAppsWidgets {
    pub container: GtkBox,
    pub browser_dropdown: DropDown,
    pub file_manager_dropdown: DropDown,
    pub terminal_dropdown: DropDown,
}

fn create_app_dropdown(choices: &[AppChoice], current_default: &str) -> DropDown {
    let names: Vec<&str> = if choices.is_empty() {
        vec!["Default"]
    } else {
        choices.iter().map(|c| c.name.as_str()).collect()
    };
    let dropdown = DropDown::new(
        Some(StringList::new(&names)),
        Option::<gtk4::Expression>::None,
    );
    dropdown.set_valign(Align::Center);
    if let Some(pos) = choices.iter().position(|c| c.desktop_id == current_default) {
        dropdown.set_selected(pos as u32);
    }
    dropdown
}

/// Renders the Default Applications collapsible card and returns widgets + choice lists.
pub fn render_default_apps_card() -> (
    DefaultAppsWidgets,
    Vec<AppChoice>,
    Vec<AppChoice>,
    Vec<AppChoice>,
) {
    let card = create_collapsible_card(
        &trans("settings.general_default_apps"),
        Some(&trans("settings.general_default_apps_desc")),
        Some("th-large"),
        false,
    );

    // 1. Web Browser
    let browsers = get_available_browsers();
    let cur_browser = get_default_browser();
    let browser_dropdown = create_app_dropdown(&browsers, &cur_browser);
    let browser_row = create_list_row(
        "",
        &trans("settings.general_default_browser"),
        "",
        Some(&browser_dropdown),
    );
    card.content.append(&browser_row);

    // 2. File Manager
    let file_managers = get_available_file_managers();
    let cur_fm = get_default_file_manager();
    let file_manager_dropdown = create_app_dropdown(&file_managers, &cur_fm);
    let fm_row = create_list_row(
        "",
        &trans("settings.general_default_file_manager"),
        "",
        Some(&file_manager_dropdown),
    );
    card.content.append(&fm_row);

    // 3. Terminal
    let terminals = get_available_terminals();
    let cur_term = get_default_terminal();
    let terminal_dropdown = create_app_dropdown(&terminals, &cur_term);
    let term_row = create_list_row(
        "",
        &trans("settings.general_default_terminal"),
        "",
        Some(&terminal_dropdown),
    );
    card.content.append(&term_row);

    let widgets = DefaultAppsWidgets {
        container: card.container,
        browser_dropdown,
        file_manager_dropdown,
        terminal_dropdown,
    };

    (widgets, browsers, file_managers, terminals)
}
