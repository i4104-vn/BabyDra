# 04 — Cấu trúc dự án & quy chuẩn viết mã

**Phạm vi:** thư mục nào nằm ở đâu, vì sao; quy tắc viết code mới.
**Phiên bản:** 2.0.0
**Cập nhật lần cuối:** 2026-08-17

---

## 1. Cây thư mục (nhánh `release`)

```text
BabyDra/
├── Cargo.toml                ← workspace: libs/ + crates/ + install/ + tests/
├── crates/                   ← Ứng dụng (mỗi crate 1 binary)
│   ├── babydra-panel/        ← Island, dock, status bar (daemon)
│   ├── babydra-switcher/     ← Alt-Tab switcher (daemon)
│   ├── babydra-screenshot/   ← Chụp màn hình
│   ├── babydra-lock/         ← Màn hình khóa
│   ├── babydra-preview/      ← Xem ảnh
│   ├── babydra-settings/     ← Control center
│   ├── babydra-explore/      ← File explorer
│   ├── babydra-greeter/      ← Login greeter
│   └── babydra-launcher/     ← App grid + search
├── libs/                     ← Thư viện dùng chung (không chạy độc lập)
│   ├── babydra-core/         ← Services, config, i18n, models (không GTK)
│   ├── babydra-ui-kit/       ← Widget, styles/ (CSS shared), theme init, icon
│   ├── babydra-island/       ← Engine Dynamic Island + features/
│   └── babydra-theme/        ← Đọc/merge theme packages
├── install/                  ← Bộ cài đặt TUI (models/, system/, tasks/, ui/)
├── configs/                  ← Cấu hình mẫu: labwc/, kitty/, nvim/, fastfetch/, themes/
├── themes/                   ← Theme packages: <theme-id>/{tokens.json,fonts.json,css/}
├── variants/                 ← <variant>/{variant.toml}
├── scripts/                  ← install.sh, start.sh, update.sh, check.sh
├── tests/                    ← Integration suite (crate babydra-tests)
└── docs/                     ← Tài liệu này
```

---

## 2. Quy chuẩn đặt tên thư mục

| Quy tắc | Ví dụ |
| :--- | :--- |
| Thư mục crate/lib dùng `kebab-case` | `babydra-panel` |
| Module con trong source dùng `snake_case` | `widgets/status_bar/` |
| Thư mục feature dùng tên ngắn, rõ chức năng | `features/media_player/` |
| Mỗi crate 1 mục đích — không nhồi nhiều binary | — |

---

## 3. Triết lý phân tách file

### 3.1. Widget: `mod.rs` + `render.rs`

```text
widgets/status_bar/
├── mod.rs      ← struct StatusBar + state + handlers (logic)
└── render.rs   ← fn render(&self) -> gtk4::Box (chỉ dựng UI)
```

### 3.2. Island feature: 1 folder chuẩn

```text
features/<tên-feature>/
├── mod.rs       ← struct + constructor + impl IslandFeature
├── view.rs      ← state hiển thị, model dữ liệu
├── render.rs    ← dựng widget từ view
└── service.rs   ← (tùy chọn) luồng nền, channel, poll
```

Cấu trúc chuẩn feature + cách đăng ký: [07-dynamic-island.md](./07-dynamic-island.md).

### 3.3. Quy tắc chung

- File dài hơn ~300 dòng → tách module con có trách nhiệm rõ ràng.
- Logic thuần (không GTK) tách khỏi UI để test được.
- Helper dùng chung đặt ở nơi dùng chung, không copy-paste.

---

## 4. Trách nhiệm từng vùng

| Vùng | Trách nhiệm | Không làm |
| :--- | :--- | :--- |
| `crates/*` | Binary GTK + daemon; gọi `init_theme()` khi khởi động | Không chứa logic dùng lại |
| `libs/babydra-core` | Service, config, i18n, models — thuần logic | Không import GTK |
| `libs/babydra-ui-kit` | Widget dùng chung, CSS shared, icon, animation | Không chứa nghiệp vụ app |
| `libs/babydra-island` | Engine + features mặc định của island | Không phụ thuộc app cụ thể |
| `libs/babydra-theme` | Resolve/merge theme packages | Không GTK, không đọc config app |
| `install/` | Wizard cài đặt: `models/`, `system/`, `tasks/`, `ui/` | Không phụ thuộc GTK |
| `tests/` | Integration tests theo vùng (`common/`, `theme/`, `installer/`…) | Không là code production |
| `configs/` | Cấu hình seed cho hệ thống | Không chứa mã nguồn |
| `themes/` + `variants/` | Theme packages & variants (dữ liệu, không code) | — |

---

## 5. Quy tắc khi viết mã mới

| Quy tắc | Chi tiết |
| :--- | :--- |
| **Đi qua tokens/config/i18n** | Không hardcode màu/font/chuỗi. Màu → theme tokens; chuỗi → `babydra_core::i18n::t` |
| **Tách logic/UI** | State thuần + `render.rs`; logic test được không cần GTK |
| **Một nguồn theme** | Luôn gọi `init_theme()`; không tự nạp CSS/đặt màu riêng |
| **Component tái sử dụng** | Dùng widget trong `ui-kit` (`create_*`); không tự dựng tay |
| **CSS đúng tầng** | Cấu trúc → `styles/shared/`; màu → `themes/<id>/css/dark.css` + `light.css` |
| **Test kèm** | Đổi logic `core`/`ui-kit`/`theme` → thêm test trong `tests/` |
| **Workspace manifest** | Không sửa `Cargo.toml` gốc trừ khi thêm/xóa crate |
| **Conventional commits** | `type(scope): description` (vd `refactor(install): …`) |

Xem thêm quy trình đóng góp: [CONTRIBUTING.md](../CONTRIBUTING.md).
