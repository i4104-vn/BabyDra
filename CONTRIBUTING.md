# Contributing to BabyDra

Cảm ơn bạn quan tâm đóng góp! Trước khi bắt đầu, hãy nắm mô hình phân nhánh và
quy chuẩn viết mã trong [`docs/04-structure.md`](docs/04-structure.md).

## Mô hình phân nhánh

| Nhánh | Vai trò | Quyền hạn |
| :--- | :--- | :--- |
| `main` | Kênh phân phối — chỉ chứa bộ cài đặt (`install/`) + tài liệu | Chỉ tác giả |
| `release` | **Nhánh mặc định** — mã nguồn đầy đủ chính thức | Chỉ tác giả |
| `develop` | Nền tảng phát triển, tách ra từ `release` | Chỉ tác giả |

**Không ai ngoài tác giả có thể push trực tiếp vào `main`, `release` hoặc `develop`.**
Người đóng góp chỉ làm việc trong **nhánh riêng của mình** (tách từ `develop`);
bộ cài đặt `babydra-installer` liệt kê các nhánh đó để người dùng cài đặt thử nghiệm.

## Checklist bắt buộc (trước khi gửi thay đổi)

- [ ] Chạy `./scripts/check.sh` xanh (check + fmt + clippy `-D warnings` + test)
- [ ] Chỉ sửa file thuộc phạm vi module mình làm (xem `docs/04-structure.md` mục 4)
- [ ] Không sửa `Cargo.toml` workspace trừ khi thêm/xóa crate
- [ ] Đổi logic `libs/babydra-core`, `libs/babydra-ui-kit`, `libs/babydra-theme`
      → có test đi kèm (xem `tests/README.md`)
- [ ] Thêm component mới → có docs (`docs/10-components.md`) + test + CSS
      đúng file chủ quyền (không ghost class)
- [ ] Không hardcode màu/font/chuỗi — qua tokens/config/i18n
- [ ] CSS: 1 class = 1 file mỗi tầng (grep trước khi thêm class)

## Quy trình

1. Checkout từ `develop`: `git checkout develop && git pull origin develop`.
2. Tạo **nhánh riêng của bạn**: `git checkout -b <tên-bạn>/<tên-công-việc>`.
3. Code theo checklist trên, commit từng phần và push nhánh của bạn lên remote.

Lưu ý: dự án **không dùng Pull Request**. Bạn **không có quyền push trực tiếp**
vào `main`/`release`/`develop` — mọi thay đổi đều đi qua nhánh riêng của bạn

## Test

```bash
cargo test --workspace          # toàn bộ workspace
cargo test -p babydra-tests     # chỉ suite integration (tests/)
./scripts/check.sh              # check + fmt + clippy + test
```

Thêm test mới → xem [`tests/README.md`](tests/README.md) để đặt file đúng
thư mục và đăng ký `[[test]]` trong `tests/Cargo.toml`.

## Đóng góp tài liệu

- Sửa tài liệu cùng commit với code — nếu đổi API, đổi luôn docs.
- Thêm nội dung mới: bổ sung vào trang chủ đề tương ứng (`01-overview`…
  `10-components`) — chỉ tạo trang mới khi chủ đề thực sự tách biệt.
- Mỗi file bắt đầu bằng metadata: **Phạm vi** / **Phiên bản** / **Cập nhật lần cuối**.
- Kiểm tra liên kết chéo sau khi đổi tên/di chuyển file.

Quy ước viết tài liệu chi tiết: [`docs/README.md`](docs/README.md) (mục "Quy ước viết tài liệu").
