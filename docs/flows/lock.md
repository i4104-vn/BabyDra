# Luồng hoạt động — `babydra-lock`

**Phạm vi:** Luồng khởi động, parse CLI, dựng UI, xác thực PAM, map nhiều màn hình.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Luồng khởi động](#1-luồng-khởi-động)
- [2. Parse CLI `--image`](#2-parse-cli---image)
- [3. Dựng UI](#3-dựng-ui)
- [4. Xác thực](#4-xác-thực)

---

## 1. Luồng khởi động

`crates/babydra-lock/src/main.rs`:

```text
main()
  1. Parse args → custom_image (--image/-i hoặc positional path)
  2. gtk4::Application::new("org.babydra.lock")

  connect_activate:
     init_theme()
     render::build_lock_ui(app, custom_image.as_deref())

  application.run_with_args::<&str>(&[])
```

---

## 2. Parse CLI `--image`

```text
duyệt args:
  "--image" | "-i" + <path> → custom_image = path
  positional (không bắt đầu bằng '-') → custom_image = path
  khác → bỏ qua
```

Nếu không có ảnh → dùng wallpaper mặc định (từ config/wallpaper).

---

## 3. Dựng UI

`render.rs`:

```text
build_lock_ui(app, custom_image)
  → cửa sổ layer-shell Layer::Overlay (bao phủ toàn màn hình)
  → map window tới MỌI monitor
  → nền = custom_image (nếu có) hoặc wallpaper hệ thống
  → widget khóa: avatar, đồng hồ, ô nhập mật khẩu, nút shutdown/reboot/suspend
```

---

## 4. Xác thực

```text
user nhập mật khẩu
  → babydra_core::verify_password(user, pwd)   ── PAM
       đúng  → mở khóa (đóng window)
       sai   → báo lỗi, reset entry
```

> [!NOTE]
> `verify_password` nằm trong core (`services::system::auth`) — xem
> [flows/core.md](./core.md) và [apis/core](../apis/core.md).
