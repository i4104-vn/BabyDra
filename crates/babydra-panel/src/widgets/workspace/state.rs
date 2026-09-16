use babydra_core::DesktopApp;

pub struct PopoverState {
    pub preview_popover: gtk4::Popover,
    pub tooltip_popover: gtk4::Popover,
}

pub struct TaskbarPreviewActions {
    pub action_triggers: Vec<(gtk4::Button, gtk4::Button, DesktopApp)>,
    pub open_new_info: Option<(gtk4::Button, String)>,
    pub close_all_btn_opt: Option<gtk4::Button>,
}
