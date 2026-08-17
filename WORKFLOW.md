# Workflow & Chống Xung Đột

Tài liệu này mô tả mô hình branch, quy trình phát triển và ma trận sở hữu file
để nhiều người có thể phát triển các phiên bản (variants) của BabyDra **song
song với xung đột tối thiểu**. Xem `planning.md` (mục 5.4 và 6) cho chi tiết.

---

## 1. Mô hình branch

```
release  ──(official)──▶  variants/default  (chính thức)
   │
   └── develop ──▶ variant/<user>-<name>   (mỗi user 1 branch variant riêng)
                      │
                      └── merge ngược: chỉ gửi PR phần dùng chung (fix core/ui-kit)
```

- **`release`** — nguồn chính thức ổn định. Mọi thay đổi đi qua PR + review + CI xanh.
- **`develop`** — tích hợp trước khi lên release.
- **`refactor/*`** — nhánh refactor lớn (vd `refactor/constructor`), merge về
  `develop`/`release` sau khi hoàn tất từng phần.
- **`variant/<user>-<name>`** — mỗi user chỉ làm việc trong thư mục
  `variants/<user>-<name>/` của riêng mình + branch riêng → file không bao giờ
  chạm nhau.

**Chính sách merge variant:**

- Chỉ **variant chính thức** (được tác giả duyệt) được merge vào `release`.
- Variant cộng đồng **giữ trên branch riêng** — không merge vào mainline để
  tránh tích lũy file lạ.
- User sửa lỗi ở core/ui-kit → gửi **PR riêng** (tách khỏi variant), review bởi
  owner tương ứng.

---

## 2. Ma trận sở hữu file

| Thư mục | Chủ sở hữu | Người khác được sửa? | Quy trình |
|---|---|---|---|
| `libs/babydra-common/` | Tác giả core | ❌ Chỉ qua PR + review | Breaking change → bump version |
| `libs/babydra-utils/` (ui-kit) | Maintainer UI kit | ❌ Chỉ qua PR + review | Thêm component mới phải có docs + test |
| `libs/babydra-theme/` | Maintainer theme engine | ❌ Chỉ qua PR | API ổn định |
| `libs/babydra-explore-kit/` | Owner explore | PR cho explore | Không đụng crate khác |
| `crates/babydra-<app>/` | Owner từng app | PR cho app đó | Không đụng app khác |
| `themes/<theme-id>/` | Owner theme đó | PR cho theme đó | Không sửa theme của người khác |
| `variants/<user>-<name>/` | **Chỉ user đó** | ❌ Không ai khác | Mỗi người 1 thư mục = 0 conflict |
| `configs/` (seed) | Tác giả | PR | Thay đổi cấu trúc = breaking |
| `docs/`, `planning.md`, `WORKFLOW.md` | Tác giả | PR | — |
| `Cargo.toml` (workspace) | Tác giả | ❌ | Chỉ đổi khi thêm/xóa crate (hiếm) |
| `scripts/` | Tác giả | PR | — |
| `tests/` | Tác giả + owner từng vùng | PR | Test theo vùng tương ứng |

**Quy tắc vàng:** *Không ai được sửa file của người khác trực tiếp; mọi thay
đổi qua PR và owner quyết định.*

---

## 3. Quy trình commit & PR

1. **Luôn làm việc trong file thuộc ownership của mình** (mục 2).
2. **Mỗi phần việc một commit gọn**, message theo chuẩn conventional commits:
   `type(scope): description` — vd `feat(theme):`, `refactor(ui-kit):`,
   `test(common):`, `docs(design):`, `style(css):`. Không dùng "phase0/1/2"
   làm scope.
3. **Trước khi commit**: chạy `./scripts/check.sh` (check + fmt + clippy + test).
4. **Rebase thay vì merge** cho branch cá nhân (giữ lịch sử tuyến tính).
5. **PR**: kèm checklist trong `CONTRIBUTING.md`; bắt buộc CI xanh + review bởi owner.
6. Không sửa workspace `Cargo.toml` trừ khi thêm/xóa crate.

---

## 4. Quy tắc code

1. Thêm component mới phải kèm: docs (`docs/design/components/<name>.md`),
   test cơ bản, CSS đúng file chủ quyền — không ghost class.
2. Không hardcode màu/font/chuỗi trong app — đi qua tokens/config/i18n.
3. CSS: **1 class = 1 file mỗi tầng** — trước khi thêm class mới, grep xem đã
   tồn tại chưa.
4. Thay đổi `babydra-common`/`ui-kit` API = breaking → bump version, deprecate
   trước khi xóa.
5. Test là lưới an toàn: viết/extend test cho module trước khi refactor nó.

---

## 5. Bảo vệ nhánh (GitHub)

- Bắt buộc CI (`scripts/check.sh`) trên `release`, `develop`, `refactor/**`.
- Yêu cầu review 1+ bởi owner cho `release`/`develop`.
- Workflow CI: `.github/workflows/ci.yml`.
