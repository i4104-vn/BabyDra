# Hướng dẫn Thiết kế Giao diện BabyDra

Tài liệu hướng dẫn cách thiết kế giao diện ứng dụng BabyDra (AI Generation Studio). Đọc `design_tokens.md` trước khi đọc bất kỳ hướng dẫn component nào.

---

## Phong cách thiết kế

Dùng phong cách Glassmorphic Acrylic, hỗ trợ Dark & Light Theme. Mọi bề mặt nổi (card, popover, input shell) dùng nền bán trong suốt kết hợp blur `24`. Bo góc mềm mại từ `16px` đến `24px` cho các khung chứa lớn, `9999px` (pill) cho nút bấm và chip.

Dành tối đa diện tích cho ảnh kết quả ở trung tâm. Dùng khoảng trắng để phân tách các vùng chức năng. Không đặt đường viền hoặc nền cứng giữa các vùng.

Mọi tương tác hover chỉ được thay đổi `background-color` và/hoặc `color` với `transition: all 200ms ease`. Không dùng `translateY`, `scale`, `transform` hay bất kỳ dịch chuyển hình học nào.

---

## Tham khảo Token hệ thống

Xem [design_tokens.md](file:///d:/src/Arch/BabyDra/docs/design/design_tokens.md) để tra cứu giá trị cụ thể cho: bảng màu, typography, border-radius, spacing, shadow.

---

## Danh mục hướng dẫn Component

| Component | File | Nội dung |
| :--- | :--- | :--- |
| Navbar | `navbar.md` | Cách tạo header: logo, nav pill, nhóm tiện ích, avatar rainbow ring |
| Navs & Tabs | `navs_tabs.md` | Cách tạo center nav pill và session sidebar dọc |
| Prompt Panel | `prompt_panel.md` | Cách tạo panel bên trái: ảnh tham chiếu, keyword chips, văn bản mô tả |
| Preview Panel | `preview_panel.md` | Cách tạo vùng ảnh chính và cột 4 biến thể |
| Input Group | `input_group.md` | Cách tạo khung nhập liệu nổi và toolbar thông số |
| Buttons | `buttons.md` | Cách tạo nút Primary, Share Pill, Upgrade, Icon Button |
| Cards | `card.md` | Cách tạo khung chứa: preview, popover, variation, reference, input shell |
| Badges & Chips | `badge.md` | Cách tạo badge PRO, keyword chip, floating icon badge |
| Dropdowns | `dropdowns.md` | Cách tạo menu popover: profile header, menu rows, badge PRO, checkmark |
| Progress | `progress.md` | Cách tạo vòng credit meter |
| Spinners | `spinners.md` | Cách tạo skeleton pulse và button spinner |
| Tooltips | `tooltips.md` | Cách tạo tooltip |

---

## Sơ đồ Bố cục

```text
+-----------------------------------------------------------------------------------+
|  [Logo]                 [Nav Pill: 5 icons]               [Share+] [Credit] [Avt] |  <- Header
+-----------------------------------------------------------------------------------+
|               |                                        | [Variations] | [Session] |
| [Prompt       |  [Main Preview]                        |  [Var 1*]    |    (+)    |
|  Panel]       |   Ảnh kết quả trung tâm               |  [Var 2 ]    |   (Avt)   |
| Ref Image     |   border-radius: 24px                  |  [Var 3 ]    |   (Avt)   |
| Chips         |   shadow hệ thống                      |  [Var 4 ]    |   (Avt)   |
| Narrative     |                                        |              |           |
+-----------------------------------------------------------------------------------+
|              [Input Floating Shell: Prompt + Generate + Toolbar]                   |  <- Fixed bottom
+-----------------------------------------------------------------------------------+
```
