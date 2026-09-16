# 10 — Component library

## Phạm vi

Các component dùng chung nằm trong `libs/babydra-ui-kit/src/components/`. Module `prelude` gom các API thường dùng; module gốc vẫn được giữ để truy cập theo nhóm chức năng.

## Bản đồ module

| Module | API chính | Mục đích |
| :--- | :--- | :--- |
| `buttons` | `create_button`, `create_accent_button`, `create_fab`, `create_icon_button`, `create_icon_btn` | Button văn bản, primary, FAB và icon. |
| `badge` | `create_icon_badge` | Badge icon cho trạng thái hoặc placeholder. |
| `cards` | `create_card`, `create_css_card`, `create_collapsible_card`, `create_switch_card`, `create_title`, `create_subtitle` | Card thường, card có CSS riêng, card thu gọn và card có switch. |
| `list_group` | `create_list_row`, `clear_box`, `clear_list_box` | Row chuẩn và refresh danh sách. |
| `modals` | `PasswordDialog`, `Wifi*Dialog`, `Vpn*Dialog`, `ModernDialogBuilder` | Overlay dialog có style thống nhất. |
| `popovers` | `create_popover`, `TooltipPopover`, `TooltipRow` | Popover neo theo widget và tooltip card. |
| `slider` | `CustomSlider`, `PillSlider`, `bind_debounced_slider` | Slider có range, tick, drag và debounce. |
| `switch` | `CustomSwitch`, `create_switch`, `ToggleRow` | Toggle Cairo-drawn và row có label. |
| `placeholder` | `create_placeholder`, `PlaceholderState` | Empty, loading và disabled state. |
| `wifi` | `create_rssi_icon`, `create_sys_wifi_icon`, `create_wifi_net_icon` | Icon tín hiệu Wi-Fi. |
| `explore` | items, file actions, dialogs, drag/drop | Thành phần riêng cho file manager nhưng vẫn dùng style chung. |

## Khởi tạo

Application GTK cần gọi theme trước khi tạo widget:

```rust
use babydra_ui_kit::prelude::*;

init_theme();
let card = create_card(gtk4::Orientation::Vertical, 12);
```

`init_theme` nạp shared CSS, theme color layer, icon search path và đồng bộ color scheme. Không tự tạo `CssProvider` riêng cho màu chung.

## Button

```rust
let primary = create_accent_button("Connect");
let icon_button = create_icon_button(
    "settings",
    18,
    &["flat", "circular"],
    Some("Open settings"),
    || {},
);
let fab = create_fab("plus");
```

`create_button` dùng style button thông thường; `create_accent_button` dành cho primary action; `create_fab` dành cho hành động nổi ở cuối vùng nội dung. Icon-only control phải có tooltip hoặc accessible label.

## Card và row

```rust
let card = create_card(gtk4::Orientation::Vertical, 12);
card.append(&create_title("Network"));
card.append(&create_subtitle("Available connections"));

let row = create_list_row("wifi", "Office", "Connected", None::<&gtk4::Widget>);
let (scroll, list) = create_scroll_list("settings-card-list");
list.append(&row);
```

`create_card` thêm class `settings-card`. `create_collapsible_card` trả về `CollapsibleCard` gồm `container`, `content`, `revealer`, label và header button để caller bổ sung nội dung hoặc truy cập trạng thái mở rộng.

`create_switch_card` trả về `(gtk4::Box, CustomSwitch)` để caller nhận event switch mà vẫn dùng layout card chuẩn.

Khi cập nhật danh sách:

```rust
clear_list_box(&list);
for item in items {
    list.append(&create_list_row(
        "folder",
        &item.name,
        &item.detail,
        None::<&gtk4::Widget>,
    ));
}
```

## Switch và slider

```rust
let toggle = create_switch(false, |active| {
    // persist and apply setting
});

let slider = CustomSlider::new_range(0, 100, 5, 50, |value| {
    // apply value
});
slider.set_value(75);
```

`CustomSwitch` kích thước vẽ mặc định `46×24`, có animation ease-out khoảng `160ms`, callback qua `connect_state_set` và setter `set_active`. `CustomSlider` vẽ track, active track, tick, label và knob; có click, drag, `value`, `set_value`, `connect_change`. `PillSlider` dùng cho control dạng pill như volume.

Không dùng `gtk4::Switch` hoặc `gtk4::Scale` thô trong vùng đã có component tương ứng. Nếu behavior mới cần dùng nhiều app, mở rộng component chung.

## Modal

`ModernDialogBuilder` tạo scrim layer, card, header, badge và action row:

```rust
use babydra_ui_kit::components::modals::dialog_builder::{
    BadgeVariant, ButtonVariant, ModernDialogBuilder,
};

let dialog = ModernDialogBuilder::new(420)
    .with_badge("lock", BadgeVariant::Primary)
    .with_title("Authentication")
    .with_subtitle("Enter your password")
    .build();

dialog.add_child(&entry);
dialog.add_action_buttons(&[
    (&cancel, ButtonVariant::Cancel),
    (&confirm, ButtonVariant::Primary),
]);
```

`PasswordDialog` cung cấp `new`, `show_for`, `hide`, `connect_submit`; khi submit, password entry được xóa trước khi gọi callback. Các dialog Wi-Fi, VPN và account dùng cùng pattern: dữ liệu riêng nằm trong dialog module, khung và button style dùng builder chung.

## Popover

```rust
let popover = create_popover(&anchor, gtk4::PositionType::Bottom, "status-popover");
```

`TooltipPopover` hỗ trợ title, key/value rows, `parse_rows`, suppress callback và hover trên cả anchor lẫn popover. Hover leave đợi khoảng `150ms` trước khi đóng để con trỏ có thể di chuyển từ anchor vào popover.

Popover cần được dispose hoặc popdown khi parent bị thay thế. Không tạo timer mới ở từng application nếu helper chung đã đáp ứng behavior.

## Placeholder

```rust
use babydra_ui_kit::prelude::{create_placeholder, PlaceholderState};

let loading = create_placeholder(PlaceholderState::Loading);
let empty = create_placeholder(PlaceholderState::Empty {
    title_key: "files.empty",
    desc_key: Some("files.empty_description"),
    icon_name: "folder",
});
```

`PlaceholderState` có ba dạng:

- `Loading`: spinner và text loading từ i18n;
- `Empty`: icon, title và description tùy chọn;
- `Disabled`: icon, title và description bắt buộc.

## Quy tắc thêm component

1. Chỉ đưa vào ui-kit khi component được dùng lại hoặc có behavior/style chung rõ ràng.
2. Public API phải tối thiểu, tên method phải phản ánh hành vi thực tế.
3. Dùng token và class CSS chung; không đặt màu literal trong component mới.
4. Widget có state phải có setter/getter hoặc callback cần thiết để caller đồng bộ state.
5. Kiểm tra dark/light, keyboard focus, hover, pressed, disabled và text dài.
6. Thêm test cho easing, parser, state transition hoặc helper thuần khi có thể.
7. Cập nhật docs này nếu thêm hoặc đổi public API.

Không tạo wrapper chỉ để đổi tên một GTK widget. Component mới phải cung cấp behavior, layout hoặc style có ý nghĩa dùng chung.
