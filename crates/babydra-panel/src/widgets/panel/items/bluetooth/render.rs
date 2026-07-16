pub fn create_bluetooth_tile() -> gtk4::Button {
    let title = babydra_common::i18n::t("control.bluetooth");
    let subtitle = babydra_common::i18n::t("control.not_connected");
    
    let (btn, _) = babydra_utils::components::create_toggle_tile(
        "bluetooth",
        &title,
        &subtitle,
        "",
        false,
        |_is_active| {}
    );
    btn
}
