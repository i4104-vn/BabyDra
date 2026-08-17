//! Signal wiring for the greeter UI: clock, splash transition, power actions and login flow.
//! Follows the `handlers.rs` convention used by babydra-settings widgets, keeping all
//! event wiring out of the layout builder.

use gtk4::prelude::*;
use tokio::sync::oneshot;

use crate::auth;
use crate::render::GreeterWidgets;
use crate::widgets;
use crate::widgets::top_bar::TopBarWidget;

/// Wires up every interactive handler on top of the pre-built greeter widgets.
pub fn setup_handlers(g: &GreeterWidgets) {
    setup_clock(&g.top_bar);
    setup_splash_transition(g);
    setup_power_buttons(&g.top_bar);
    setup_login_flow(g);
}

// ---------------------------------------------------------------------------
// Top bar clock
// ---------------------------------------------------------------------------

/// Sets up `clock`.
fn setup_clock(top_bar: &TopBarWidget) {
    tracing::info!(target: "babydra-greeter", "Setting up top bar clock timer (interval: 1 second)");
    babydra_core::update_clock(
        &top_bar.clock_label,
        &top_bar.date_label,
        "greeter.date_format",
    );
    let clock_label = top_bar.clock_label.clone();
    let date_label = top_bar.date_label.clone();
    glib::timeout_add_seconds_local(1, move || {
        babydra_core::update_clock(&clock_label, &date_label, "greeter.date_format");
        glib::ControlFlow::Continue
    });
}

// ---------------------------------------------------------------------------
// Splash screen transition
// ---------------------------------------------------------------------------

/// Sets up `splash transition`.
fn setup_splash_transition(g: &GreeterWidgets) {
    tracing::info!(target: "babydra-greeter", "Initializing splash screen transition (showing splash, hiding login panel)");
    g.login.container.set_opacity(0.0);
    g.login.container.set_visible(false);
    g.splash.container.set_visible(true);
    g.splash.container.set_opacity(1.0);

    let splash_container = g.splash.container.clone();
    let login_container = g.login.container.clone();
    let pass_entry = g.login.pass_entry.clone();

    // After 2 seconds, fade out splash screen and reveal floating login panel
    glib::timeout_add_seconds_local(2, move || {
        tracing::info!(target: "babydra-greeter", "Splash screen timer elapsed (2s): hiding splash and fading in login panel");
        splash_container.set_opacity(0.0);
        splash_container.set_visible(false);

        login_container.set_visible(true);
        login_container.set_opacity(1.0);

        pass_entry.grab_focus();
        glib::ControlFlow::Break
    });
}

// ---------------------------------------------------------------------------
// Power buttons (poweroff / reboot / suspend)
// ---------------------------------------------------------------------------

/// Sets up `power buttons`.
fn setup_power_buttons(top_bar: &TopBarWidget) {
    top_bar.power_btn.connect_clicked(|_| {
        tracing::info!(target: "babydra-greeter", "User clicked Power Off button -> babydra_core::power::poweroff()");
        babydra_core::power::poweroff();
    });
    top_bar.reboot_btn.connect_clicked(|_| {
        tracing::info!(target: "babydra-greeter", "User clicked Reboot button -> babydra_core::power::reboot()");
        babydra_core::power::reboot();
    });
    top_bar.suspend_btn.connect_clicked(|_| {
        tracing::info!(target: "babydra-greeter", "User clicked Suspend button -> babydra_core::power::suspend()");
        babydra_core::power::suspend();
    });
}

// ---------------------------------------------------------------------------
// Login flow
// ---------------------------------------------------------------------------

/// Sets up `login flow`.
fn setup_login_flow(g: &GreeterWidgets) {
    let user_dropdown = g.login.user_dropdown.clone();
    let users = g.login.users.clone();
    let pass_entry = g.login.pass_entry.clone();
    let login_btn = g.login.login_btn.clone();
    let btn_spinner = g.login.btn_spinner.clone();
    let power_btn = g.top_bar.power_btn.clone();
    let reboot_btn = g.top_bar.reboot_btn.clone();
    let suspend_btn = g.top_bar.suspend_btn.clone();
    let error_label = g.login.error_label.clone();
    let error_box = g.login.error_box.clone();
    let login_panel = g.login.login_panel.clone();

    let do_login_action = move || {
        if !login_btn.is_sensitive() {
            return;
        }

        let selected_idx = user_dropdown.selected() as usize;
        let user = users.get(selected_idx).cloned().unwrap_or_default();
        let pass = pass_entry.text().to_string();
        if user.is_empty() || pass.is_empty() {
            tracing::warn!(target: "babydra-greeter", "Login submit ignored: username or password is empty");
            return;
        }

        tracing::info!(target: "babydra-greeter", "Login action triggered for user: {:?}", user);

        // Disable controls and show the spinner while authentication runs
        user_dropdown.set_sensitive(false);
        pass_entry.set_sensitive(false);
        login_btn.set_sensitive(false);
        power_btn.set_sensitive(false);
        reboot_btn.set_sensitive(false);
        suspend_btn.set_sensitive(false);

        btn_spinner.start();
        login_btn.set_child(Some(&btn_spinner));

        if let Err(e) = std::fs::write(widgets::LAST_USER_FILE, &user) {
            tracing::warn!(target: "babydra-greeter", "Failed to save last user to {:?}: {}", widgets::LAST_USER_FILE, e);
        } else {
            tracing::info!(target: "babydra-greeter", "Saved last user {:?} to {:?}", user, widgets::LAST_USER_FILE);
        }

        let (tx, rx) = oneshot::channel();

        std::thread::spawn(move || {
            let result = auth::do_login(user, pass);
            let _ = tx.send(result);
        });

        let user_dropdown_c = user_dropdown.clone();
        let pass_entry_c = pass_entry.clone();
        let login_btn_c = login_btn.clone();
        let btn_spinner_c = btn_spinner.clone();
        let power_btn_c = power_btn.clone();
        let reboot_btn_c = reboot_btn.clone();
        let suspend_btn_c = suspend_btn.clone();
        let error_label_c = error_label.clone();
        let error_box_c = error_box.clone();
        let login_panel_c = login_panel.clone();

        glib::MainContext::default().spawn_local(async move {
            if let Ok(result) = rx.await {
                match result {
                    Ok(_) => {
                        tracing::info!(target: "babydra-greeter", "Login authentication completed successfully!");
                        error_box_c.set_visible(false);
                    }
                    Err(err) => {
                        tracing::error!(target: "babydra-greeter", "Login authentication failed: {}", err);
                        // Re-enable controls and restore submit button label on failure
                        btn_spinner_c.stop();
                        login_btn_c.set_child(Option::<&gtk4::Widget>::None);
                        login_btn_c.set_label("➔");

                        user_dropdown_c.set_sensitive(true);
                        pass_entry_c.set_sensitive(true);
                        login_btn_c.set_sensitive(true);
                        power_btn_c.set_sensitive(true);
                        reboot_btn_c.set_sensitive(true);
                        suspend_btn_c.set_sensitive(true);

                        error_label_c.set_text(&err);
                        error_box_c.set_visible(true);
                        pass_entry_c.set_text("");
                        pass_entry_c.grab_focus();

                        login_panel_c.add_css_class("shake-error");
                        let login_panel_cb = login_panel_c.clone();
                        glib::timeout_add_local(std::time::Duration::from_millis(400), move || {
                            login_panel_cb.remove_css_class("shake-error");
                            glib::ControlFlow::Break
                        });
                    }
                }
            }
        });
    };

    let do_login_action_btn = do_login_action.clone();
    g.login.login_btn.connect_clicked(move |_| {
        do_login_action_btn();
    });

    g.login.pass_entry.connect_activate(move |_| {
        do_login_action();
    });
}
