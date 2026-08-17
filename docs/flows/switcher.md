# Luồng hoạt động — `babydra-switcher`

**Phạm vi:** Mô hình daemon-client, luồng socket, message pump, và vòng đời overlay Alt-Tab.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Hai chế độ hoạt động](#1-hai-chế-độ-hoạt-động)
- [2. Luồng daemon](#2-luồng-daemon)
- [3. Luồng socket listener & message pump](#3-luồng-socket-listener--message-pump)
- [4. Luồng one-shot (fallback)](#4-luồng-one-shot-fallback)
- [5. Dọn dẹp socket](#5-dọn-dẹp-socket)

---

## 1. Hai chế độ hoạt động

| Chế độ | Lệnh | Khi nào |
| :--- | :--- | :--- |
| **Daemon** | `babydra-switcher --daemon` | autostart của labwc — giữ overlay trong bộ nhớ |
| **One-shot** | `babydra-switcher` (không flag) | fallback / test — spawn, hiện, thoát |

```text
main()
  ├─ args.contains("--daemon") → run_daemon()
  └─ ngược lại → run_oneshot()
```

---

## 2. Luồng daemon

```text
run_daemon()
  1. Nếu socket /tmp/babydra-switcher.socket đã connect được
       → daemon khác đang chạy → báo lỗi + thoát (tránh 2 daemon)
  2. gtk4::Application::new("org.babydra.switcher")

  connect_activate:
    a. controller = render::build_switcher_ui(app)   ── overlay + show/hide/next functions
    b. Bọc show_fn / hide_fn / next_fn trong Rc
    c. spawn_socket_listener(SOCKET_PATH, tx)        ── thread lắng nghe socket
    d. setup_message_pump(rx, ...)                   ── poll channel trên main thread (8ms)

  application.run()
  sau khi thoát → remove_file(SOCKET_PATH)
```

---

## 3. Luồng socket listener & message pump

### 3.1. Socket listener (thread nền)

`crates/babydra-switcher/src/daemon.rs`:

```text
spawn_socket_listener(path, tx)
  loop:
    remove_file(path)                    ── xóa socket cũ (stale)
    UnixListener::bind(path)
    for stream in listener.incoming():
        đọc buffer 8 bytes
        msg == b"show" || b"next" → tx.send(ShowOrNext)
        msg == b"hide"             → tx.send(Hide)
    bind lỗi → sleep 500ms → thử lại
```

### 3.2. Message pump (main thread)

```text
setup_message_pump(rx, on_show_or_next, on_hide)
  timeout_add_local(8ms):
    try_lock rx → drain try_recv():
        ShowOrNext → nếu window đang visible → next_fn() (cycle)
                      ngược lại → show_fn() (hiện lần đầu)
        Hide       → hide_fn()
    Continue
```

Client gửi:

```text
babydra-switcher (one-shot, từ Alt+Tab keybind)
  └─ try_signal_daemon(b"show")      ── connect socket + write "show" + thoát ngay
```

> [!IMPORTANT]
> Nhờ daemon giữ overlay sẵn, nhấn Alt+Tab → client gửi tín hiệu → daemon hiện
> window trong **< 10ms** — không có cold start. Đây là ví dụ chuẩn của mô hình
> Daemon-Client (xem [architecture](../architecture/index.md) mục 4).

---

## 4. Luồng one-shot (fallback)

```text
run_oneshot()
  1. try_signal_daemon(b"show")  ── daemon đang chạy? gửi + thoát (không mở window mới)
  2. apps = get_running_apps()   ── rỗng → thoát
  3. gtk4::Application → build_switcher_ui → show_fn() ngay
  4. vẫn spawn socket listener + message pump (cho Alt+Tab lần sau)
  5. application.run()
```

---

## 5. Dọn dẹp socket

- Daemon và one-shot đều `remove_file(SOCKET_PATH)` sau khi `application.run()` kết thúc.
- Socket listener cũng tự xóa stale socket mỗi lần bind — phòng socket mồ côi từ lần chạy trước.

> [!NOTE]
> Nguồn dữ liệu window: `babydra_core::get_running_apps()` + `spawn_switcher_tracker()`
> (panel spawn) — chi tiết [flows/core.md](./core.md).
