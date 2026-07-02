//! Shared struct models and messaging types for the Dynamic Island widget.

pub mod notification;
pub mod widgets;

pub use notification::{ActiveNotification, NotificationMsg};
pub use widgets::IslandWidgets;

