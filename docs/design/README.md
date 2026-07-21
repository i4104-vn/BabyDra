# Tài liệu Thiết kế Giao diện Người dùng (UI/UX Design Specification)

Chào mừng bạn đến với tài liệu hướng dẫn và đặc tả thiết kế giao diện cho ứng dụng **BabyDra** (AI Generation Studio). 

Tài liệu này tập trung mô tả chi tiết **hướng thiết kế thị giác (Visual Design)**, **trải nghiệm người dùng (UX)**, **ngôn ngữ thiết kế**, **tỉ lệ bố cục** và **trạng thái tương tác** của từng thành phần giao diện theo chuẩn danh mục Bootstrap.

---

## 🎨 1. Hướng Thiết kế Tổng quan (Design Direction & Concept)

- **Phong cách Thiết kế**: Modern Minimalist (Tối giản hiện đại) kết hợp hiệu ứng Nổi (Elevated Floating Cards) và bo góc lớn mềm mại (Soft Rounded Shapes).
- **Tông màu Chủ đạo**: Gam màu xám kem ấm nhạt (`Warm Pinkish Gray`) tạo cảm giác hiện đại, nghệ thuật và thân thiện.
- **Trải nghiệm Thị giác (Visual Experience)**:
  - Bố cục mở, tập trung tối đa vào tác phẩm nghệ thuật ở trung tâm.
  - Sử dụng khoảng trắng (Negative Space) hợp lý để giảm bớt sự chói mắt và phân tách các khu vực chức năng một cách tự nhiên.
  - Các yếu tố tương tác (Nút bấm, Ô nhập liệu, Toolbar) sử dụng dạng khoang nhộng (Pill-shaped elements) giúp giao diện trông mềm mại và sang trọng.

---

## 📐 2. Bảng Hướng dẫn Thiết kế (Design Guidelines)

- **[Design Tokens & Specs](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/design_tokens.md)**: Quy định toàn bộ quy chuẩn về Màu sắc (Palette), Typography (Kiểu chữ & Kích thước), Bo góc (Border Radius), Khoảng cách (Spacing) và Bóng đổ (Shadows & Elevation).

---

## 📦 3. Danh mục Thiết kế các Thành phần (UI Component Specs)

| Thành phần (Component) | Mô tả Thiết kế Chi tiết |
| :--- | :--- |
| **[Header Navbar](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/navbar.md)** | Định hướng thanh tiêu đề chính: Logo nhận diện, bộ đếm Credit ngày, nút nâng cấp nổi bật và Avatar tài khoản. |
| **[Navs & Tabs](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/navs_tabs.md)** | Định hướng thiết kế thanh chuyển danh mục (Center Navigation Pill) và thanh quản lý phiên làm việc dọc mép phải (Session Sidebar). |
| **[Prompt Panel](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/prompt_panel.md)** | Định hướng thiết kế Panel ngữ cảnh bên trái: Ảnh tham chiếu góc nhìn ống kính fisheye, các thẻ từ khóa (Chips) và văn bản mô tả nghệ thuật. |
| **[Preview Panel](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/preview_panel.md)** | Định hướng thiết kế Vùng hiển thị ảnh kết quả chính kích thước lớn và cột chọn 4 ảnh biến thể (Variations) với hiệu ứng viền đè active. |
| **[Input Group](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/input_group.md)** | Định hướng thiết kế Cụm ô nhập liệu Prompt nổi ở phía dưới màn hình, nút bấm `Generate` đen tuyền và dãy nút điều chỉnh thông số (4:3, 4K, Style...). |
| **[Buttons](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/buttons.md)** | Định hướng các loại nút bấm: Nút Primary (`Generate`), Nút Secondary (`Upgrade now`), Nút biểu tượng (Theme toggle, `+` create). |
| **[Cards](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/card.md)** | Định hướng cấu trúc các hộp chứa (Cards): Khung ảnh kết quả, Thẻ biến thể, Thẻ ảnh tham chiếu và Khung nhập liệu. |
| **[Badges & Chips](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/badge.md)** | Định hướng các nhãn từ khóa Prompt (Token Chips) và huy hiệu biểu tượng (Palette badge). |
| **[Dropdowns](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/dropdowns.md)** | Định hướng menu nổi (Popover Dropdown) lựa chọn thông số cấu hình. |
| **[Progress](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/progress.md)** | Định hướng vòng phần trăm Credit (`18% Daily Credits`). |
| **[Spinners & Loaders](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/spinners.md)** | Định hướng hiệu ứng phản hồi trạng thái chờ khi AI đang sinh ảnh. |
| **[Tooltips](file:///c:/Users/Administrator/Documents/GitHub/BabyDra/docs/design/components/tooltips.md)** | Định hướng hiển thị thông tin hỗ trợ khi rê chuột qua icon và avatar. |

---

## 🖼️ 4. Sơ đồ Bố cục Không gian (Spatial Layout Hierarchy)

```text
+-----------------------------------------------------------------------------------+
|  [Logo]                 [Nav Pills: Home/Chat/Img/Vid/Folder]       [Controls]    | <- Header Area
+-----------------------------------------------------------------------------------+
|               |                                           | [Variations] | [Side] |
| [Left Panel]  |  [Center Main Preview Area]               |  Thumbnails  | [Bar ] |
| Ref Image     |   Bức ảnh kết quả hiển thị trung tâm     |  [Var 1*]    |  (+)   |
| Prompt Chips  |   Tỷ lệ 1:1 / 4:3 bo góc mềm mại         |  [Var 2 ]    |  (🦊)  |
| Narrative Text|                                           |  [Var 3 ]    |  (🐻)  |
|               |                                           |  [Var 4 ]    |  (🦎)  |
+-----------------------------------------------------------------------------------+
|                 [Bottom Floating Control Center & Prompt Input]                   | <- Floating Action Zone
+-----------------------------------------------------------------------------------+
```
