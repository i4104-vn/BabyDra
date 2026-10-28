pub mod modal;
pub mod render;

use std::cell::RefCell;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Widget;

use babydra_core::services::system::startup::{
    add_startup_cmd, delete_startup_cmd, get_startup_commands, update_startup_cmd,
};

type RefreshCallback = Rc<dyn Fn()>;

/// Refreshes the card grid with current autostart commands.
fn refresh_cards(
    grid: &crate::widgets::helpers::ResponsiveCardGrid,
    stack: &gtk4::Stack,
    modal_dialog: &modal::StartupAddDialog,
    trigger_refresh: RefreshCallback,
) {
    grid.clear();
    let cmds = get_startup_commands();

    if cmds.is_empty() {
        stack.set_visible_child_name("empty");
        return;
    }

    stack.set_visible_child_name("cards");

    for cmd in cmds {
        let cmd_id = cmd.id;
        let refresh_fn = trigger_refresh.clone();
        let modal_clone = modal_dialog.clone();
        let cmd_clone = cmd.clone();

        let card = render::create_card(
            &cmd,
            move || {
                match delete_startup_cmd(cmd_id) {
                    Ok(_) => {
                        let title = babydra_core::i18n::trans("settings.startup_deleted_title");
                        let msg = babydra_core::i18n::trans("settings.startup_deleted_msg");
                        babydra_core::send_settings_notif(&title, &msg);
                        refresh_fn();
                    }
                    Err(e) => {
                        let title = babydra_core::i18n::trans("settings.notif_startup_failed_title");
                        let msg = format!(
                            "{}: {}",
                            babydra_core::i18n::trans("settings.notif_startup_failed_msg"),
                            e
                        );
                        babydra_core::send_settings_notif(&title, &msg);
                    }
                }
            },
            move || {
                modal_clone.show_edit(&cmd_clone);
            },
        );
        grid.append(&card);
    }
}

/// Creates a new startup applications settings widget.
pub fn create_startup() -> Widget {
    let (widget, add_dialog) = render::build_startup_ui();

    let card_grid = widget.card_grid.clone();
    let stack = widget.stack.clone();

    // Self-referential closure pattern for refresh triggers
    let refresh_slot: Rc<RefCell<Option<RefreshCallback>>> = Rc::new(RefCell::new(None));

    let card_grid_c = card_grid.clone();
    let stack_c = stack.clone();
    let dialog_c = add_dialog.clone();
    let refresh_slot_c = refresh_slot.clone();

    let do_refresh = Rc::new(move || {
        let trigger = refresh_slot_c.borrow().as_ref().unwrap().clone();
        refresh_cards(&card_grid_c, &stack_c, &dialog_c, trigger);
    });

    *refresh_slot.borrow_mut() = Some(do_refresh.clone());

    // Initial load
    do_refresh();

    // Wire Add button in header to open modal in add mode
    let add_dialog_show = add_dialog.clone();
    widget.add_btn.connect_clicked(move |_| {
        add_dialog_show.show_add();
    });

    // Wire modal submission (handles both Add and Edit)
    let refresh_for_submit = do_refresh.clone();
    add_dialog.connect_submit(move |id_opt, name, cmd| {
        let res = if let Some(id) = id_opt {
            update_startup_cmd(id, name, &cmd)
        } else {
            add_startup_cmd(name, &cmd)
        };

        match res {
            Ok(_) => {
                let (title_key, msg_key) = if id_opt.is_some() {
                    ("settings.startup_updated_title", "settings.startup_updated_msg")
                } else {
                    ("settings.startup_added_title", "settings.startup_added_msg")
                };
                let title = babydra_core::i18n::trans(title_key);
                let msg = babydra_core::i18n::trans(msg_key);
                babydra_core::send_settings_notif(&title, &msg);
                refresh_for_submit();
            }
            Err(e) => {
                let title = babydra_core::i18n::trans("settings.notif_startup_failed_title");
                let msg = format!(
                    "{}: {}",
                    babydra_core::i18n::trans("settings.notif_startup_failed_msg"),
                    e
                );
                babydra_core::send_settings_notif(&title, &msg);
            }
        }
    });

    widget.root.into()
}
