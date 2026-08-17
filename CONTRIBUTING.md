# Contributing to BabyDra

Cảm ơn bạn quan tâm đóng góp! Đọc kỹ `WORKFLOW.md` (mô hình branch + ma trận
sở hữu) trước khi bắt đầu.

## Checklist bắt buộc (PR template)

Trước khi mở PR, kiểm tra tất cả:

- [ ] Chạy `./scripts/check.sh` xanh (check + fmt + clippy + test)
- [ ] Chỉ sửa file thuộc ownership của mình (xem `WORKFLOW.md` mục 2)
- [ ] Không sửa `Cargo.toml` workspace trừ khi thêm/xóa crate
- [ ] Đổi logic `common`/`ui-kit`/`theme` → có test đi kèm
- [ ] Thêm component mới → có docs (`docs/design/components/`) + test + CSS
      đúng file chủ quyền (không ghost class)
- [ ] Không hardcode màu/font/chuỗi — qua tokens/config/i18n
- [ ] CSS: 1 class = 1 file mỗi tầng (grep trước khi thêm class)
- [ ] Commit gọn, conventional commits: `type(scope): description`
- [ ] Message PR tóm tắt thay đổi + ảnh/screenshot nếu là UI

## Quy trình

1. Fork / checkout nhánh riêng: `git checkout -b variant/<user>-<name>` hoặc
   `feature/<name>` từ `develop`.
2. Code theo checklist trên, commit từng phần.
3. Rebase lên `develop` (không merge) trước khi mở PR.
4. PR về `develop`; owner tương ứng review. Chỉ tác giả mới merge lên `release`.

## Test

```bash
cargo test --workspace          # toàn bộ
cargo test -p babydra-tests     # chỉ suite integration
./scripts/check.sh              # check + fmt + clippy + test
```

Thêm test mới → xem `tests/README.md`.
