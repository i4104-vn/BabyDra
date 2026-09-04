//! Signal wiring for the greeter UI: clock, splash transition, power actions and login flow.
//! Follows the `handler.rs` convention used by babydra-settings widgets, keeping all
//! event wiring out of the layout builder.

use gtk4::prelude::*;
use tokio::sync::oneshot;

use crate::auth;
use crate::render::GreeterWidgets;
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

/// Sets up clock and date labels with 1-second update timer.
fn setup_clock(top_bar: &TopBarWidget) {
    tracing::info!(target: "babydra-greeter", "Setting up top bar clock timer (interval: 1 second)");
    let (time, date) = babydra_core::format_clock_date("greeter.date_format");
    top_bar.clock_label.set_text(&time);
    top_bar.date_label.set_text(&date);
    let clock_label = top_bar.clock_label.clone();
    let date_label = top_bar.date_label.clone();
    glib::timeout_add_seconds_local(1, move || {
        let (time, date) = babydra_core::format_clock_date("greeter.date_format");
        clock_label.set_text(&time);
        date_label.set_text(&date);
        glib::ControlFlow::Continue
    });
}

// ---------------------------------------------------------------------------
// Splash screen transition
// ---------------------------------------------------------------------------

/// Sets up splash screen transition (2 second delay, then fade to login).
fn setup_splash_transition(g: &GreeterWidgets) {
    tracing::info!(target: "babydra-greeter", "Initializing splash screen transition (showing splash, hiding login panel)");
    g.login.container.set_opacity(0.0);
    g.login.container.set_visible(false);
    g.splash.container.set_visible(true);
    g.splash.container.set_opacity(1.0);

    let splash_container = g.splash.container.clone();
    let login_container = g.login.container.clone();
    let pass_entry = g.login.pass_entry.clone();

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

/// Sets up power, reboot, and suspend button handlers.
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

/// Sets up login flow: user dropdown, password entry, submit button, and Enter key.
fn setup_login_flow(g: &GreeterWidgets) {
    let login_action = create_login_action(g);
    
    let login_action_btn = login_action.clone();
    g.login.login_btn.connect_clicked(move |_| {
        login_action_btn();
    });

    g.login.pass_entry.connect_activate(move |_| {
        login_action();
    });
}

#[derive(Clone)]
struct LoginRefs {
    user_dropdown: gtk4::DropDown,
    users: Vec<String>,
    pass_entry: gtk4::PasswordEntry,
    login_btn: gtk4::Button,
    btn_spinner: gtk4::Spinner,
    power_btn: gtk4::Button,
    reboot_btn: gtk4::Button,
    suspend_btn: gtk4::Button,
    error_label: gtk4::Label,
    error_box: gtk4::Box,
    login_panel: gtk4::Box,
}

impl LoginRefs {
    fn from_greeter(g: &GreeterWidgets) -> Self {
        Self {
            user_dropdown: g.login.user_dropdown.clone(),
            users: g.login.users.clone(),
            pass_entry: g.login.pass_entry.clone(),
            login_btn: g.login.login_btn.clone(),
            btn_spinner: g.login.btn_spinner.clone(),
            power_btn: g.top_bar.power_btn.clone(),
            reboot_btn: g.top_bar.reboot_btn.clone(),
            suspend_btn: g.top_bar.suspend_btn.clone(),
            error_label: g.login.error_label.clone(),
            error_box: g.login.error_box.clone(),
            login_panel: g.login.login_panel.clone(),
        }
    }

    fn set_controls_sensitive(&self, sensitive: bool) {
        self.user_dropdown.set_sensitive(sensitive);
        self.pass_entry.set_sensitive(sensitive);
        self.login_btn.set_sensitive(sensitive);
        self.power_btn.set_sensitive(sensitive);
        self.reboot_btn.set_sensitive(sensitive);
        self.suspend_btn.set_sensitive(sensitive);
    }
}

/// Creates the login action closure with all widget references.
fn create_login_action(g: &GreeterWidgets) -> impl Fn() + Clone + 'static {
    let refs = LoginRefs::from_greeter(g);

    move || {
        if !refs.login_btn.is_sensitive() {
            return;
        }

        let selected_idx = refs.user_dropdown.selected() as usize;
        let user = refs.users.get(selected_idx).cloned().unwrap_or_default();
        let pass = refs.pass_entry.text().to_string();
        if user.is_empty() || pass.is_empty() {
            tracing::warn!(target: "babydra-greeter", "Login submit ignored: username or password is empty");
            return;
        }

        tracing::info!(target: "babydra-greeter", "Login action triggered for user: {:?}", user);

        // Disable controls and show the spinner while authentication runs
        refs.set_controls_sensitive(false);
        refs.btn_spinner.start();
        refs.login_btn.set_child(Some(&refs.btn_spinner));

        babydra_core::save_last_user(&user);
        tracing::info!(target: "babydra-greeter", "Saved last user {:?} to {:?}", user, babydra_core::services::last_user::get_last_user_path());

        let (tx, rx) = oneshot::channel();

        std::thread::spawn(move || {
            let result = auth::do_login(user, pass);
            let _ = tx.send(result);
        });

        let r = refs.clone();
        glib::MainContext::default().spawn_local(async move {
            if let Ok(result) = rx.await {
                match result {
                    Ok(_) => {
                        tracing::info!(target: "babydra-greeter", "Login authentication completed successfully!");
                        r.error_box.set_visible(false);
                    }
                    Err(err) => {
                        tracing::error!(target: "babydra-greeter", "Login authentication failed: {}", err);
                        // Re-enable controls and restore submit button label on failure
                        r.btn_spinner.stop();
                        r.login_btn.set_child(Option::<&gtk4::Widget>::None);
                        r.login_btn.set_label("➔");
                        r.set_controls_sensitive(true);

                        r.error_label.set_text(&err);
                        r.error_box.set_visible(true);
                        r.pass_entry.set_text("");
                        r.pass_entry.grab_focus();

                        r.login_panel.add_css_class("shake-error");
                        let panel = r.login_panel.clone();
                        glib::timeout_add_local(std::time::Duration::from_millis(400), move || {
                            panel.remove_css_class("shake-error");
                            glib::ControlFlow::Break
                        });
                    }
                }
            }
        });
    }
}
