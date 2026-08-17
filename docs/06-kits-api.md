# Chương 06: UI Kit & API — Tổng hợp

**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17
**Phạm vi:** Tổng quan UI kit của BabyDra (`libs/babydra-ui-kit`), cách dùng `prelude`, liên kết tới API reference chi tiết.

---

## 1. Kit là gì?

**Kit** (feature kit) là crate **tái sử dụng được** đóng gói một nhóm widget/feature
theo domain — nằm trong `libs/`. Khác với `libs/` logic thuần (`babydra-core`,
`babydra-theme`) và `crates/` (ứng dụng chạy độc lập), kit chứa **giao diện GTK
dùng chung** và có thể được nhiều app dùng chung:

| Kit | Domain | Mô tả |
| :--- | :--- | :--- |
| [`babydra-ui-kit`](./kit-apis/ui-kit.md) | Giao diện | Widget GTK4 dùng chung + theme/icon/animation/battery/window helpers |
| `babydra-ui-kit::components::explore` | Explore | Dialog, context menu, drag & drop, file items, selection — nằm trong cùng crate |

UI kit expose 2 module **`prelude`** — import 1 dòng là dùng được toàn bộ API thông dụng:

```rust
use babydra_ui_kit::prelude::*;          // widget + UI helpers chung
use babydra_ui_kit::components::explore::prelude::*; // feature API của Explore
```

---

## 2. Cấu trúc module

| Module | Nội dung |
| :--- | :--- |
| `components` | Widget builders: buttons, card, list_group, modal, placeholder, popovers, slider, switch, tooltips, wifi |
| `ui` | Theme, icon, animation, battery, window helpers |
| `components::explore` | Feature components cho file manager: context_menu, dialogs, drag, helpers, items, selection, widgets |
| `prelude` | Re-export API thông dụng của `components` + `ui` |
| `components::explore::prelude` | Re-export API feature của `components::explore` (tách riêng để tránh trùng tên như `create_list_row`) |

---

## 3. Các API chính (tóm tắt)

### 3.1. Widget & UI helpers (`prelude`)

| Nhóm | API tiêu biểu |
| :--- | :--- |
| Widget | `create_button`, `create_fab`, `create_card`, `create_switch_card`, `create_scrollable_list`, `create_list_row`, `CustomSwitch`, `CustomSlider`, `create_placeholder_row` |
| Modal | `PasswordDialog`, `WifiConfigDialog`, `WifiInfoDialog`, `WifiPasswordDialog`, `VpnConfigDialog`, `VpnLogDialog` |
| Popover | `create_popover`, `attach_hover_popover`, `HoverPopoverRow` |
| Theme | `init_theme`, `set_dark_mode`, `is_dark_mode` |
| Icon | `get_icon`, `get_icon_colored`, `get_logo_png`, `get_system_or_file_icon` |
| Animation | `slide_in/out`, `genie_in/out`, `island_*`, `ease_*` |
| Battery | `get_battery_color_hex`, `draw_cairo_battery` |
| Window | `init_layer_window`, `setup_click_outside_dismiss` |

### 3.2. Explore feature (`components::explore::prelude`)

| Nhóm | API tiêu biểu |
| :--- | :--- |
| Context menu | `show_for_file`, `show_for_empty`, `CLIPBOARD`, `execute_paste`, `execute_undo` |
| Dialog | `show_new_folder_dialog`, `show_rename_dialog`, `show_conflict_dialog`, `show_properties_dialog`, `perform_decompress_async`, `show_alert_dialog` |
| Drag & drop | `create_drag_source`, `create_dir_drop_target`, `create_background_drop_target` |
| Items | `create_grid_file_item`, `create_list_row` |
| Selection | `wire_rubberband_grid`, `wire_rubberband_listbox` |
| Helpers | `format_size`, `format_date`, `is_archive_file`, `is_in_trash`, `restore_from_trash`, `sanitize_path` |

---

## 4. Bắt đầu nhanh (ví dụ kết hợp 2 prelude)

```rust
use babydra_ui_kit::prelude::*;
use babydra_ui_kit::components::explore::prelude::*;

fn build_file_window() {
    init_theme();

    // Thanh công cụ từ ui-kit
    let fab = create_fab("plus");

    // Click → dialog tạo thư mục từ explore feature
    let current = current_path.clone();
    fab.connect_clicked(move |_| {
        show_new_folder_dialog(current.clone(), nav_cb.clone(), None);
    });

    // Context menu trên file
    show_for_file(&widget, x, y, targets, current, nav_cb, &window);
}
```

---

## 5. Danh sách API chi tiết

| Tài liệu | Nội dung |
| :--- | :--- |
| [API `babydra-ui-kit`](./kit-apis/ui-kit.md) | Từng component, helper, signature, ví dụ |
| [API `babydra-ui-kit::components::explore`](./kit-apis/explore-kit.md) | Từng feature, helper, signature, ví dụ |
| [Kiến trúc](./02-architecture.md) | Vai trò của libs/kits/crates trong luồng dữ liệu |
| [Cấu trúc dự án](./03-project-structure.md) | Vị trí thư mục, quy chuẩn |
| [Design components](./design/README.md) | Thiết kế giao diện của từng component |

---

## 6. Quy tắc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `prelude::*` cho API thông dụng — gọn và ổn định |
| DO | Dùng module sâu (`components::modal::…`) khi cần kiểm soát cụ thể |
| DO | Thêm API mới vào kit → cập nhật `prelude` + smoke test `tests/kits/prelude.rs` + tài liệu này |
| DO NOT | Nhét logic hệ thống vào kit — logic thuần ở `libs/babydra-core` |
| DO NOT | Import `gtk4` trực tiếp trong `babydra-core` — chỉ kit/crate UI mới dùng |

> [!IMPORTANT]
> **Bảo vệ API:** `tests/kits/prelude.rs` khai báo fn-pointer signature của các
> API chính — đổi/đổi tên API sẽ làm test fail ngay, giúp docs không bị lệch code.
