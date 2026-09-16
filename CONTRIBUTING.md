# Đóng góp cho BabyDra

Tài liệu này mô tả quy trình thay đổi mã nguồn. Trước khi bắt đầu, đọc [docs/01-overview.md](docs/01-overview.md) và [docs/04-structure.md](docs/04-structure.md) để hiểu branch và ranh giới module.

## Mô hình branch

| Branch | Nội dung | Quy tắc |
| :--- | :--- | :--- |
| `main` | Bộ cài đặt và tài liệu | Không đưa mã nguồn ứng dụng hoặc dữ liệu triển khai của một phiên bản cụ thể vào đây. |
| `release` | Workspace nguồn được phát hành | Mỗi thay đổi về binary, package hoặc configuration phải cập nhật `workspace.toml` nếu cần. |
| `develop` | Tích hợp phát triển | Dùng để kiểm tra thay đổi trước khi cập nhật `release`. |
| Branch cá nhân | Một tính năng hoặc bản thử nghiệm | Có thể được bộ cài đặt phát hiện nếu branch chứa workspace Cargo hợp lệ. |

## Quy trình làm việc

```bash
git fetch origin
git switch develop
git pull --ff-only origin develop
git switch -c <ten-nguoi-dung>/<ten-thay-doi>
```

Thực hiện thay đổi trong phạm vi cần thiết. Không sửa `main` để thêm tên ứng dụng hoặc package mới. Nếu branch của bạn thay đổi những gì cần cài đặt, sửa `workspace.toml` trên chính branch đó.

## Kiểm tra trước khi commit

Trên branch chứa workspace Rust:

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Trên `main`, kiểm tra riêng bộ cài đặt:

```bash
cargo test --manifest-path install/Cargo.toml
cargo check --manifest-path install/Cargo.toml
```

Nếu thay đổi có liên quan đến package, scope hoặc binary, kiểm tra thêm:

1. `workspace.toml` hợp lệ bằng parser TOML.
2. Mỗi `name` trong `[[binaries]]` tạo ra một file trong `target/release`, hoặc có `source` trỏ tới tên file build thực tế.
3. `scope` chỉ dùng `user` hoặc `system`.
4. Package không bị lặp và có đúng nguồn `pacman` hoặc `aur`.
5. Branch nguồn vẫn build được khi không có thay đổi nào trong `main`.

## Quy tắc code

- Mỗi crate có một trách nhiệm rõ ràng.
- Logic không phụ thuộc GTK đặt ở thư viện hoặc module thuần để có thể test độc lập.
- UI dùng component và theme chung; không tạo màu, spacing hoặc widget riêng nếu đã có abstraction tương ứng.
- Chuỗi hiển thị đi qua i18n khi module đã hỗ trợ i18n.
- Không sao chép logic service vào nhiều binary.
- Không thêm danh sách hardcode vào installer để xử lý một crate mới.
- Khi thêm binary, ưu tiên để Cargo discovery phát hiện; chỉ dùng `workspace.toml` để khai báo policy như tên cài đặt, scope và mô tả.

## Quy tắc commit

Dùng Conventional Commits:

```text
type(scope): short description
```

Ví dụ:

```text
feat(panel): add workspace indicator
fix(installer): handle missing branch manifest
docs(setup): document workspace.toml
```

Một commit nên có phạm vi rõ ràng và không trộn thay đổi không liên quan.

## Tài liệu

Nếu thay đổi làm thay đổi hành vi công khai, cập nhật tài liệu tương ứng trong cùng commit. Không sử dụng icon trang trí, lời quảng cáo hoặc mô tả mơ hồ để thay thế cho điều kiện kỹ thuật.
