//! Integration tests for ModernDialogBuilder and PasswordDialog.

use babydra_ui_kit::components::modals::{BadgeVariant, ModernDialogBuilder, PasswordDialog};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_modern_dialog_builder_and_password_dialog_wiring() {
    let _ = gtk4::init();

    // 1. Test ModernDialogBuilder
    let dialog = ModernDialogBuilder::new(420)
        .with_badge("lock", BadgeVariant::Primary)
        .with_title("Security Check")
        .with_subtitle("Please authenticate")
        .build();

    // The container (scrim layer) must contain the card widget as a child
    let first_child = dialog.container().first_child();
    assert!(first_child.is_some(), "Dialog container must contain the card!");
    assert_eq!(
        first_child.as_ref(),
        Some(&dialog.card().clone().upcast::<gtk4::Widget>()),
        "First child of container must be the dialog card"
    );

    // The card must have hexpand and vexpand enabled to center properly
    assert!(dialog.card().hexpands(), "Card must have hexpand enabled");
    assert!(dialog.card().vexpands(), "Card must have vexpand enabled");
    assert_eq!(dialog.card().halign(), gtk4::Align::Center);
    assert_eq!(dialog.card().valign(), gtk4::Align::Center);

    // Title label must match
    assert_eq!(dialog.title_label().text(), "Security Check");
    assert_eq!(dialog.subtitle_label().map(|l| l.text()), Some("Please authenticate".into()));

    // 2. Test PasswordDialog
    let auth_dlg = PasswordDialog::new("Initial Title", "Initial Sub");

    // Check card is attached to container scrim
    let card_child = auth_dlg.container.first_child();
    assert!(card_child.is_some(), "PasswordDialog container must contain the card!");

    // Dynamic prompt updates via show_for
    auth_dlg.show_for("Uninstall TestApp", "Enter sudo password for removal:");
    assert_eq!(auth_dlg.title_lbl.text(), "Uninstall TestApp");
    assert_eq!(auth_dlg.sub_lbl.as_ref().map(|l| l.text()), Some("Enter sudo password for removal:".into()));
    assert!(auth_dlg.container.is_visible(), "Container must be visible after show_for");

    // Confirm button submit wiring
    let submitted_password = Rc::new(RefCell::new(None));
    let sub_clone = submitted_password.clone();
    auth_dlg.connect_submit(move |pwd| {
        *sub_clone.borrow_mut() = pwd;
    });

    auth_dlg.password_entry.set_text("secret_password_123");
    // Simulate clicking the real confirm button
    auth_dlg.confirm_btn.emit_clicked();

    assert_eq!(
        *submitted_password.borrow(),
        Some("secret_password_123".to_string()),
        "Password submission must receive password entered into entry"
    );
    assert!(!auth_dlg.container.is_visible(), "Dialog should hide after submit");

    // Cancel button wiring
    auth_dlg.show_for("Another Title", "Another Sub");
    auth_dlg.password_entry.set_text("will_cancel");
    auth_dlg.cancel_btn.emit_clicked();

    assert!(!auth_dlg.container.is_visible(), "Dialog should hide after cancel");
    assert_eq!(auth_dlg.password_entry.text(), "", "Password entry should be cleared after cancel");
}

