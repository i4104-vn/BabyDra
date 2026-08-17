# UI Kit & API — Tổng hợp

**Phạm vi:** Tổng quan các thư viện có API dùng được, cách dùng `prelude`, liên kết tới API reference chi tiết.
**Phiên bản:** 1.1.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Kit là gì?](#1-kit-là-gì)
- [2. Bản đồ API theo thư viện](#2-bản-đồ-api-theo-thư-luviện)
- [3. Cấu trúc module của ui-kit](#3-cấu-trúc-module-của-ui-kit)
- [4. Bắt đầu nhanh](#4-bắt-đầu-nhanh)
- [5. Quy tắc](#5-quy-tắc)

---

## 1. Kit là gì?

**Kit** (feature kit) là crate **tái sử dụng được** đóng gói một nhóm widget/feature theo domain — nằm trong `libs/`. Khác với `libs/` logic thuần (`babydra-core`, `babydra-theme`) và `crates/` (ứng dụng chạy độc lập), kit chứa **giao diện GTK dùng chung**:

| Kit | Domain | Mô tả |
| :--- | :--- | :--- |
| [`babydra-ui-kit`](./ui-kit.md) | Giao diện | Widget GTK4 dùng chung + theme/icon/animation/battery/window helpers |
| `babydra-ui-kit::components::explore` | Explore | Dialog, context menu, drag & drop, file items, selection — nằm trong cùng crate |
| `babydra-island` | Dynamic Island | Manager + view stack mở rộng được (xem [guides/island](../guides/island.md)) |

---

## 2. Bản đồ API theo thư viện

| Tài liệu | Nội dung |
| :--- | :--- |
| [API `babydra-core`](./core.md) | Logic thuần: services, models, config, i18n — không GTK |
| [API `babydra-ui-kit`](./ui-kit.md) | Từng component, helper, signature, ví dụ |
| [API `components::explore`](./explore-kit.md) | Từng feature, helper, signature, ví dụ |
| [Guide island](../guides/island.md) | Sử dụng & mở rộng Dynamic Island |
| [Kiến trúc](../architecture/index.md) | Vai trò của libs/kits/crates trong luồng dữ liệu |
| [Cấu trúc dự án](../structure/index.md) | Vị trí thư mục, quy chuẩn |

---

## 3. Cấu trúc module của ui-kit

| Module | Nội dung |
| :--- | :--- |
| `components` | Widget builders: buttons, card, list_group, modal, placeholder, popovers, slider, switch, tooltips, wifi |
| `ui` | Theme, icon, animation, battery, window helpers |
| `components::explore` | Feature components cho file manager: context_menu, dialogs, drag, helpers, items, selection, widgets |
| `prelude` | Re-export API thông dụng của `components` + `ui` |
| `components::explore::prelude` | Re-export API feature của `components::explore` (tách riêng để tránh trùng tên như `create_list_row`) |

```rust
use babydra_ui_kit::prelude::*;          // widget + UI helpers chung
use babydra_ui_kit::components::explore::prelude::*; // feature API của Explore
```

---

## 4. Bắt đầu nhanh

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

## 5. Quy tắc

| Quy tắc | Chi tiết |
| :--- | :--- |
| DO | Dùng `prelude::*` cho API thông dụng — gọn và ổn định |
| DO | Dùng module sâu (`components::modal::…`) khi cần kiểm soát cụ thể |
| DO | Thêm API mới vào kit → cập nhật `prelude` + smoke test `tests/kits/prelude.rs` + tài liệu này |
| DO NOT | Nhét logic hệ thống vào kit — logic thuần ở `libs/babydra-core` |
| DO NOT | Import `gtk4` trực tiếp trong `babydra-core` — chỉ kit/crate UI mới dùng |

> [!IMPORTANT]
> **Bảo vệ API:** `tests/kits/prelude.rs` khai báo fn-pointer signature của các API chính — đổi/đổi tên API sẽ làm test fail ngay, giúp docs không bị lệch code.
