# Luồng hoạt động — `babydra-preview`

**Phạm vi:** Luồng mở ảnh từ argv và fallback FileDialog.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Luồng khởi động](#1-luồng-khởi-động)
- [2. Mở ảnh từ argv](#2-mở-ảnh-từ-argv)
- [3. Fallback FileDialog](#3-fallback-filedialog)
- [4. Viewer](#4-viewer)

---

## 1. Luồng khởi động

`crates/babydra-preview/src/main.rs`:

```text
main()
  → gtk4::Application::new("com.babydra.preview")

  connect_activate:
     init_theme()
     arg_path = args[1]?
        ├─ path tồn tại → widgets::build_ui(app, path); return
        └─ không có / sai   → fallback FileDialog
```

---

## 2. Mở ảnh từ argv

```text
babydra-preview /path/to/image.png
  → path tồn tại → build_ui(app, path) → viewer hiện ngay
```

(Ứng dụng preview được gọi từ file manager / .desktop entry với đường dẫn ảnh làm đối số.)

---

## 3. Fallback FileDialog

Khi không có argv hoặc path không hợp lệ:

```text
1. Tạo window rỗng 400x200 (title i18n "common.app_preview_title")
2. FileDialog + FileFilter (image/png, jpeg, webp)
3. file_dialog.open(Some(window), None, callback):
     Ok(file) có path → build_ui(app, path) + đóng window rỗng
     khác           → đóng window rỗng
4. window rỗng present()
```

---

## 4. Viewer

`widgets/viewer.rs`:

```text
build_ui(app, path)
  → viewer window
  → đọc EXIF (exif_reader.rs) → metadata nếu có
  → hiển thị ảnh (hardware-accelerated)
  → zoom / pan
```

> [!NOTE]
> `read_exif` nằm trong core (`services::exif`) — xem [flows/core.md](./core.md).
