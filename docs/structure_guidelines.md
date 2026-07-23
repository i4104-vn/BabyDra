# Quy chuẩn Đặt tên Thư mục và Phân tách Mã nguồn (Coding Standards)

Tài liệu này hướng dẫn chi tiết quy chuẩn đặt tên thư mục, thư mục con (sub-folder) và triết lý phân tách các file mã nguồn (code-splitting) đang được áp dụng trong dự án **BabyDra**.

---

## 1. Quy chuẩn đặt tên thư mục (Directory Naming)

### 1.1. Thư mục Crate chính (Root-level Crates)
Toàn bộ các crate ứng dụng và thư viện dùng chung ở tầng gốc của dự án được đặt tên theo định dạng **kebab-case** (chữ thường, phân tách bằng dấu gạch nối):
- Ví dụ ứng dụng: `crates/babydra-panel/`, `crates/babydra-explore/`, `crates/babydra-screenshot/`.
- Ví dụ thư viện: `libs/babydra-common/`, `libs/babydra-utils/`, `libs/babydra-island/`.

### 1.2. Thư mục con mã nguồn (Sub-folders under `src/`)
Tất cả các thư mục con phục vụ phân tách module Rust bên trong thư mục `src/` đều phải sử dụng định dạng **snake_case** (chữ thường, phân tách bằng dấu gạch dưới):
- Ví dụ phân tách module logic: `libs/babydra-common/src/services/system/volume/`.
- Ví dụ phân tách widget UI: `crates/babydra-panel/src/widgets/panel/items/darkmode/`.

---

## 2. Phương pháp Phân tách File Code (Code Splitting Philosophy)

Nhằm tránh hiện tượng các file mã nguồn phình to khó bảo trì (monolithic files), dự án tuân thủ nghiêm ngặt quy tắc phân tách nhiệm vụ (Separation of Concerns - SoC):

### 2.1. Phân tách theo cấu trúc Mod - Render
Một module giao diện điển hình sẽ được tách tối thiểu thành hai file:

1. **`mod.rs` (Cổng kết nối nghiệp vụ - Controller):**
   - Đóng vai trò là file cấu hình chính của thư mục module.
   - Nhiệm vụ: Đăng ký các module con, ánh xạ dữ liệu nghiệp vụ lấy từ `babydra-common` và gắn các callback xử lý sự kiện (event handling) khi người dùng tương tác.
   - Ví dụ: [backlight/mod.rs](file:///home/i4104/BabyDra/crates/babydra-panel/src/widgets/panel/items/backlight/mod.rs).
2. **`render.rs` (Chỉ dựng giao diện - View):**
   - Nhiệm vụ: Tập trung hoàn toàn vào việc dựng layout GTK (GtkBox, GtkButton, GtkScale), gán thuộc tính hiển thị và thêm các class CSS cho widget.
   - **Tuyệt đối không** chứa logic đọc/ghi hệ thống phức tạp trong file này.
   - Ví dụ: [backlight/render.rs](file:///home/i4104/BabyDra/crates/babydra-panel/src/widgets/panel/items/backlight/render.rs).

### 2.2. Phân tách Dữ liệu và Xử lý nghiệp vụ (Models vs Services)
Tại thư viện lõi `babydra-common`, dữ liệu và thuật toán xử lý dữ liệu được tách rời hoàn toàn:
- **Tầng dữ liệu (`models`):** Chỉ chứa định nghĩa cấu trúc dữ liệu (`struct`, `enum`) tinh khiết để vận chuyển và lưu trữ trạng thái, không chứa thuật toán. Tất cả được lưu tập trung tại [libs/babydra-common/src/models/](file:///home/i4104/BabyDra/libs/babydra-common/src/models/mod.rs).
- **Tầng xử lý (`services`):** Chứa mã lệnh giao tiếp trực tiếp với hệ điều hành (lệnh Terminal, đọc ghi file hệ thống `/sys/class`, socket IPC...). Tất cả được lưu tại [libs/babydra-common/src/services/](file:///home/i4104/BabyDra/libs/babydra-common/src/services/mod.rs).

---

## 3. Bản đồ cấu trúc File mẫu (Reference Directory Tree)

Dưới đây là sơ đồ cấu trúc của một phần hệ thống để làm mẫu khi phát triển module mới:

```text
babydra-panel (Crate ứng dụng)
├── Cargo.toml
└── src/
    ├── main.rs (Khởi tạo App)
    ├── render.rs (Cấu hình Layer Shell, ghép layout chính)
    └── widgets/
        ├── mod.rs (Khai báo danh sách widgets)
        └── panel/
            ├── mod.rs (Quản lý trạng thái Control Center)
            └── items/
                ├── mod.rs (Xuất bản các toggle con)
                └── volume/
                    ├── mod.rs (Bắt sự kiện âm lượng thay đổi, cập nhật hardware)
                    └── render.rs (Tạo nút Icon loa, thanh trượt âm lượng GtkScale)
```

## 4. Nguyên tắc chung khi viết mã nguồn mới

1. **Không lạm dụng `mod.rs`:** Hạn chế viết logic tính toán hoặc giao diện dài quá 150 dòng trong `mod.rs`. Hãy tạo file con chuyên biệt (như `render.rs`, `helper.rs`) và gọi chúng từ `mod.rs`.
2. **Re-export trực quan:** Sử dụng `pub use` tại `mod.rs` hoặc `lib.rs` cấp cha để tạo ra giao diện API phẳng (flat access), giúp các crate ứng dụng gọi thư viện một cách ngắn gọn nhất (ví dụ: `babydra_common::verify_password` thay vì đường dẫn dài `babydra_common::services::system::auth::verify_password`).
3. **Cách ly hoàn toàn CSS:** Toàn bộ CSS định dạng giao diện của toàn bộ dự án **phải** được đặt trong thư mục `libs/babydra-utils/src/styles/` và nạp thông qua [ui/theme/mod.rs](file:///home/i4104/BabyDra/libs/babydra-utils/src/ui/theme/mod.rs), không viết mã CSS inline cứng trực tiếp trong Rust code. Các tệp CSS dùng chung (`button.css`, `switch.css`, `sidebar.css`, `scrollbar.css`) được lưu trữ tập trung tại thư mục `styles/<theme>/shared/`.
