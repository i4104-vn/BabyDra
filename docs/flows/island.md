# Luồng hoạt động — `babydra-island`

**Phạm vi:** Tóm tắt luồng vận hành của Dynamic Island và con trỏ tới tài liệu chi tiết.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

> [!IMPORTANT]
> Tài liệu **chi tiết đầy đủ** (thành phần runtime, controller loop, arbitration,
> transition, luồng media player/notification từng dòng code) nằm ở
> [guides/island-internals](../guides/island-internals.md). Trang này chỉ là tóm tắt nhanh.

---

## Mục lục

- [1. Vị trí trong hệ thống](#1-vị-trí-trong-hệ-thống)
- [2. Controller loop (tóm tắt)](#2-controller-loop-tóm-tắt)
- [3. Luồng media player (tóm tắt)](#3-luồng-media-player-tóm-tắt)
- [4. Luồng notification (tóm tắt)](#4-luồng-notification-tóm-tắt)
- [5. Luồng override (tóm tắt)](#5-luồng-override-tóm-tắt)

---

## 1. Vị trí trong hệ thống

```text
panel (rebuild_panel_window)
  └─ create_system_island() → build_default_island()
       ├─ register NotificationFeature (priority 90)
       ├─ register MediaPlayerFeature (priority 50)
       ├─ idle logo
       └─ build → controller loop (150ms) + default_island() toàn process
```

Các feature khác (volume, clipboard...) lấy manager qua `babydra_island::default_island()` để đăng ký thêm — xem [guides/island](../guides/island.md).

---

## 2. Controller loop (tóm tắt)

```text
mỗi 150ms (island_tick):
  1. Timer     → hết show_for → bỏ request; hết override_show_for → nhả override
  2. Feature tick → mọi feature gọi handle.show()/hide() theo trạng thái
  3. Arbitration → override > priority > gần đây nhất > thứ tự đăng ký
  4. Transition  → on_hide(view cũ) → animate expand/collapse → on_show(view mới)
```

---

## 3. Luồng media player (tóm tắt)

```text
thread poll playerctl (1s)
  → raw metadata line → tokio channel
  → main-thread receiver → cache (Rc<RefCell>)
  → tick: đọc cache → parse → player đang chạy? handle.show() : handle.hide()
  → đang hiển thị? update_player_view: label, progress, artwork
  → artwork: thread tải (file:// hoặc curl http) → art receiver → gán notch (16px) + popover (240px)
  → click capsule → popover.toggle()
```

---

## 4. Luồng notification (tóm tắt)

```text
app gửi notification → babydra_core::send_notification
  → D-Bus daemon (island host) → channel
  → main thread → SHARED_NOTIFICATION (ActiveNotification + timestamp)
  → tick: đọc → render text/icon + đo chiều cao → handle.show()
  → hover → kéo dài vòng đời
  → hết 5s → xóa + handle.hide()
  → click → focus app gửi
```

---

## 5. Luồng override (tóm tắt)

```text
volume overlay:
  handle.set_content(widget)
  handle.override_show_for(1500ms)
    → override_active = true (thắng tuyệt đối, bỏ qua priority)
    → hết 1500ms: nhả override + rút request
    → arbitration quay lại → media player (vẫn đang request) tự khôi phục
```

> [!NOTE]
> Sử dụng API đầy đủ: [guides/island](../guides/island.md) • Cấu trúc feature:
> [guides/island-features](../guides/island-features.md) • Nội bộ chi tiết:
> [guides/island-internals](../guides/island-internals.md).
