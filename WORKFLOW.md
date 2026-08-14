# Quy chuẩn Phân nhánh và Quy trình Phát triển Hệ thống BabyDra

## 1. Nguyên tắc tổ chức kho mã nguồn

Hệ thống quản lý phiên bản của dự án BabyDra được phân tách thành 3 nhánh chính nhằm đảm bảo tính toàn vẹn của mã nguồn phân phối, đồng thời thiết lập môi trường phát triển phân tán cho cộng đồng:

```
                            Kho mã nguồn BabyDra
                                     │
           ┌─────────────────────────┴─────────────────────────┐
           │                                                   │
           ▼                                                   ▼
       Nhánh main                                        Nhánh release
   (Kênh phân phối TUI)                              (Mã nguồn chính thức)
           │                                                   │
           │                                                   │ Checkout
           │                                                   ▼
           │                                             Nhánh develop
           │                                         (Nền tảng phát triển)
           │                                                   │
           │                                                   │ Checkout
           │                                                   ▼
           ├─────── Cài đặt từ kênh Release ─────────── feature/<tên-user>
           └─────── Cài đặt từ kênh Develop ────────── (Không gian riêng)
```

---

## 2. Chi tiết vai trò của các nhánh

### 2.1. Nhánh `main` (Kênh phân phối & Bộ cài đặt)
- **Nhiệm vụ**: Chỉ lưu trữ bộ cài đặt TUI (`babydra-installer`), script thực thi (`install.sh`) và tài liệu hướng dẫn (`README.md`, `WORKFLOW.md`, `docs/`).
- **Cơ chế hoạt động**: Khi người dùng kích hoạt cài đặt, công cụ sẽ tự động kéo mã nguồn từ nhánh chỉ định (`release` hoặc `develop`), biên dịch các gói nhị phân và cài đặt vào hệ thống (`~/.local/bin` và `/var/lib/babydra`).
- **Quy định**: Không chứa mã nguồn phát triển trực tiếp của các ứng dụng đồ họa để tối ưu dung lượng tải về.

### 2.2. Nhánh `release` (Mã nguồn chính thức)
- **Nhiệm vụ**: Lưu trữ toàn bộ mã nguồn đầy đủ và đã qua kiểm thử của tác giả (`crates/`, `libs/`, `configs/`, assets).
- **Quyền hạn**: Do tác giả trực tiếp kiểm soát, cam kết tính ổn định và bảo mật cao nhất.

### 2.3. Nhánh `develop` (Nền tảng phát triển cộng đồng)
- **Nhiệm vụ**: Nhánh mốc xuất phát được checkout từ `release` để tích hợp các tính năng thử nghiệm và biến thể mở rộng.
- **Quy tắc bảo vệ**: Nhánh `develop` được bảo vệ (Protected). Các nhà phát triển không được phép merge trực tiếp mã nguồn vào `develop`.

### 2.4. Không gian nhánh cá nhân `feature/<tên-user>`
- **Quy chuẩn đặt tên**: Bắt buộc tuân thủ tiền tố `feature/<tên-người-dùng>` (ví dụ: `feature/nguyenvana`, `feature/tranvanb/dark-mode`).
- **Phạm vi thao tác**: Lập trình viên có toàn quyền tạo nhánh con, checkout và merge nội bộ trong không gian nhánh của mình.
- **Quy định cô lập**: Không thực hiện merge trực tiếp vào `develop` hoặc `release`.

---

## 3. Quy trình đồng bộ mã nguồn cho lập trình viên

Khi tác giả cập nhật tính năng mới vào nhánh `develop`, các lập trình viên trên nhánh `feature/<tên-user>` thực hiện đồng bộ theo quy chuẩn sau nhằm tránh xung đột mã nguồn:

```bash
# 1. Chuyển về nhánh làm việc cá nhân
git checkout feature/<tên-user>

# 2. Lấy dữ liệu mới nhất từ remote
git fetch origin develop

# 3. Áp dụng kỹ thuật Git Rebase để đặt các commit cá nhân lên trên mã nguồn mới
git rebase origin/develop
```

### Ưu điểm kỹ thuật của Git Rebase:
- Giữ lịch sử commit tuyến tính, không tạo merge commit dư thừa.
- Nếu phát sinh xung đột, Git sẽ cô lập chính xác tại commit gây lỗi để xử lý cục bộ trước khi tiếp tục.

---

## 4. Quy trình biên dịch và kiểm thử trước khi commit

Trước khi commit bất kỳ thay đổi nào, lập trình viên phải bảo đảm toàn bộ workspace biên dịch thành công mà không phát sinh lỗi:

```bash
# 1. Kiểm tra cú pháp và kiểu dữ liệu trên toàn bộ workspace
cargo check --workspace

# 2. Biên dịch bản phát hành kiểm thử
cargo build --release --workspace

# 3. Kiểm tra định dạng mã nguồn theo chuẩn Rust
cargo fmt --check
```
