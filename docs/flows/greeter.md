# Luồng hoạt động — `babydra-greeter`

**Phạm vi:** Luồng khởi động trong greetd/cage, dựng UI, xác thực PAM qua greetd protocol.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Bối cảnh chạy](#1-bối-cảnh-chạy)
- [2. Luồng khởi động](#2-luồng-khởi-động)
- [3. Dựng UI & handlers](#3-dựng-ui--handlers)
- [4. Xác thực qua greetd](#4-xác-thực-qua-greetd)

---

## 1. Bối cảnh chạy

`babydra-greeter` chạy **trong cage compositor**, được greetd launch:

```text
greetd → cage → babydra-greeter
  ├─ env GREETD_SOCK   ── socket giao tiếp greetd
  ├─ env WAYLAND_DISPLAY
  └─ env XDG_CONFIG_HOME
```

---

## 2. Luồng khởi động

`crates/babydra-greeter/src/main.rs`:

```text
main()
  1. init_logger("babydra-greeter", "displaymanager.log")   ── log vào ~/.cache/babydra
  2. Đọc env: GREETD_SOCK / WAYLAND_DISPLAY / XDG_CONFIG_HOME → info log
  3. gtk4::Application::builder().application_id("com.babydra.greeter")

  connect_activate:
     greeter = render::build_greeter_ui(app)   ── UI đăng nhập
     handlers::setup_handlers(&greeter)        ── gắn xử lý sự kiện
     greeter.window.present()

  application.run()
```

---

## 3. Dựng UI & handlers

| Widget | Chức năng |
| :--- | :--- |
| `widgets/login.rs` | Form đăng nhập: username/password + nút đăng nhập |
| `widgets/splash.rs` | Splash screen trong lúc chờ |
| `widgets/top_bar.rs` | Thanh trên cùng (giờ, trạng thái...) |
| `theme.rs` | Theme cho greeter |

`handlers.rs`:

```text
setup_handlers(greeter)
  → connect submit → auth::authenticate(user, pwd)
  → connect power actions (shutdown/reboot) nếu có
```

---

## 4. Xác thực qua greetd

```text
user submit
  → auth.rs: giao tiếp greetd protocol qua GREETD_SOCK (Unix socket)
  → greetd xác thực (PAM) và khởi chạy session (thường là labwc)
  → thành công → greeter thoát, session bắt đầu
```

> [!NOTE]
> Greeter dùng **greetd protocol** (qua socket), không gọi `verify_password`
> trực tiếp — khác với lock. Chi tiết setup greetd: [setup](../setup/index.md)
> và [structure](../structure/index.md) mục installer step 6.
