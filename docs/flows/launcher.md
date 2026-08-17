# Luồng hoạt động — `babydra-launcher`

**Phạm vi:** Luồng mở launcher, fuzzy search ứng dụng/file, chạy app.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Luồng khởi động](#1-luồng-khởi-động)
- [2. Dựng UI](#2-dựng-ui)
- [3. Luồng tìm kiếm](#3-luồng-tìm-kiếm)
- [4. Chạy ứng dụng](#4-chạy-ứng-dụng)

---

## 1. Luồng khởi động

`crates/babydra-launcher/src/main.rs`:

```text
main()
  → gtk4::Application::new("org.babydra.launcher")

  connect_activate:
     init_theme()
     launcher_window = Rc<RefCell<Option<ApplicationWindow>>>   ── (1 cửa sổ duy nhất)
     window = babydra_launcher::build_launcher_ui(app, launcher_window.clone())
     window.present()
     *launcher_window.borrow_mut() = Some(window)
```

Launcher có cả **lib target** (`lib.rs`) — panel import `build_launcher_ui` để mở launcher khi click logo (xem [flows/panel.md](./panel.md) mục 6).

```text
lib.rs
  pub use render::build_launcher_ui;
  pub use results::{repopulate_results, update_highlight};
```

---

## 2. Dựng UI

| Widget | Chức năng |
| :--- | :--- |
| `widgets/search/` | Ô tìm kiếm + kết quả ứng dụng (fuzzy) |
| `widgets/app_row/` | Một dòng ứng dụng trong kết quả |
| `widgets/file_search/` | Tìm kiếm file |
| `widgets/footer/` | Gợi ý phím tắt đáy |
| `results.rs` | `repopulate_results`, `update_highlight` |

---

## 3. Luồng tìm kiếm

```text
user gõ query
  → repopulate_results(query)
      ├─ babydra_core::find_desktop_apps()   ── lọc theo fuzzy match
      ├─ search_files(query)                 ── tìm file (nếu bật)
      └─ render kết quả + update_highlight (tô ký tự khớp)
```

---

## 4. Chạy ứng dụng

```text
user chọn kết quả (Enter/click)
  → chạy desktop app (exec) hoặc mở file
  → đóng launcher window
```

> [!NOTE]
> Dữ liệu app từ core (`find_desktop_apps`) — chi tiết [flows/core.md](./core.md).
