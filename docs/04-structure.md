# 04 — Cấu trúc dự án và quy tắc phát triển

## Phạm vi

Trang này xác định file nào thuộc lớp nào, nơi đặt code mới và cách tránh để installer phụ thuộc vào tên riêng của một branch.

## Cây thư mục của branch nguồn

```text
BabyDra/
├── Cargo.toml                 Cargo workspace
├── crates/                     Binary ứng dụng
│   ├── babydra-panel/
│   ├── babydra-desktop/
│   ├── babydra-switcher/
│   ├── babydra-keymap/
│   ├── babydra-workspace/
│   ├── babydra-explore/
│   ├── babydra-settings/
│   ├── babydra-launcher/
│   ├── babydra-lock/
│   ├── babydra-greeter/
│   ├── babydra-screenshot/
│   └── babydra-preview/
├── libs/
│   ├── babydra-core/
│   ├── babydra-ui-kit/
│   ├── babydra-island/
│   └── babydra-theme/
├── configs/                   Cấu hình labwc, terminal, editor, theme
├── themes/                    Theme package runtime
├── variants/                  Variant và variant.toml
├── workspace.toml             Metadata cài đặt của branch
├── tests/                     Integration tests nếu branch có
└── docs/                      Tài liệu nếu branch duy trì cùng source
```

## Cây thư mục của `main`

```text
BabyDra/
├── install/
│   ├── src/app/               Wizard state và actions
│   ├── src/models/            Model của installer
│   ├── src/system/            Discovery, git, manifest, sudo
│   ├── src/tasks/             Các task cài đặt
│   ├── src/ui/                Giao diện Ratatui
│   ├── Cargo.toml             Crate độc lập
│   └── run.sh
├── docs/                      Tài liệu tổng quát
└── README.md
```

`main` không chứa `workspace.toml` của project. File đó phải được commit trên branch chứa mã nguồn để installer lấy đúng metadata theo phiên bản.

## Trách nhiệm module

| Vùng | Trách nhiệm | Không đặt vào đây |
| :--- | :--- | :--- |
| `crates/*` | Lifecycle ứng dụng, event, UI riêng của binary | Service dùng chung hoặc danh sách package cài đặt |
| `libs/babydra-core` | Service và model thuần | Widget GTK hoặc layout app |
| `libs/babydra-ui-kit` | Component, style layout, icon, animation | Nghiệp vụ mạng hoặc package manager |
| `libs/babydra-island` | Island controller và feature | Logic installer |
| `libs/babydra-theme` | Resolve theme và CSS runtime | Trạng thái riêng của một app |
| `install/src/system` | Discovery, git, manifest, system helper | Tên binary/package cố định của project |
| `install/src/tasks` | Thực thi policy đã discovery | Logic scan chỉ phục vụ một app cụ thể |
| `configs`, `themes`, `variants` | Dữ liệu triển khai | Logic Rust |

## Quy tắc discovery và manifest

Khi thêm binary mới:

1. Thêm Cargo package hoặc `[[bin]]` trên branch nguồn.
2. Đảm bảo target tạo ra executable trong `target/release`.
3. Thêm entry vào `workspace.toml` nếu cần mô tả, đổi tên đích hoặc chọn `system` scope.
4. Không thêm tên binary vào `main/install/src`.

Cargo discovery là cơ chế phát hiện fallback. `workspace.toml` là nơi khai báo policy. Hai cơ chế này không nên bị trộn: discovery không đoán package AUR, còn manifest không cần lặp lại mọi thông tin Cargo nếu tên và scope mặc định đã đủ.

## Quy tắc tổ chức module

### UI

Module UI nên tách state, event và render khi có đủ độ phức tạp:

```text
widgets/<feature>/
├── mod.rs        State, constructor và public API
├── render.rs     Dựng widget từ state
├── handlers.rs   Callback và event handler nếu cần
└── service.rs    Tác vụ nền hoặc channel nếu cần
```

Không tách file chỉ để làm ngắn dòng code. Mục tiêu là giữ dependency và trách nhiệm có thể kiểm tra.

### Installer

Installer nên tuân theo luồng:

```text
UI state → InstallPlan → worker → discovery/manifest → tasks → InstallEvent → UI
```

UI không chạy lệnh hệ thống trực tiếp. Task nhận dữ liệu đã được discovery, thực hiện một hành động có log và trả số lượng thành công/lỗi.

## Quy tắc code

- Dùng `snake_case` cho module Rust và `kebab-case` cho crate/directory.
- Tách logic thuần khỏi GTK để test được.
- Dùng `babydra-ui-kit` thay vì tự dựng widget tương đương.
- Dùng theme token và i18n thay vì đặt màu hoặc chuỗi hiển thị rải rác.
- Không thêm branch name, binary name hoặc package name vào installer trừ khi đó là fallback tổng quát có lý do rõ ràng.
- Thay đổi schema phải có test parser và cập nhật [03-setup.md](03-setup.md).
- Commit theo Conventional Commits.
