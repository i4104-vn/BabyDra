//! UI widget handles and state structs for the panel status widgets.

use gtk4::prelude::*;
use std::rc::Rc;

pub(crate) struct NetworkWidgets {
    pub(crate) container: gtk4::Box,
}

#[derive(Clone)]
pub(crate) struct StatusPopovers {
    pub(crate) vpn_popover: gtk4::Popover,
    pub(crate) net_popover: gtk4::Popover,
    pub(crate) vol_popover: gtk4::Popover,
    pub(crate) bat_popover_opt: Option<gtk4::Popover>,
    pub(crate) update_volume_popover: Rc<dyn Fn()>,
    pub(crate) current_volume: Rc<std::cell::Cell<f64>>,
    pub(crate) current_muted: Rc<std::cell::Cell<bool>>,
}

impl StatusPopovers {
    pub fn popdown_all(&self) {
        self.vpn_popover.popdown();
        self.net_popover.popdown();
        self.vol_popover.popdown();
        if let Some(ref bp) = self.bat_popover_opt {
            bp.popdown();
        }
    }
}
