# BabyDra — Desktop Shell

BabyDra là một môi trường desktop (desktop shell) Linux dựa trên giao thức Wayland, được phát triển dành cho Arch Linux bằng ngôn ngữ Rust và bộ công cụ GTK4 Layer Shell, vận hành trên trình quản lý cửa sổ labwc. Mục tiêu thiết kế của dự án là giảm thiểu tài nguyên hệ thống và độ trễ phản hồi trong khi vẫn đảm bảo đầy đủ các chức năng của một môi trường desktop hiện đại.

## Thành phần hệ thống

Hệ thống được phân tách thành các crate độc lập, mỗi crate đảm nhận một chức năng riêng biệt:

| Thành phần | Crate | Chức năng |
| :--- | :--- | :--- |
| Panel | `babydra-panel` | Dynamic Island, dock, thanh trạng thái, trung tâm thông báo |
| Desktop | `babydra-desktop` | Lớp desktop, hình nền, icon desktop, menu ngữ cảnh |
| Window Switcher | `babydra-switcher` | Chuyển đổi cửa sổ Alt-Tab kèm icon và xem trước |
| File Explorer | `babydra-explore` | Trình quản lý tệp GTK4 |
| Settings | `babydra-settings` | Trung tâm cấu hình hệ thống |
| Launcher | `babydra-launcher` | Lưới ứng dụng và tìm kiếm nhanh |
| Lock | `babydra-lock` | Màn hình khóa với xác thực PAM |
| Greeter | `babydra-greeter` | Màn hình đăng nhập cho greetd/cage |
| Screenshot | `babydra-screenshot` | Chụp màn hình theo vùng, cửa sổ hoặc toàn màn hình |
| Preview | `babydra-preview` | Trình xem ảnh và phương tiện nhanh |
| Keymap | `babydra-keymap` | Trình quản lý phím tắt toàn cục (daemon evdev) |

Ngoài các crate ứng dụng, hệ thống bao gồm hai thư viện dùng chung: `babydra-core` (lớp dịch vụ, mô hình dữ liệu và cấu hình) và `babydra-ui-kit` (bộ thành phần giao diện GTK4 dùng chung).

## Mô hình phân nhánh

Kho mã nguồn được tổ chức theo mô hình ba nhánh, do tác giả trực tiếp quản lý:

| Nhánh | Vai trò |
| :--- | :--- |
| `main` | Kênh phân phối — chỉ chứa bộ cài đặt (`install/`) và tài liệu |
| `release` | Nhánh mặc định, chứa mã nguồn đầy đủ của bản phát hành chính thức |
| `develop` | Nền tảng phát triển, tách ra từ nhánh `release` |

Nhánh `main` không chứa mã nguồn ứng dụng, do đó việc build trực tiếp trên nhánh này là không thực hiện được. Người đóng góp tạo nhánh riêng từ `develop` và làm việc trong nhánh của mình; các nhánh đóng góp có thể được chọn trong bộ cài đặt để cài đặt thử nghiệm.

## Cài đặt

### Yêu cầu hệ thống

- Hệ điều hành: Arch Linux hoặc bản phân phối tương thích.
- Công cụ: `git`, `cargo` (Rust 1.80 trở lên), `sudo`.
- Máy chủ đồ họa: Wayland với trình quản lý cửa sổ `labwc`.

### Bộ cài đặt TUI

Nhánh `main` cung cấp `babydra-installer` — một chương trình cài đặt giao diện dòng lệnh (TUI) viết bằng ratatui, là crate độc lập không phụ thuộc workspace. Để khởi chạy:

```bash
cd install
cargo run --release
```

Hoặc sử dụng tập lệnh đi kèm:

```bash
./install/run.sh
```

Chương trình chấp nhận một đối số tùy chọn là đường dẫn tới thư mục chứa binary đã build (ví dụ `target/release`), cùng các tùy chọn `--help` và `--version`.

### Luồng cài đặt

Bộ cài đặt gồm sáu bước. Các bước cấu hình hệ thống được thực hiện tự động và không yêu cầu xác nhận từng hạng mục; người dùng chỉ lựa chọn nhánh nguồn, thành phần cần cài và variant giao diện:

1. **Welcome** — hiển thị thông tin môi trường: thư mục gốc workspace, thư mục binary nguồn, số lượng thành phần phát hiện được.
2. **Source Branch** — lựa chọn giữa chế độ cài đặt binary có sẵn hoặc checkout một nhánh để build từ nguồn (`cargo build --release`).
3. **Binaries** — danh sách các crate được quét tự động từ thư mục `crates/` của nhánh đã chọn; nếu nhánh hiện tại không chứa mã nguồn, danh sách được truy vấn từ các nhánh `release`/`develop` thông qua `git ls-tree`. Toàn bộ thành phần được chọn mặc định.
4. **Variant** — lựa chọn variant giao diện (theme, danh sách ứng dụng) từ thư mục `variants/`.
5. **Execute** — xác nhận và thực thi kịch bản cài đặt sau khi nhập mật khẩu sudo.
6. **Summary** — tổng kết kết quả và thời gian thực hiện.

Kịch bản cài đặt thực hiện tuần tự các công việc sau:

- Kiểm tra thông tin xác thực sudo một lần duy nhất trước khi bắt đầu (mật khẩu được truyền qua stdin, không hiển thị trên TTY).
- Checkout nhánh đã chọn, đồng bộ với remote và build ở chế độ release (nếu chọn build từ nguồn).
- Chấm dứt các tiến trình đang chạy để tránh lỗi `ETXTBSY` khi ghi đè binary.
- Cài đặt các gói hệ thống cần thiết: `pacman`, trình hỗ trợ AUR `yay`, và các gói từ AUR; cấu hình quyền `i2c-dev`, CPU governor và nhóm `input`.
- Sao chép binary vào `~/.local/bin` (riêng `babydra-greeter` vào `/usr/bin`); khi sao chép vào `/usr/bin` thất bại, binary được đưa về `~/.local/bin` để đảm bảo hệ thống vẫn sử dụng được.
- Đưa binary và tài nguyên hệ thống vào `/var/lib/babydra/bin` với quyền truy cập 0777.
- Đồng bộ cấu hình labwc, GTK 3/4, fontconfig, kitty, neovim, fastfetch; cài đặt theme, icon, cursor; đăng ký tập tin `.desktop`, liên kết MIME và dịch vụ DBus `FileManager1`.
- Triển khai các gói theme vào `~/.babydra/themes` và `/usr/share/babydra/themes`, ghi lựa chọn theme của variant vào `~/.babydra/babydra.conf`.
- Cấu hình greetd (`/etc/greetd/config.toml`), vô hiệu hóa getty trên tty2–6 và kích hoạt `greetd.service`.
- Khởi động lại `babydra-panel` và các tiến trình nền.

### Build thủ công

Trên nhánh chứa mã nguồn (`release` hoặc `develop`), toàn bộ hệ thống được build bằng một lệnh duy nhất:

```bash
cargo build --release
```

## Phát triển và đóng góp

Quy trình làm việc chuẩn trên nhánh `develop`:

```bash
git checkout develop
git pull origin develop
git checkout -b <user>/<workspace>
cargo check --workspace
```

Trước khi gửi thay đổi, thực hiện bộ kiểm tra:

```bash
./scripts/check.sh
```

Tập lệnh này chạy `cargo check`, `cargo fmt`, `cargo clippy` (với `-D warnings`) và `cargo test`. Quy trình và quy tắc đóng góp được trình bày chi tiết trong [CONTRIBUTING.md](CONTRIBUTING.md).

## Tài liệu

Tài liệu kỹ thuật nằm trong thư mục `docs/` (bắt đầu từ [docs/README.md](docs/README.md)):

| Tài liệu | Nội dung |
| :--- | :--- |
| [01 — Tổng quan](docs/01-overview.md) | Giới thiệu, thành phần, mô hình phân nhánh |
| [02 — Kiến trúc](docs/02-architecture.md) | Mẫu thiết kế, mô hình daemon-client, sơ đồ hệ thống |
| [03 — Cài đặt và build](docs/03-setup.md) | Cài đặt qua bộ cài đặt hoặc tập lệnh, build từ nguồn |
| [04 — Cấu trúc và quy chuẩn](docs/04-structure.md) | Cây thư mục, trách nhiệm module, quy ước mã nguồn |
| [05 — Themes và Variants](docs/05-themes-variants.md) | Tạo theme và variant mới |
| [06 — Luồng hoạt động](docs/06-system-flows.md) | Luồng hoạt động của từng crate |
| [07 — Dynamic Island](docs/07-dynamic-island.md) | Sử dụng và mở rộng Dynamic Island |
| [08 — API](docs/08-apis.md) | Tham chiếu API (core, ui-kit, explore) |
| [09 — Design](docs/09-design.md) | Ngôn ngữ thiết kế |
| [10 — Components](docs/10-components.md) | Thư viện thành phần giao diện |

## Giấy phép

Dự án được phát hành theo giấy phép Apache License 2.0. Nội dung đầy đủ của giấy phép được trình bày trong tập tin [LICENSE](LICENSE).
