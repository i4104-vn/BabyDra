# Chương 03: Cấu trúc Dự án BabyDra

**Phiên bản:** 1.1.0  
**Phạm vi:** Mô tả chi tiết cấu trúc thư mục, trách nhiệm của từng module và quy chuẩn tổ chức mã nguồn

---

## 1. Cấu trúc thư mục tổng thể

```
BabyDra/
├── Cargo.toml                  # Workspace manifest
├── install/                    # Bộ công cụ cài đặt TUI (babydra-installer)
│   ├── Cargo.toml
│   ├── run.sh                  # Script thực thi nhanh bộ cài đặt
│   └── src/
│       ├── main.rs
│       ├── app/                # Quản lý trạng thái và bộ xử lý sự kiện phím
│       ├── models/             # Kiểu dữ liệu, enum bước và kênh phát hành
│       ├── system/             # Thao tác hệ thống tập tin và kiểm tra quyền
│       ├── tasks/              # Tác vụ pull code, biên dịch và sao chép binary
│       └── ui/                 # Giao diện hai khung chia (Ratatui 2-Panel)
├── crates/                     # Các ứng dụng đồ họa thực thi độc lập
│   ├── babydra-panel/          # Thanh taskbar, dock, khay hệ thống, control center
│   ├── babydra-switcher/       # Trình chuyển đổi cửa sổ Alt+Tab
│   ├── babydra-screenshot/     # Công cụ chụp màn hình
│   ├── babydra-lock/           # Màn hình khóa
│   ├── babydra-greeter/        # Màn hình đăng nhập hệ thống tương thích Greetd
│   ├── babydra-settings/       # Ứng dụng cài đặt hệ thống
│   ├── babydra-preview/        # Trình xem trước tập tin hình ảnh
│   └── babydra-explore/        # Trình quản lý tập tin đồ họa
├── libs/                       # Các thư viện dùng chung nội bộ
│   ├── babydra-common/         # Dịch vụ hệ thống, D-Bus, sysfs, mô hình dữ liệu
│   ├── babydra-utils/          # Định kiểu CSS, widget dùng chung, theme
│   ├── babydra-launcher/       # Logic phân tích và khởi chạy ứng dụng
│   └── babydra-island/         # Giao diện Dynamic Island
├── configs/                    # Cấu hình mẫu cho hệ thống
│   ├── labwc/                  # Cấu hình rc.xml, autostart, scripts
│   ├── kitty/                  # Cấu hình terminal emulator
│   ├── nvim/                   # Cấu hình trình soạn thảo Neovim
│   └── themes/                 # Giao diện GTK, bộ biểu tượng We10X và con trỏ
├── docs/                       # Tài liệu kỹ thuật chi tiết
├── README.md                   # Hướng dẫn tổng quan
└── WORKFLOW.md                 # Quy chuẩn phân nhánh và quy trình phát triển
```

---

## 2. Trách nhiệm chi tiết của các thành phần

### 2.1. Bộ cài đặt TUI (`install/`)
- Cung cấp giao diện tương tác dạng bảng điều khiển để người dùng cấu hình các bước triển khai.
- Chịu trách nhiệm kéo mã nguồn từ nhánh chỉ định (`release` hoặc `develop`), thực hiện quy trình biên dịch tối ưu (`cargo build --release`), và sao chép các tệp thực thi vào đường dẫn đích (`~/.local/bin` và `/var/lib/babydra/bin`).

### 2.2. Nhóm ứng dụng đồ họa (`crates/`)
- **`babydra-panel`**: Đóng vai trò là tiến trình nền duy trì thanh tác vụ, lắng nghe tín hiệu phím tắt qua Unix Domain Socket để mở menu điều khiển hoặc khay hệ thống.
- **`babydra-greeter`**: Tích hợp với `greetd` và `cage` trên Wayland VT1 để xác thực phiên đăng nhập người dùng.
- **`babydra-settings`**: Giao diện quản lý cấu hình hệ thống (mạng không dây, bluetooth, thiết bị hiển thị, quản lý năng lượng và thay đổi hình nền).

### 2.3. Nhóm thư viện dùng chung (`libs/`)
- **`babydra-common`**: Đóng gói các tác vụ truy vấn kernel sysfs, điều khiển giao tiếp liên tiến trình qua D-Bus, và xử lý tập tin cấu hình `~/.babydra/babydra.conf`.
- **`babydra-utils`**: Đảm bảo tính nhất quán về giao diện bằng cách nạp bộ stylesheet CSS toàn cục vào ngữ cảnh hiển thị đồ họa GTK.

---

## 3. Quy chuẩn cấu trúc nội bộ của từng module

Mỗi module đồ họa trong `crates/` phải tuân thủ cấu trúc phân tách logic và giao diện:

- `mod.rs`: Khai báo module, định nghĩa kiểu dữ liệu và cấu trúc điều khiển.
- `render.rs`: Xây dựng cây phân cấp widget giao diện bằng GTK4.
- `handlers.rs`: Xử lý các sự kiện tương tác từ người dùng (click, phím bấm, thay đổi trạng thái).
