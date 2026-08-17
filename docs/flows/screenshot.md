# Luồng hoạt động — `babydra-screenshot`

**Phạm vi:** Luồng chụp toàn màn hình (`--full`) và chụp vùng (regional) + editor.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Hai chế độ chụp](#1-hai-chế-độ-chụp)
- [2. Luồng `--full`](#2-luồng---full)
- [3. Luồng regional capture](#3-luồng-regional-capture)
- [4. Editor](#4-editor)
- [5. Lưu & copy](#5-lưu--copy)

---

## 1. Hai chế độ chụp

| Chế độ | Khi nào | Luồng |
| :--- | :--- | :--- |
| `--full` | Chụp toàn màn hình ngay | `handle_fullscreen_capture()` rồi thoát |
| Mặc định | Chụp vùng (slurp/grim) | Capture → editor → lưu/copy |

```text
main()
  ├─ args.contains("--full") → handle_fullscreen_capture(); return;
  └─ capture_screen_to_temp() → editor
```

---

## 2. Luồng `--full`

```text
--full
  → babydra_core::handle_fullscreen_capture()
       ├─ grim chụp toàn màn hình
       ├─ copy vào clipboard (wl-clipboard) — trigger_copy
       └─ (hoặc lưu file)
  → return (không mở UI)
```

Chi tiết helpers: [flows/core.md](./core.md) mục screenshot helpers.

---

## 3. Luồng regional capture

```text
main (không --full)
  1. temp_path = capture_screen_to_temp()
       → chụp màn hình hiện tại ra file tạm (Some(path))
       → None → return (chụp lỗi)
  2. gtk4::Application::new("org.babydra.screenshot")

  connect_activate:
     init_theme()
     window = build_editor_ui(app, &temp_path)   ── editor với ảnh nền = screenshot
     window.present()

  application.run()

  3. Sau khi thoát: remove_file(temp_path)        ── dọn file tạm
```

---

## 4. Editor

`crates/babydra-screenshot/src/widgets/editor.rs` + `canvas.rs` + `color_popover.rs`:

```text
build_editor_ui(app, temp_path)
  → cửa sổ editor layer-shell Overlay (toàn màn hình)
  → canvas vẽ vùng chọn (slurp-style): kéo chuột chọn vùng → highlight
  → toolbar: save / copy / cancel / color picker
```

| Widget | Chức năng |
| :--- | :--- |
| `canvas.rs` | Vẽ vùng chọn + overlay mờ bên ngoài vùng |
| `color_popover.rs` | Chọn màu (annotation) |
| `editor.rs` | Ghép UI + xử lý hành động |
| `clipboard.rs` | Copy ảnh vào clipboard (wl-clipboard) |

---

## 5. Lưu & copy

Sau khi chọn vùng xong:

```text
save  → babydra_core::trigger_save() → lưu file theo get_screenshot_save_path()
copy  → babydra_core::trigger_copy() → wl-clipboard (wl-copy)
```

> [!NOTE]
> Cần `grim`, `slurp`, `wl-clipboard` cài sẵn và phiên Wayland — xem
> [setup](../setup/index.md) mục yêu cầu hệ thống.
