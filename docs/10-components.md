# 10 — Component library

## Phạm vi

Component library nằm trong `libs/babydra-ui-kit/src/components/`. Mục tiêu là cung cấp các control có cùng spacing, state, theme và accessibility.

## Bản đồ component

| Nhóm | API tiêu biểu | Thư mục |
| :--- | :--- | :--- |
| Button | `create_icon_button`, `create_close_button` | `buttons/`, `close_button/` |
| Badge | `create_status_badge`, `create_icon_badge` | `badge/` |
| Card/list | `create_card`, `create_item_row`, `create_list_row`, `create_scrollable_list` | `card/`, `list_group/` |
| Switch | `create_switch`, `ToggleRow` | `switch/` |
| Slider | `CustomSlider::new`, `CustomSlider::new_range` | `slider/` |
| Modal | `PasswordDialog`, `Wifi*Dialog`, `Vpn*Dialog` | `modal/` |
| Popover | `create_popover`, `attach_hover_popover` | `popovers/` |
| Placeholder | `create_placeholder_row` | `placeholder/` |
| Progress | `create_progress_bar`, `create_disk_progress` | `progress/` |
| Spinner | `create_spinner`, `create_loading_box` | `spinners/` |
| Tooltip | `set_tooltip` | `tooltips/` |
| Wi-Fi icon | `create_system_wifi_signal_icon` | `wifi/` |

## Button

```rust
let button = create_icon_button(
    "edit-delete",
    16,
    &["flat", "circular"],
    Some("Remove item"),
    || {},
);
```

Button hành động chính dùng class `suggested-action`; button phụ dùng style chuẩn của ui-kit. Icon-only button luôn có tooltip hoặc accessible label.

## Card và list

```rust
let card = create_card(Orientation::Vertical, 12);
card.append(&create_title("Network"));
card.append(&create_item_row("Wi-Fi", "Connected", None));

let row = create_list_row(&icon, &title, &description, Some(&right_widget));
list_box.append(&row);
```

Settings section nên dùng `create_card`. Khi refresh danh sách, gọi `clear_list_box` trước khi append dữ liệu mới để tránh giữ widget cũ.

## Switch và slider

```rust
let switch = create_switch(false, |active| {
    // apply setting
});

let slider = CustomSlider::new_range(0, 100, 5, 60, |value| {
    // update value
});
```

Không dùng `gtk4::Switch` hoặc `gtk4::Scale` thô trong UI đã có component tương ứng. Custom control giữ animation, token màu và event semantics nhất quán.

## Modal và popover

Modal chuẩn dùng overlay/card của ui-kit thay vì tạo `gtk4::Dialog` riêng cho mỗi application. Dialog phải có:

- tiêu đề và mô tả ngắn;
- primary action rõ ràng;
- cancel/close action;
- vùng hiển thị lỗi;
- focus hợp lý khi mở.

Popover cần xử lý cả pointer rời parent và pointer đi vào popover. Dùng helper hover của ui-kit khi hành vi phù hợp thay vì tự tạo timer ở mỗi application.

## Placeholder và loading

Phân biệt ba trạng thái:

| Trạng thái | Dùng khi |
| :--- | :--- |
| Empty | Dữ liệu đã tải xong nhưng không có item. |
| Loading | Đang chờ service hoặc I/O. |
| Disabled/error | Không thể thực hiện thao tác hoặc service không khả dụng. |

Dùng `create_placeholder_row(PlaceholderState)` để các trạng thái có cùng layout và i18n.

## Quy tắc khi thêm component

1. Xác định component có thực sự dùng chung hay chỉ là layout riêng của một app.
2. Đặt state và API tối thiểu trong module component.
3. Dùng token theme thay vì màu literal.
4. Cung cấp tooltip/accessible label cho control không có text.
5. Kiểm tra dark/light, hover, focus, pressed, disabled và keyboard navigation.
6. Cập nhật tài liệu này và thêm test logic nếu component có state hoặc animation phức tạp.

Không tạo một component mới chỉ để bọc một GTK widget mà không thêm hành vi, style hoặc contract dùng chung.
