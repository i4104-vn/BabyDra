# 01 — Tổng quan dự án

## Phạm vi

Trang này giải thích BabyDra gồm những gì, branch nào giữ dữ liệu nào và bộ cài đặt lấy thông tin từ đâu.

## BabyDra là gì

BabyDra là một desktop shell cho Linux. Hệ thống được xây dựng từ nhiều binary Rust chạy trên Wayland, compositor labwc và GTK4. Các binary được chia thành daemon chạy liên tục, ứng dụng mở theo nhu cầu và thư viện dùng chung.

Dự án có hai đặc điểm quan trọng:

1. Mã nguồn ứng dụng và metadata triển khai sống trên các branch nguồn như `release`, `develop` hoặc branch tính năng.
2. Bộ cài đặt sống trên `main` và không biết tên cụ thể của từng phiên bản. Nó đọc Cargo workspace, executable đã build và `workspace.toml` từ branch được chọn.

## Thành phần

### Binary

| Thành phần | Loại | Trách nhiệm |
| :--- | :--- | :--- |
| `babydra-panel` | Daemon | Panel, dock, system tray, notification và Dynamic Island. |
| `babydra-desktop` | Daemon | Wallpaper, desktop icon, file watcher và context menu. |
| `babydra-switcher` | Daemon | Overlay chuyển cửa sổ và giao tiếp với compositor. |
| `babydra-keymap` | Daemon | Đọc sự kiện bàn phím từ evdev và phát lệnh phím tắt. |
| `babydra-workspace` | Daemon/client | Chuyển đổi và hiển thị workspace Wayland. |
| `babydra-explore` | Ứng dụng | Quản lý file GTK4, tab, preview và FileManager D-Bus. |
| `babydra-settings` | Ứng dụng | Cấu hình mạng, VPN, âm thanh, màn hình, theme và phím tắt. |
| `babydra-launcher` | Ứng dụng | Đọc desktop entry, tìm kiếm và khởi chạy ứng dụng. |
| `babydra-lock` | Ứng dụng | Overlay khóa màn hình và xác thực PAM. |
| `babydra-greeter` | Ứng dụng hệ thống | Giao diện đăng nhập chạy dưới greetd/cage. |
| `babydra-screenshot` | Ứng dụng | Chụp toàn màn hình, cửa sổ, vùng chọn và OCR. |
| `babydra-preview` | Ứng dụng | Xem nhanh hình ảnh và phương tiện. |

Danh sách trên mô tả các binary hiện có của branch phát hành. Installer không sử dụng danh sách này làm dữ liệu điều khiển. Một branch có thể thêm, bớt hoặc đổi tên binary mà không cần sửa `main`.

### Thư viện

| Thư viện | Trách nhiệm |
| :--- | :--- |
| `babydra-core` | Service hệ thống, D-Bus, cấu hình, i18n và model không phụ thuộc GTK. |
| `babydra-ui-kit` | Component GTK4, icon, animation và CSS dùng chung. |
| `babydra-island` | Controller, view, feature và arbitration của Dynamic Island. |
| `babydra-theme` | Resolve theme, kế thừa package và sinh CSS runtime. |

## Mô hình branch

```text
main
├── install/                 Bộ cài đặt độc lập
└── docs/                    Tài liệu và hướng dẫn triển khai

release / develop / feature
├── Cargo.toml               Cargo workspace
├── crates/                  Binary ứng dụng
├── libs/                    Thư viện dùng chung
├── assets/                  Tài nguyên triển khai
│   ├── configs/             Cấu hình hệ thống và dotfiles
│   ├── desktop/             Desktop entries và MIME
│   └── themes/              Theme package runtime
├── updater/                 Crate cập nhật hệ thống
└── workspace.toml           Metadata dành cho installer
```

`main` có thể được clone và chạy installer dù chưa checkout branch nguồn. Khi người dùng chọn branch, installer tạo worktree tại `branches/<branch>` trong repository, build tại đó và đọc tài nguyên từ worktree.

## Cấu trúc `workspace.toml`

File phải nằm ở root của branch nguồn. Các nhóm dữ liệu hiện được hỗ trợ:

| Nhóm | Mục đích |
| :--- | :--- |
| `[[binaries]]` | Khai báo tên binary cài đặt, tên file nguồn tùy chọn, mô tả và scope. |
| `[packages]` | Danh sách package `pacman` và `aur` của branch. |
| `[installer]` | Bật các bước cài đặt tùy chọn mà branch thực sự yêu cầu. |
| `[gsettings]` | Các cặp schema/key và giá trị cần áp dụng sau khi cài. |
| `[mime]` | Khai báo các MIME type associations cho ứng dụng. |

Installer ưu tiên file `workspace.toml` của worktree. Nếu file không tồn tại, installer vẫn thử Cargo discovery để branch cũ hoặc branch thử nghiệm có thể được build; các package và GSettings không được tự suy đoán.

## Đọc tiếp

- Cơ chế module và dependency: [02 — Kiến trúc](02-architecture.md).
- Cài đặt và schema manifest: [03 — Cài đặt và build](03-setup.md).
- Vị trí code: [04 — Cấu trúc dự án](04-structure.md).
