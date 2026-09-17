//! Desktop notifications model.

/// Representation of an active desktop notification.
#[derive(Clone, Debug)]
pub struct ActiveNotification {
    /// Summary or title of the notification message.
    pub title: String,
    /// Detailed description body of the notification.
    pub body: String,
    /// Icon key or file path representing the sender app.
    pub icon: String,
    /// Friendly name of the app sending the notification.
    pub app_name: String,
    /// Optional command to execute when the notification is clicked.
    pub command: Option<String>,
    /// Time instant when the notification was created.
    pub timestamp: std::time::Instant,
}

/// Dynamic Island DBus communication command messages.
#[derive(Debug)]
pub enum NotificationMsg {
    /// Triggered on receiving a new notification.
    New {
        summary: String,
        body: String,
        icon: String,
        app_name: String,
        timeout: i32,
        command: Option<String>,
    },
    /// Command to close the current active popup.
    Close,
}
