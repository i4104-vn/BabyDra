# Luồng hoạt động — `babydra-installer` (TUI)

**Phạm vi:** Luồng khởi động TUI, event loop, 8 bước wizard, worker cài đặt nền.
**Phiên bản:** 1.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## Mục lục

- [1. Luồng khởi động](#1-luồng-khởi-động)
- [2. Event loop 50ms](#2-event-loop-50ms)
- [3. 8 bước wizard](#3-8-bước-wizard)
- [4. Worker cài đặt](#4-worker-cài-đặt)
- [5. Library target cho test](#5-library-target-cho-test)

---

## 1. Luồng khởi động

`install/src/main.rs`:

```text
main()
  1. setup_panic_hook()          ── thoát raw mode + alternate screen khi panic
  2. App::new()
  3. Parse args:
       --help / -h        → in usage, thoát
       --version / -v     → in version, thoát
       [SOURCE_BIN_DIR]   → app.source_binary_dir = path + rescan_binaries()
  4. enable_raw_mode() + EnterAlternateScreen
  5. Terminal::new(CrosstermBackend)
  6. run_app(&mut terminal, &mut app)
  7. disable_raw_mode() + LeaveAlternateScreen + show_cursor
```

---

## 2. Event loop 50ms

```text
run_app(terminal, app)
  loop:
    terminal.draw(|f| ui::draw(f, app))     ── vẽ toàn bộ màn hình
    nếu event::poll(50ms):
        Event::Key (KeyEventKind::Press) → app.handle_key(key)
    app.on_tick()                           ── cập nhật trạng thái (logs, progress)
    nếu app.should_quit → break
```

---

## 3. 8 bước wizard

| Bước | Module `ui/steps/` | Chức năng |
| :--- | :--- | :--- |
| 1 | `welcome.rs` | Chọn kênh (Release/Develop/LocalSource) + preset profile |
| 2 | `packages.rs` | Pacman deps, AUR, kernel/i2c permissions |
| 3 | `binaries.rs` | Chọn 9 binary component |
| 4 | `varlib.rs` | Stage binaries/wallpapers/logos vào /var/lib/babydra |
| 5 | `configs.rs` | Configs, themes, icons, .desktop entries |
| 6 | `display_manager.rs` | Cấu hình greetd + mask gettys |
| 7 | `progress.rs` | Execute + log realtime + progress gauge |
| 8 | `summary.rs` | Kết quả và thoát |

State wizard nằm trong `app/` (`App` struct) — `models/step.rs` (`WizardStep`), `models/options.rs` (kênh, preset).

---

## 4. Worker cài đặt

Bước 7 bấm `i`/`Enter` → xác nhận → `tasks/`:

```text
spawn_installation_worker(plan, tx)
  → thread worker chạy InstallPlan từng task
  → gửi InstallEvent (log line / tiến trình) qua channel
  → main thread (on_tick) đọc channel → cập nhật log + progress gauge

tasks/
├── packages.rs        ── pacman deps / AUR (yay) / kernel perms
├── binaries.rs        ── copy binary vào ~/.local/bin hoặc /usr/bin
├── varlib.rs          ── stage vào /var/lib/babydra
├── configs.rs         ── sync labwc, .desktop, dotfiles, themes, gsettings
└── display_manager.rs ── greetd config, mask gettys
```

---

## 5. Library target cho test

`install/src/lib.rs` — re-export `models`, `system`, `tasks` để integration test trong `tests/installer/` chạy pipeline **không cần mở TUI**:

```rust
pub use tasks::{spawn_installation_worker, InstallEvent, InstallPlan};
pub use system::{find_workspace_root, default_binary_source_dir, ...};
```

> [!NOTE]
> `install.sh` (nhánh main) build binary installer rồi `exec` TUI. Chi tiết 8 bước
> + phím tắt: [structure](../structure/index.md) mục 4; cài đặt: [setup](../setup/index.md) mục 4.
