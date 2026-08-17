# Component: Switch

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Switch (Toggle)** trong hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-utils/src/components/switch/mod.rs`

> [!NOTE]
> Hệ switch duy nhất là **`CustomSwitch`** (vẽ Cairo, màu lấy từ `ui/theme/colors.rs`).
> CSS `switch.baby-switch` cũ đã bị xóa (dead CSS, Phase 2 T2.2).

---

## 1. Tổng quan

BabyDra dùng **CustomSwitch** — một toggle switch vẽ bằng Cairo (`DrawingArea`), không dùng `gtk4::Switch` mặc định. Lý do: kiểm soát hoàn toàn màu sắc, kích thước và animation.

| Thành phần | Mô tả |
| :--- | :--- |
| `CustomSwitch` | Switch vẽ tay 46×24px, animation trượt 160ms (ease-out cubic) |
| `ToggleRow` | Hàng gồm label trạng thái "On/Off" + `CustomSwitch` — dùng đồng bộ cho Wi-Fi, Bluetooth... |

---

## 2. API

### 2.1. CustomSwitch

```rust
pub fn create_switch(initial_active: bool, on_changed: impl Fn(bool) + 'static) -> CustomSwitch
```

Các phương thức:

| Phương thức | Mô tả |
| :--- | :--- |
| `sw.is_active() -> bool` | Đọc trạng thái hiện tại |
| `sw.set_active(bool)` | Đặt trạng thái (tự chạy animation trượt nếu khác giá trị cũ) |
| `sw.connect_state_set(f: impl Fn(bool))` | Đăng ký callback khi trạng thái đổi |
| `sw.container` | Widget `DrawingArea` để append vào layout |

**Thông số kỹ thuật:**

- Kích thước: `content_width 46`, `content_height 24`.
- Animation trượt knob: **160ms**, easing **ease-out cubic** (`1 - (1-t)³`).
- Màu khi bật: nền lerp về `#3b82f6`, knob trắng có bóng đổ nhẹ.
- Màu khi tắt: dark `rgba(255,255,255,0.16)` / light `rgba(0,0,0,0.14)`, có viền mờ.
- Con trỏ: `pointer`.

### 2.2. ToggleRow

```rust
pub struct ToggleRow {
    pub container: gtk4::Box,
    pub switch: CustomSwitch,
    pub label: gtk4::Label,
}
pub fn new(initial_active: bool) -> Self
pub fn set_active(&self, active: bool)
```

- `label` tự đồng bộ text "On"/"Off" qua i18n (`settings.on` / `settings.off`).
- Class trạng thái: `toggle-status-on` (chữ sáng) / `toggle-status-off` (chữ mờ).

### 2.3. Style

```css
/* Dark */
.toggle-status-on  { color: rgba(255, 255, 255, 0.9); }
.toggle-status-off { color: rgba(255, 255, 255, 0.4); }
```

> [!NOTE]
> Ngoài ra còn có CSS cho switch GTK mặc định (`switch.baby-switch`) tại `styles/shared/shared/switch.css` — pill 40×20px, slider tròn 16px, `:checked` → nền `#3b82f6`. Ưu tiên dùng `CustomSwitch` cho các control tương tác.

---

## 3. Ví dụ sử dụng

```rust
// Switch độc lập
let sw = create_switch(false, |active| {
    // active == true -> bật
});

// ToggleRow có label On/Off
let row = ToggleRow::new(false);
row.set_active(true);
container.append(&row.container);
```

---

## 4. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `create_switch` / `ToggleRow` cho mọi control bật-tắt trong Settings |
| DO | Luôn gắn callback qua `connect_state_set` để cập nhật logic |
| DO NOT | Không dùng `gtk4::Switch` thô (kiểu dáng không đồng nhất với hệ thống) |
| DO NOT | Không tự đổi kích thước `DrawingArea` của switch (46×24 là chuẩn) |
