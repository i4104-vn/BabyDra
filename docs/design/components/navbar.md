# Component: Navbar (Sidebar Navigation)

Tài liệu quy định cách sử dụng, kiểu dáng, và mã nguồn chuẩn cho **Navigation Row** trong **Sidebar** của hệ thống BabyDra.

**Vị trí mã nguồn:** `libs/babydra-ui-kit/src/components/navbar/mod.rs`

---

## 1. Tổng quan

Navigation row là một dòng điều hướng trong sidebar gồm: **icon badge tròn** + **nhãn**. Dùng cho thanh điều hướng trái của Settings và Explore.

```
┌─────────────────────────┐
│  (icon)  Appearance     │  ← settings-sidebar-row
│  (icon)  Apps           │
└─────────────────────────┘
```

---

## 2. API

```rust
pub fn create_sidebar_row(label: &str, icon_name: &str) -> gtk4::Box
pub fn create_sidebar_row_with_badge(label: &str, icon_name: &str, badge_class: &str) -> gtk4::Box
```

| Hàm | Badge class | Công dụng |
| :--- | :--- | :--- |
| `create_sidebar_row` | `badge-slate` (mặc định) | Row tiêu chuẩn |
| `create_sidebar_row_with_badge` | tùy chọn | Row với badge màu khác |

**Cấu trúc:**

- Row: class `settings-sidebar-row`, spacing 12.
- Badge: class `sidebar-icon-badge` + `badge_class`, chứa icon 16px.
- Nhãn: class `sidebar-row-label`.

---

## 3. Style tham chiếu (shared)

```css
.sidebar {
    border-radius: 14px;
    margin: 8px 4px 8px 8px;
    min-width: 180px;
}

.sidebar-item {
    background: transparent;
    border: none;
    border-radius: 10px;
    padding: 8px 12px;
    font-size: 13px;
    margin: 1px 6px;
    transition: background-color 0.15s ease, color 0.15s ease;
}

.sidebar-section-label {
    font-size: 11px;
    font-weight: 600;
    padding: 10px 16px 4px 16px;
    letter-spacing: 0.5px;
}
```

---

## 4. Ví dụ sử dụng

```rust
let row = create_sidebar_row("Appearance", "preferences-desktop-theme");
sidebar.append(&row);

// Badge đặc biệt (vd: badge màu trạng thái)
let row2 = create_sidebar_row_with_badge("Updates", "system-software-update", "badge-green");
sidebar.append(&row2);
```

---

## 5. Quy tắc bắt buộc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dòng điều hướng sidebar dùng `create_sidebar_row` / `_with_badge` |
| DO | Icon badge phải dùng class `sidebar-icon-badge` + badge màu chuẩn |
| DO | Sidebar container dùng class `sidebar` (bo 14px, margin chuẩn) |
| DO NOT | Không tự dựng row tay với layout khác spacing 12 |
