# 03 — Cài đặt và build

## Phạm vi

Trang này mô tả yêu cầu hệ thống, cách chạy installer từ `main`, cách build workspace nguồn và schema của `workspace.toml`.

## Yêu cầu hệ thống

| Thành phần | Yêu cầu |
| :--- | :--- |
| Hệ điều hành | Arch Linux hoặc hệ tương thích với `pacman`. |
| Session | Wayland; các cấu hình hiện tại dùng labwc. |
| Toolchain | Rust và Cargo tương thích với `Cargo.toml` của branch nguồn. |
| Quyền | `sudo` cho package, binary system, `/var/lib` và greetd. |
| Mạng | Cần cho `git fetch`, package repository và AUR nếu branch khai báo. |

Các package cụ thể không được ghi trong tài liệu `main`. Chúng thuộc `workspace.toml` của branch được cài.

## Chạy installer

Từ root của repository trên `main`:

```bash
./install/run.sh
```

Hoặc chạy trực tiếp:

```bash
cd install
cargo run --release
```

Installer thực hiện các bước chính:

1. Tìm repository root và liệt kê local/remote branch có `Cargo.toml`.
2. Cho người dùng chọn branch nguồn và binary cần cài.
3. Checkout branch vào `branches/<branch>` để không thay đổi branch hiện tại của repository chính.
4. Đọc `branches/<branch>/workspace.toml` nếu file tồn tại.
5. Build bằng `cargo build --release --workspace`.
6. Cài package được manifest khai báo, copy binary và staging binary.
7. Đồng bộ config, theme, desktop entry, D-Bus service, user service và greetd theo tài nguyên có trong source tree.

Installer xác thực sudo trước khi chạy task có thay đổi hệ thống. Nếu xác thực thất bại, worker dừng trước khi thực hiện cài đặt một phần.

## Đường dẫn sau khi cài

| Dữ liệu | Đường dẫn mặc định |
| :--- | :--- |
| Binary scope `user` | `~/.local/bin/<name>` |
| Binary scope `system` | `/usr/bin/<name>` |
| Binary staging | `/var/lib/babydra/bin/` |
| User config | `~/.config/` |
| Theme runtime | `~/.babydra/themes/` và `/usr/share/babydra/themes/` |
| Variant selection | `~/.babydra/babydra.conf` |
| User systemd unit | `~/.config/systemd/user/` |
| Greetd config | `/etc/greetd/config.toml` |

## Schema `workspace.toml`

File nằm tại root branch nguồn:

```toml
[[binaries]]
name = "babydra-panel"       # Tên file đích sau khi cài
source = "babydra-panel"     # Tên file trong target/release; mặc định bằng name
scope = "user"               # user hoặc system
description = "Desktop panel"
export_desktop = false       # chỉ sinh .desktop khi đặt true

[[binaries]]
name = "babydra-greeter"
scope = "system"
export_desktop = false

[[binaries]]
name = "babydra-explore"
scope = "user"
export_desktop = true

[packages]
pacman = ["gtk4", "labwc"]
aur = ["fastfetch"]

[installer]
features = ["wtype", "kernel_permissions", "greetd"]

[gsettings]
"org.gnome.desktop.interface.font-name" = "Inter 11"
"org.gnome.desktop.interface.cursor-size" = "24"
```

### `[[binaries]]`

| Field | Bắt buộc | Ý nghĩa |
| :--- | :--- | :--- |
| `name` | Có | Tên binary đích và tên được dùng khi kiểm tra process/target. |
| `source` | Không | Tên file executable trong `target/release`. Mặc định bằng `name`. |
| `scope` | Không | `user` hoặc `system`; mặc định là `user`. |
| `description` | Không | Mô tả hiển thị trong TUI; nếu thiếu, installer dùng mô tả tổng quát. |
| `export_desktop` | Không | Boolean, mặc định `false`; sinh `<name>.desktop` cho binary khi là `true`. Nếu source đã có file cùng tên, installer dùng file source. |

Nếu Cargo target có tên khác tên cài đặt, dùng `source`. Ví dụ:

```toml
[[binaries]]
name = "my-shell"
source = "shell-daemon"
scope = "user"
export_desktop = true
```

`export_desktop` chỉ áp dụng cho desktop entry được installer sinh tự động. Các file `.desktop` có sẵn trong source branch vẫn được cài đặt và giữ nguyên nội dung.

### `[packages]`

`pacman` và `aur` là mảng chuỗi package. Installer dùng `--needed` để tránh cài lại package đã có. Nếu một mảng rỗng hoặc không tồn tại, task tương ứng được bỏ qua.

Pacman được gọi theo dạng:

```text
sudo pacman -Syu --needed --noconfirm <packages...>
```

AUR được gọi qua `yay` theo dạng:

```text
yay -S --noconfirm --needed <packages...>
```

Nếu branch cần AUR nhưng máy chưa có `yay`, installer bootstrap `yay-bin` trước khi cài danh sách AUR.

### `[installer]`

`features` là mảng tùy chọn để branch bật các bước hệ thống không thể suy ra an toàn từ danh sách package hoặc binary. Các giá trị được hỗ trợ hiện tại là:

| Giá trị | Tác dụng |
| :--- | :--- |
| `wtype` | Build helper Wayland `wtype` nếu hệ thống chưa có. |
| `kernel_permissions` | Cấu hình quyền i2c, CPU và nhóm input cho phần cứng của workspace. |
| `greetd` | Cấu hình, mask VT phụ và enable `greetd.service`. |

Không thêm feature nếu branch không sử dụng chức năng tương ứng. Installer không chạy các bước này theo mặc định.

### `[gsettings]`

Key có dạng `<schema>.<key>`, giá trị là string. Installer tách ở dấu chấm cuối cùng rồi gọi `gsettings set`. Các cấu hình không phải string hiện chưa thuộc schema này.

## Build branch nguồn

Trên branch có `Cargo.toml` workspace:

```bash
cargo check --workspace
cargo test --workspace
cargo build --release --workspace
```

Build một package:

```bash
cargo build -p babydra-panel
cargo run -p babydra-settings
```

Kiểm tra format và lint:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Installer độc lập trên `main` được kiểm tra bằng:

```bash
cargo test --manifest-path install/Cargo.toml
cargo check --manifest-path install/Cargo.toml
```

## Troubleshooting

### Không thấy branch trong TUI

Kiểm tra repository có remote `origin`, branch có `Cargo.toml` ở root và fetch có hoàn tất trong thời gian chờ. Installer lọc branch không có workspace để tránh hiển thị lựa chọn không build được.

### Không thấy binary

Kiểm tra `target/release/<source>` sau build. Nếu tên file khác tên cài đặt, thêm `source` vào `workspace.toml`. Nếu binary mới chưa có manifest, Cargo discovery vẫn có thể tìm `src/main.rs` hoặc `src/bin/*`.

### Package không được cài

Xác nhận package nằm đúng mảng `pacman` hoặc `aur` trong `workspace.toml` của branch đã chọn. Không thêm package vào `main`.

### Worktree lỗi hoặc chứa mã cũ

Xóa worktree branch cụ thể sau khi xác nhận không có thay đổi cần giữ, rồi chạy installer lại. Không xóa toàn bộ thư mục repository hoặc dùng thao tác reset trên branch đang làm việc.
