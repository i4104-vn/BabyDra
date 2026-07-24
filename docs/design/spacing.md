# Spacing

Khoảng cách trong BabyDra không phải số ngẫu nhiên — có một hệ thống tư duy đằng sau mỗi giá trị padding, gap, và margin. Hiểu hệ thống này giúp đưa ra quyết định spacing nhất quán mà không cần tra cứu từng trường hợp.

---

## Triết lý: Khoảng trắng là thứ chủ động, không phải khoảng thừa

Trong thiết kế kém, khoảng trắng là "chỗ không có gì". Trong BabyDra, khoảng trắng là công cụ thiết kế chủ động: nó nhóm những thứ liên quan lại gần nhau, và tách những thứ không liên quan ra xa.

Khi hai phần tử ở gần nhau, mắt người đọc chúng như một nhóm. Khi xa hơn, chúng được đọc là hai thực thể riêng. Spacing là ngôn ngữ ngầm của layout.

---

## Hệ thống ba cấp khoảng cách

BabyDra sử dụng ba cấp spacing với ý nghĩa rõ ràng:

**Micro (`4px–6px`)** — Khoảng cách trong cùng một phần tử.
Giữa icon và chữ trong cùng một dòng. Giữa hai thứ thuộc về nhau về mặt ngữ nghĩa. Khoảng cách quá nhỏ để mắt phân biệt là "tách biệt" — chúng đọc như một đơn vị.

**Standard (`8px–12px`)** — Khoảng cách giữa các phần tử trong cùng một nhóm.
Padding bên trong dòng menu. Gap giữa các chip trong toolbar. Giữa avatar và tên người dùng trong profile header. Đây là khoảng cách "thoải mái" — phần tử có hơi thở nhưng vẫn rõ ràng là cùng nhóm.

**Section (`16px–20px`)** — Khoảng cách giữa các nhóm chức năng khác nhau.
Padding viền trong dropdown. Khoảng cách giữa profile header và danh sách menu bên dưới. Giữa các section trong settings. Đây là tín hiệu "hết nhóm này, bắt đầu nhóm mới".

---

## Padding bên trong phần tử

Padding không đồng nhất giữa mọi loại phần tử — nó tỷ lệ với kích thước và chức năng của phần tử:

- **Badge nhỏ**: `2px 8px` — padding ít vì phần tử nhỏ, thêm nhiều trông phồng.
- **Chip tương tác**: `4px–6px` theo chiều dọc, `10px–12px` theo chiều ngang — đủ diện tích để dễ nhấn.
- **Dòng menu**: `8px` dọc, `12px` ngang — tỷ lệ vàng cho list item có thể click.
- **Nút bấm lớn**: `10px–12px` dọc, `20px–24px` ngang.
- **Dropdown container**: `12px–16px` bốn cạnh.

---

## Tại sao không dùng hệ thống bội số 8

Nhiều hệ thống dùng bội số 8 (8, 16, 24, 32...) vì tính toán dễ. BabyDra không cứng nhắc theo điều này — `12px` và `6px` đều là giá trị hợp lệ khi ngữ cảnh yêu cầu. Quan trọng hơn là **cảm giác cân bằng thị giác** chứ không phải con số có chia hết cho 8 không.

Tuy nhiên, hạn chế dưới 6 giá trị spacing khác nhau trong cùng một component. Nếu thấy mình đang dùng `7px`, `9px`, `11px`, `13px` — đó là dấu hiệu cần đơn giản hóa.

---

## Khoảng cách trong Glassmorphic layout

Giao diện kính mờ có một yếu tố spacing đặc biệt: các surface không chạm vào nhau. Giữa panel này và panel kia luôn có một khoảng hở để wallpaper nền nhìn qua. Điều này tăng cường cảm giác "floating" và "trong suốt" của hệ thống.

Khoảng hở tối thiểu giữa hai surface cạnh nhau: `8px–12px`. Nhỏ hơn thì hai surface trông như dính vào nhau và mất đi cảm giác kính mờ.
