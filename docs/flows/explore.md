# Luồng hoạt động — `babydra-explore`

**Phạm vi:** Luồng khởi động, SessionState, cây widget window, luồng điều hướng thư mục, gestures.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Luồng khởi động](#1-luồng-khởi-động)
- [2. SessionState](#2-sessionstate)
- [3. Cây widget window](#3-cây-widget-window)
- [4. Luồng điều hướng thư mục](#4-luồng-điều-hướng-thư-mục)
- [5. Gestures & DnD](#5-gestures--dnd)
- [6. Thao tác file async](#6-thao-tác-file-async)

---

## 1. Luồng khởi động

`crates/babydra-explore/src/main.rs`:

```text
main()
  1. tokio::runtime::Runtime::new() + enter    ── I/O nặng chạy async, không block UI
  2. gtk4::Application::builder("org.babydra.explore", NON_UNIQUE)

  connect_activate:
     target_dir = components::explore::parse_target_dir()   ── thư mục mở đầu (hoặc home)
     session = Rc<RefCell<SessionState::new(target_dir)>>
     window = widgets::window::create_explore_window(app, session)
     window.present()

  app.run() → exit(code)
```

---

## 2. SessionState

`widgets/state.rs` — nguồn dữ liệu duy nhất cho phiên làm việc:

```text
SessionState {
    tabs,            ── danh sách tab
    current_tab,
    current_path,    ── thư mục đang xem
    active_pane,     ── pane đang active (split/preview)
    show_hidden,     ── hiện file ẩn
    sort (column, order),
    selection,       ── file được chọn
    ...
}
```

Mọi widget đọc/ghi qua `Rc<RefCell<SessionState>>` — luồng một chiều (xem [architecture](../architecture/index.md) mục 3).

---

## 3. Cây widget window

`widgets/window/`:

```text
create_explore_window(app, session)
  → window/mod.rs
  ├── layout/   split.rs (split pane), preview.rs (preview pane), mod.rs
  ├── handlers/ events.rs (sự kiện toàn cửa sổ), navigation.rs (điều hướng), mod.rs
  ├── widgets/  tabs.rs (thanh tab)
  └── render.rs
```

Các vùng chính:

| Vùng | Module | Chức năng |
| :--- | :--- | :--- |
| Header bar | `widgets/header_bar/` | Thanh địa chỉ + nút điều hướng (back/forward/up/home) |
| Content view | `widgets/content_view/` | Grid hoặc List hiển thị file/folder |
| Sidebar | `widgets/sidebar/` | Cây thư mục + bookmarks |
| Preview panel | `widgets/preview_panel/` | Xem trước nhanh file (actions, create) |
| Info panel | `widgets/info_panel/` | Thông tin file/folder |
| Status bar | `widgets/status_bar/` | Thanh trạng thái đáy |
| Tab bar | `widgets/tab_bar/` | Tab phiên làm việc |
| Settings dialog | `widgets/settings_dialog/` | context menu, general, keybinds |

---

## 4. Luồng điều hướng thư mục

```text
user click thư mục / gõ path / back-forward
  → window/handlers/navigation.rs
  → cập nhật SessionState.current_path
  → content_view/render.rs: load_directory(path, show_hidden)  (core, async)
  → renderer: grid_renderer hoặc list_renderer dựng items
  → gộp theo grouping (models::explore::get_group_name) nếu bật
```

Content view chia:

```text
content_view/
├── rendering/   renderer.rs, grid_renderer.rs, list_renderer.rs
├── gestures/    background.rs, clipboard.rs, flowbox.rs, listbox.rs
├── items/       grid_item.rs
├── actions.rs
└── render.rs / mod.rs
```

---

## 5. Gestures & DnD

| Gesture | Module | Chức năng |
| :--- | :--- | :--- |
| Rubberband chọn nhiều | `components::explore::selection` | Kéo chọn vùng trên grid/list |
| Clipboard (cut/copy/paste) | `content_view/gestures/clipboard.rs` | Thao tác nội bộ + hệ thống |
| Background click | `content_view/gestures/background.rs` | Click vùng trống → context menu |
| Drag & drop | `components::explore::drag` | Kéo file vào folder |

---

## 6. Thao tác file async

```text
copy / move / delete / rename / send_to_trash
  → babydra_core::services::explore (copy_path, move_path, ...)  ── async qua tokio
  → kết quả → cập nhật SessionState → render lại
```

Context menu + dialogs dùng `babydra_ui_kit::components::explore::prelude::*` (xem [flows/ui-kit.md](./ui-kit.md) và [apis/explore-kit](../apis/explore-kit.md)).

> [!NOTE]
> Khác với các app khác, explore dùng **tokio** cho I/O nặng (đọc thư mục lớn,
> tính kích thước) — State vẫn là `Rc<RefCell<SessionState>>`, luồng vẫn một chiều
> (xem [architecture](../architecture/index.md) mục 8 FAQ).
