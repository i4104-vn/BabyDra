# 10 — Component Library

**Phạm vi:** từng component trong `libs/babydra-ui-kit/src/components/` — API, class CSS, quy tắc.
**Phiên bản:** 2.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## 1. Bản đồ component

| Component | Entry point | File nguồn |
| :--- | :--- | :--- |
| Button | `create_icon_button`, `.suggested-action`, `.connect-pill-btn` | `components/buttons/` |
| Badge | `create_status_badge`, `create_icon_badge` | `components/badge/` |
| Card | `create_card`, `create_switch_card`, `create_scrollable_list` | `components/card/` |
| Switch | `create_switch`, `ToggleRow` | `components/switch/` |
| Slider | `CustomSlider::new`, `new_range` | `components/slider/` |
| Modal | `PasswordDialog`, `Wifi*Dialog`, `Vpn*Dialog` | `components/modal/` |
| Popover | `create_popover`, `attach_hover_popover` | `components/popovers/` |
| Navbar | `create_sidebar_row[_with_badge]` | `components/navbar/` |
| List | `create_list_row`, `clear_list_box` | `components/list_group/` |
| Placeholder | `create_placeholder_row(PlaceholderState)` | `components/placeholder/` |
| Progress | `create_progress_bar`, `create_disk_progress` | `components/progress/` |
| Spinner | `create_spinner`, `create_loading_box` | `components/spinners/` |
| Tooltip | `set_tooltip` | `components/tooltips/` |
| Close | `create_close_button[_with_label]` | `components/close_button/` |
| Wi-Fi icon | `create_system_wifi_signal_icon`, `create_wifi_signal_icon_for_network` | `components/wifi/` |

> [!NOTE]
> Module `alerts` cũ đã gộp vào `placeholder` (T2.4) — `create_placeholder_message` đã deprecated, dùng `create_placeholder_row`.

---

## 2. Button

- **Primary** (`.suggested-action`): nền `#3b82f6`, hover `#2563eb`, active `#1d4ed8`, chữ trắng 13px/600 — hành động chính duy nhất mỗi dialog.
- **Secondary** (`.connect-pill-btn`): nền kính mờ `rgba(255,255,255,0.08)` dark / `rgba(0,0,0,0.05)` light — hủy, đóng, phụ.
- Hình dạng: pill `9999px` (nút hành động) / tròn `50%` (nút icon). **Không** bo vuông, **không** transform khi hover.
- Transition: `background-color 200ms ease`.

```rust
let btn = create_icon_button("edit-delete", 16, &["flat", "circular"], Some("Forget Network"), || {});
```

---

## 3. Badge

| Loại | API | Class |
| :--- | :--- | :--- |
| Status | `create_status_badge(text, is_success)` | `success-text` / `settings-desc` |
| Icon (44px) | `create_icon_badge(icon, size, false)` | `blue-icon-badge` |
| Icon (34px) | `create_icon_badge(icon, size, true)` | `blue-icon-badge-sm` |

```rust
let badge = create_icon_badge("wifi", 16, true);
```

---

## 4. Card & List

```rust
let card = create_card(Orientation::Vertical, 12);          // class settings-card
card.append(&create_title("Network"));
card.append(&create_item_row("Wi-Fi", "Connected", Some(&badge)));

let (switch_card, sw) = create_switch_card("Bluetooth", "Toggle adapter");
let (scroll, list_box) = create_scrollable_list("settings-card-list");  // cuộn dọc

// Danh sách thường
let row = create_list_row(&icon, &title, &desc, Some(&widget));
clear_list_box(&list_box);   // refresh trước khi thêm lại
```

Quy tắc: mọi khối Settings nằm trong `create_card`; dòng item dùng `create_item_row`/`create_list_row` — không tự dựng `Box` tay.

---

## 5. Switch & Slider (vẽ Cairo)

### CustomSwitch

```rust
let sw = create_switch(false, |active| { /* … */ });
sw.set_active(true);   // tự chạy animation trượt 160ms ease-out cubic
```

- Kích thước chuẩn 46×24; bật → nền lerp `#3b82f6`; tắt → viền mờ.
- `ToggleRow` = label On/Off + switch (i18n `settings.on/off`).

### CustomSlider

```rust
let slider = CustomSlider::new(50, |v| set_brightness(v));
let vol = CustomSlider::new_range(0, 100, 5, 60, |v| volume::set_volume(v));
```

- Track 6px round cap, phần điền `#3b82f6`, knob trắng viền accent, tick marks + nhãn `%`.

> [!IMPORTANT]
> Không dùng `gtk4::Switch` / `gtk4::Scale` thô — dùng CustomSwitch/CustomSlider cho đồng nhất.

---

## 6. Modal (Dialog)

Mọi dialog: class `auth-dialog-card`, ẩn sẵn, bật qua `show_for_*`, nút chính `suggested-action` + phụ `connect-pill-btn`.

| Dialog | API nổi bật |
| :--- | :--- |
| `PasswordDialog` | `show_for(title, sub)`, `connect_submit(Option<String>)` |
| `WifiPasswordDialog` | `show_for(ssid, security)`, `set_error`, `connect_submit((pwd, user))` — `"8021x"` hiện ô username |
| `WifiInfoDialog` | `show_for(net, config)`, `connect_configure`, `connect_forget` |
| `WifiConfigDialog` | `show_for(ssid, cfg)`, segmented DHCP/Static, `connect_save` |
| `VpnConfigDialog` | `apply_config_file(path)`, `show_for_new/edit`, `connect_save/delete` |
| `VpnLogDialog` | `show_for_vpn(name)`, TextView log tô màu |

Quy tắc: không dùng `gtk4::Dialog` riêng — dùng overlay box chuẩn.

---

## 7. Popover

```rust
let card = build_hover_popover_card("Power", vec![
    HoverPopoverRow::new("Battery", "78%", Some("success-text")),
]);
let pop = create_popover_with_content(&icon, PositionType::Top, "status-popover", &card);
attach_hover_popover(&icon, &pop, Rc::new(|| { /* cập nhật dữ liệu */ }));
```

- Hover popover: vào → `update_fn()` + popup; rời → chờ 150ms → popdown (cho chuột kịp di vào); `set_autohide(false)`.

---

## 8. Các component nhỏ

| Component | API | Ghi chú |
| :--- | :--- | :--- |
| Navbar | `create_sidebar_row(label, icon)` | Row: badge 16px + label, spacing 12, class `settings-sidebar-row` |
| Placeholder | `create_placeholder_row(Disabled/Loading/Empty)` | i18n key, icon badge 44px, margin 40px |
| Progress | `create_progress_bar(fraction, class)` | track mờ, phần điền `#3b82f6`, radius 9999px |
| Spinner | `create_spinner(size)` · `create_loading_box(text)` | ưu tiên skeleton cho vùng lớn (xem 09-design.md) |
| Tooltip | `set_tooltip(widget, text)` | icon-only button **bắt buộc** có tooltip |
| Close | `create_close_button(class)` · `_with_label(text, class)` | icon `window-close` 12px, cursor pointer |
| Wi-Fi icon | `create_system_wifi_signal_icon(size, color)` | 0–4 vạch; tắt xám `#6B7280`, chưa kết nối `#9CA3AF`, đã kết nối `#3B82F6` |

---

## 9. Quy tắc chung

| DO | DO NOT |
| :--- | :--- |
| Dùng `create_*` component có sẵn | Tự dựng widget tay (Box/Button thô) |
| Truyền i18n key vào placeholder/chuỗi | Hardcode chuỗi hiển thị |
| Icon badge qua `create_icon_badge` | Tự vẽ hình tròn kính mờ |
| Màu qua token (`#3b82f6`, alpha chuẩn) | Tự đặt màu mới ngoài bảng token |
| CSS màu sửa ở cả `dark.css` + `light.css` | Chỉ sửa 1 file màu |
