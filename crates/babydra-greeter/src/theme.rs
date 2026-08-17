/// Load css.
pub fn load_css() {
    tracing::info!(target: "babydra-greeter", "Loading inline CSS stylesheet into CssProvider");
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        r#"
        * {
            font-family: 'Segoe UI Variable Static Text', 'Segoe UI Variable Display', 'Segoe UI', 'Inter', system-ui, -apple-system, sans-serif;
            outline: none;
            outline-width: 0px;
            outline-color: transparent;
            outline-style: none;
        }

        *:focus, *:focus-within {
            outline: none;
            outline-width: 0px;
            outline-color: transparent;
            outline-style: none;
            box-shadow: none;
        }

        window {
            background-color: transparent;
            font-family: 'Segoe UI Variable Static Text', 'Segoe UI Variable Display', 'Segoe UI', 'Inter', system-ui, -apple-system, sans-serif;
        }

        .greeter-tint {
            background: radial-gradient(circle at center, rgba(0, 0, 0, 0.25) 0%, rgba(0, 0, 0, 0.65) 100%);
        }

        .top-bar {
            padding: 24px 36px;
        }

        .clock-time {
            font-size: 72px;
            font-weight: 800;
            color: #ffffff;
            letter-spacing: -1.5px;
            text-shadow: 0 4px 24px rgba(0, 0, 0, 0.7);
        }

        .clock-date {
            font-size: 18px;
            font-weight: 600;
            color: rgba(255, 255, 255, 0.85);
            letter-spacing: 0.5px;
            margin-top: -2px;
            text-shadow: 0 2px 12px rgba(0, 0, 0, 0.6);
        }

        .top-pill {
            background: rgba(0, 0, 0, 0.4);
            border: 1px solid rgba(255, 255, 255, 0.15);
            border-radius: 9999px;
            padding: 8px 18px;
            color: rgba(255, 255, 255, 0.9);
            font-size: 14px;
            font-weight: 600;
        }

        .power-btn {
            background: rgba(0, 0, 0, 0.4);
            border: 1px solid rgba(255, 255, 255, 0.15);
            border-radius: 9999px;
            min-width: 40px;
            min-height: 40px;
            padding: 0px;
            color: #ffffff;
            transition: all 200ms ease;
        }

        .power-btn:hover {
            background: rgba(239, 68, 68, 0.35);
            border-color: rgba(239, 68, 68, 0.6);
            color: #ff8888;
        }

        .action-btn-reboot:hover {
            background: rgba(245, 158, 11, 0.35);
            border-color: rgba(245, 158, 11, 0.6);
            color: #fcd34d;
        }

        .action-btn-suspend:hover {
            background: rgba(99, 102, 241, 0.35);
            border-color: rgba(99, 102, 241, 0.6);
            color: #a5b4fc;
        }

        /* Splash Screen Styling */
        .splash-box {
            background: rgba(13, 15, 23, 0.92);
            padding: 50px 60px;
            border-radius: 36px;
            border: 1px solid rgba(255, 255, 255, 0.12);
            box-shadow: 0 30px 80px rgba(0, 0, 0, 0.7);
            transition: opacity 600ms ease-out;
        }

        .splash-logo-wrapper {
            background: radial-gradient(circle, rgba(99, 102, 241, 0.25) 0%, rgba(13, 15, 23, 0) 70%);
            padding: 12px;
            border-radius: 9999px;
            min-width: 110px;
            min-height: 110px;
        }

        .splash-title {
            font-size: 32px;
            font-weight: 800;
            color: #ffffff;
            letter-spacing: -0.5px;
            margin-top: 12px;
        }

        .splash-subtitle {
            font-size: 14px;
            font-weight: 500;
            color: rgba(255, 255, 255, 0.65);
        }

        .splash-spinner {
            margin-top: 20px;
            color: #6366f1;
        }

        /* Floating Login Center Layout */
        .login-box {
            transition: opacity 600ms ease-in;
        }

        .login-panel {
            background-color: transparent;
            padding: 0px;
            min-width: 340px;
        }

        .avatar-ring {
            border-radius: 9999px;
            padding: 0px;
            margin-bottom: 4px;
            box-shadow: none;
        }

        .avatar-inner {
            border-radius: 9999px;
            border: 2px solid rgba(255, 255, 255, 0.3);
        }

        .avatar-img {
            border-radius: 9999px;
        }

        .login-username-label {
            font-size: 24px;
            font-weight: 700;
            color: #ffffff;
            text-shadow: 0 2px 12px rgba(0, 0, 0, 0.85);
            margin-bottom: 20px;
        }

        .input-capsule {
            background: rgba(0, 0, 0, 0.48);
            border: 1px solid rgba(255, 255, 255, 0.2);
            border-radius: 9999px;
            padding: 6px 14px;
            min-width: 320px;
            box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
            transition: all 200ms ease;
            margin-bottom: 6px;
        }

        .input-capsule:focus-within {
            background: rgba(0, 0, 0, 0.68);
            border-color: rgba(255, 255, 255, 0.55);
            box-shadow: 0 0 20px rgba(255, 255, 255, 0.2);
        }

        .login-input, entry, passwordentry, text, entry text, passwordentry text {
            background: transparent;
            border: none;
            box-shadow: none;
            outline: none;
            outline-width: 0px;
            outline-color: transparent;
            outline-style: none;
            color: #ffffff;
            font-size: 15px;
            font-weight: 500;
            min-height: 38px;
            caret-color: #ffffff;
        }

        .login-input:focus, .login-input:focus-within,
        entry:focus, entry:focus-within,
        passwordentry:focus, passwordentry:focus-within,
        text:focus, text:focus-within,
        entry text:focus, passwordentry text:focus {
            background: transparent;
            border: none;
            box-shadow: none;
            outline: none;
            outline-width: 0px;
            outline-color: transparent;
            outline-style: none;
        }

        .login-dropdown, dropdown {
            background: transparent;
            border: none;
            box-shadow: none;
            color: #ffffff;
            font-size: 15px;
            font-weight: 500;
            min-height: 38px;
        }

        dropdown > button {
            background: transparent;
            border: none;
            box-shadow: none;
            color: #ffffff;
            font-size: 15px;
            font-weight: 500;
            padding: 0 4px;
            min-height: 38px;
        }

        dropdown > button:hover, dropdown > button:active, dropdown > button:checked {
            background: transparent;
            border: none;
            box-shadow: none;
            color: #ffffff;
        }

        dropdown > button image {
            color: rgba(255, 255, 255, 0.75);
        }

        popover.menu, dropdown popover {
            background: transparent;
            padding: 0px;
            margin-top: 6px;
        }

        dropdown popover contents {
            background: rgba(20, 23, 36, 0.96);
            border: 1px solid rgba(255, 255, 255, 0.18);
            border-radius: 16px;
            padding: 6px;
            box-shadow: 0 16px 40px rgba(0, 0, 0, 0.65);
        }

        dropdown popover scrolledwindow {
            min-height: 0px;
        }

        dropdown listview {
            background: transparent;
            color: #ffffff;
            padding: 2px;
        }

        dropdown listview row {
            padding: 8px 14px;
            margin: 2px 0px;
            border-radius: 10px;
            color: rgba(255, 255, 255, 0.9);
            font-size: 14px;
            font-weight: 500;
            min-height: 36px;
            transition: all 150ms ease;
        }

        dropdown listview row cell {
            padding: 4px 8px;
        }

        dropdown listview row label {
            font-size: 14px;
            font-weight: 500;
        }

        dropdown listview row image,
        dropdown listview row image.mark {
            margin-left: 16px;
            opacity: 0.9;
        }

        dropdown listview row:hover {
            background: rgba(255, 255, 255, 0.12);
            color: #ffffff;
        }

        dropdown listview row:selected {
            background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%);
            color: #ffffff;
            font-weight: 600;
            box-shadow: 0 4px 12px rgba(99, 102, 241, 0.35);
        }

        .input-icon {
            color: rgba(255, 255, 255, 0.85);
            margin-right: 6px;
        }

        .action-arrow-btn {
            background: rgba(255, 255, 255, 0.18);
            border: 1px solid rgba(255, 255, 255, 0.25);
            border-radius: 9999px;
            color: #ffffff;
            min-width: 36px;
            min-height: 36px;
            padding: 0px;
            transition: all 200ms ease;
        }

        .action-arrow-btn:hover {
            background: rgba(255, 255, 255, 0.38);
            border-color: rgba(255, 255, 255, 0.6);
        }

        .action-arrow-btn:disabled {
            opacity: 0.85;
            background: rgba(255, 255, 255, 0.15);
        }

        .action-arrow-btn spinner {
            color: #ffffff;
        }

        .error-badge {
            background: rgba(239, 68, 68, 0.25);
            border: 1px solid rgba(239, 68, 68, 0.5);
            border-radius: 9999px;
            padding: 8px 16px;
            margin-top: 14px;
        }

        .error-msg {
            color: #ffffff;
            font-weight: 600;
            font-size: 13px;
        }

        @keyframes shake {
            0% { margin-left: 0px; margin-right: 0px; }
            15% { margin-left: -12px; margin-right: 12px; }
            30% { margin-left: 12px; margin-right: -12px; }
            45% { margin-left: -8px; margin-right: 8px; }
            60% { margin-left: 8px; margin-right: -8px; }
            75% { margin-left: -4px; margin-right: 4px; }
            90% { margin-left: 4px; margin-right: -4px; }
            100% { margin-left: 0px; margin-right: 0px; }
        }

        .shake-error {
            animation: shake 400ms ease-in-out;
        }
        "#
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_USER,
    );
    tracing::info!(target: "babydra-greeter", "CSS theme loaded into default GDK Display provider successfully");
}
