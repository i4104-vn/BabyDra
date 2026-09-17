//! Area selection and folder navigation action handlers.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use gtk4::Label;

use babydra_core::models::recording::{RecordingConfig, RecordingMode};
use babydra_core::services::recording::{get_recordings_dir, select_geometry_str_with_slurp};
use crate::island::ui::IslandPopover;

/// Opens the recordings directory in the system file manager.
pub fn open_recordings_dir_action() {
    let _ = std::process::Command::new("xdg-open")
        .arg(get_recordings_dir())
        .spawn();
}

/// Triggers slurp area selection while temporarily closing the popover,
/// then reopens the popover and updates configuration upon completion.
pub fn select_recording_area_action(
    popover: &IslandPopover,
    area_label: &Label,
    config: &Rc<RefCell<RecordingConfig>>,
) {
    popover.popdown();

    let (sender, receiver) = tokio::sync::oneshot::channel::<Option<String>>();

    std::thread::spawn(move || {
        // Wait briefly to allow the compositor to unmap the popover window before slurp takes a snapshot
        std::thread::sleep(Duration::from_millis(150));
        let res = select_geometry_str_with_slurp();
        let _ = sender.send(res);
    });

    let popover = popover.clone();
    let area_label = area_label.clone();
    let config = config.clone();

    glib::MainContext::default().spawn_local(async move {
        if let Ok(Some(geometry)) = receiver.await {
            area_label.set_text(&geometry);
            config.borrow_mut().mode = RecordingMode::Window(geometry);
        }
        popover.popup();
    });
}
