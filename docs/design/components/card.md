# Component: Card

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Card** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/card/`

---

## 1. Tổng quan

Card là bề mặt chứa dùng cho mọi khối nội dung trong Settings và Explore. Gồm 4 nhóm hàm:

| Nhóm | File | Công dụng |
| :--- | :--- | :--- |
| Card cơ bản | `standard.rs` | Tạo card kính mờ, tiêu đề, phụ đề, dòng item |
| Switch Card | `switch_card.rs` | Card có sẵn một `CustomSwitch` (bật/tắt) |
| Danh sách cuộn | `scrollable.rs` | `ScrolledWindow` + `ListBox` kết hợp |
| Grid file | qua `mod.rs` | `create_grid_file_item` (re-export từ explore) |

---

## 2. API

### 2.1. Card cơ bản (`standard.rs`)

```rust
pub fn create_card(orientation: Orientation, spacing: i32) -> gtk4::Box
pub fn create_card_with_class(orientation: Orientation, spacing: i32, css_class: &str) -> gtk4::Box
pub fn create_title(text: &str) -> gtk4::Label        // class "settings-title", căn trái
pub fn create_subtitle(text: &str) -> gtk4::Label     // class "settings-subtitle", căn trái
pub fn create_item_row(title: &str, subtitle: &str, suffix_widget: Option<&impl IsA<gtk4::Widget>>) -> gtk4::Box
```

- `create_card` — Box gắn class `settings-card` (surface kính mờ chuẩn).
- `create_item_row` — dòng `settings-item-row`: cột text (title `settings-label` + subtitle `settings-desc`) bên trái, widget tùy chọn bên phải (switch, nút, badge...).

### 2.2. Switch Card (`switch_card.rs`)

```rust
pub fn create_switch_card(title: &str, subtitle: &str) -> (gtk4::Box, crate::components::switch::CustomSwitch)
```

Trả về cặp (card, switch) — card đã chứa sẵn title + subtitle + switch ở phía phải. Sau đó dùng `sw.connect_state_set(...)` để lắng nghe thay đổi.

### 2.3. Danh sách cuộn (`scrollable.rs`)

```rust
pub fn create_scrollable_list(css_class: &str) -> (gtk4::ScrolledWindow, gtk4::ListBox)
```

- Policy: cuộn dọc tự động (`Automatic`), ngang không cuộn (`Never`).
- `ListBox` chế độ `SelectionMode::None`, gắn class tùy chọn (vd: `settings-card-list`).

---

## 3. Style

### Dark theme — surface card chuẩn

```css
.settings-card,
.glass-panel {
    background-color: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.10);
    border-top: 1px solid rgba(255, 255, 255, 0.16);   /* bevel ánh sáng trên */
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.35);
    padding: 16px 20px;
}
```

### Shared — cấu trúc dòng trong card

```css
.settings-card { border-radius: 14px; padding: 12px; }

.settings-card row,
.settings-card-row {
    background: transparent;
    transition: background-color 200ms ease;
}
.settings-card row:first-child { border-radius: 10px 10px 0 0; }
.settings-card row:last-child  { border-radius: 0 0 10px 10px; }
.settings-card row:only-child  { border-radius: 10px; }
```

> [!NOTE]
> Card là **surface nổi cấp 1** (xem [surfaces.md](../surfaces.md)): nền bán trong suốt + border bevel + shadow — không thêm blur riêng ngoài hệ thống.

---

## 4. Ví dụ sử dụng

```rust
// Card dọc chứa tiêu đề và các dòng
let card = create_card(Orientation::Vertical, 12);
card.append(&create_title("Network"));
card.append(&create_item_row("Wi-Fi", "Connected to Home-5G", Some(&wifi_badge)));

// Card có switch bật/tắt
let (switch_card, sw) = create_switch_card("Bluetooth", "Toggle Bluetooth adapter");
sw.connect_state_set(|active| { /* ... */ });

// Danh sách cuộn
let (scroll, list_box) = create_scrollable_list("settings-card-list");
for item in items {
    list_box.append(&create_item_row(&item.title, &item.desc, None));
}
```

---

## 5. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Mọi khối nội dung Settings phải nằm trong `create_card` hoặc `create_card_with_class` |
| DO | Dòng item dùng `create_item_row` — không tự dựng `Box` tay |
| DO | Card chứa danh sách dài phải dùng `create_scrollable_list` |
| DO NOT | Không ghi đè `border-radius`, `box-shadow` của `.settings-card` trong CSS ứng dụng riêng |
| DO NOT | Không tạo card mới không dựa trên `settings-card` (phá vỡ tính nhất quán) |
