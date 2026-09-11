//! UI widget handles and state structs for the panel status widgets.

use gtk4::prelude::*;
use std::rc::Rc;

pub struct NetworkWidgets {
    pub container: gtk4::Box,
    pub wifi_icon: gtk4::Image,
    pub eth_area: gtk4::DrawingArea,
}

#[derive(Clone)]
pub struct StatusPopovers {
    pub vpn_popover: gtk4::Popover,
    pub net_popover: gtk4::Popover,
    pub vol_popover: gtk4::Popover,
    pub bat_popover_opt: Option<gtk4::Popover>,
    pub update_volume_popover: Rc<dyn Fn()>,
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
