pub mod modal;
pub mod render;

use babydra_core::services::system::startup::{
    add_startup_cmd, delete_startup_cmd, get_startup_commands,
};
use gtk4::prelude::*;
use gtk4::Widget;
use std::cell::RefCell;
use std::rc::Rc;

/// Refreshes the FlowBox card grid with current autostart commands.
fn refresh_cards(
    flow_box: &gtk4::FlowBox,
    stack: &gtk4::Stack,
    trigger_refresh: Rc<dyn Fn()>,
) {
    crate::widgets::helpers::clear_flow_box(flow_box);
    let cmds = get_startup_commands();

    if cmds.is_empty() {
        stack.set_visible_child_name("empty");
        return;
    }

    stack.set_visible_child_name("cards");

    for cmd in cmds {
        let cmd_id = cmd.id;
        let refresh_fn = trigger_refresh.clone();
        let card = render::create_card(&cmd, move || {
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
        });
        flow_box.append(&card);
    }
}

/// Creates a new startup applications settings widget.
pub fn create_startup() -> Widget {
    let (widget, add_dialog) = render::build_startup_ui();

    let flow_box = widget.flow_box.clone();
    let stack = widget.stack.clone();

    // Self-referential closure pattern for refresh triggers
    let refresh_slot: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));

    let flow_box_c = flow_box.clone();
    let stack_c = stack.clone();
    let refresh_slot_c = refresh_slot.clone();

    let do_refresh = Rc::new(move || {
        let trigger = refresh_slot_c.borrow().as_ref().unwrap().clone();
        refresh_cards(&flow_box_c, &stack_c, trigger);
    });

    *refresh_slot.borrow_mut() = Some(do_refresh.clone());

    // Initial load
    do_refresh();

    // Wire Add button in header to open modal
    let add_dialog_show = add_dialog.clone();
    widget.add_btn.connect_clicked(move |_| {
        add_dialog_show.show();
    });

    // Wire modal Add submission
    let refresh_for_add = do_refresh.clone();
    add_dialog.connect_add(move |name, cmd| {
        match add_startup_cmd(name, &cmd) {
            Ok(_) => {
                let title = babydra_core::i18n::trans("settings.startup_added_title");
                let msg = babydra_core::i18n::trans("settings.startup_added_msg");
                babydra_core::send_settings_notif(&title, &msg);
                refresh_for_add();
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
