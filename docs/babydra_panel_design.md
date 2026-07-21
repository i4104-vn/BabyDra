# Hướng dẫn Phong cách Thiết kế Giao diện BabyDra (UI/UX Design System)

Tài liệu này định nghĩa các quy chuẩn thiết kế giao diện người dùng (UI), phong cách mỹ thuật, bảng màu sắc thống nhất và trải nghiệm chuyển động (UX) đặc trưng của hệ điều hành **BabyDra**. Đây là bộ tiêu chuẩn trực quan áp dụng đồng bộ cho tất cả các thành phần đồ họa trong hệ thống.

---

## 1. Phong cách Mỹ thuật Chủ đạo & Hai Trạng thái Nền (Visual Theme)

Hệ thống giao diện áp dụng ngôn ngữ thiết kế **Glassmorphic** (kính mờ) hiện đại, hỗ trợ đồng thời hai chế độ nền sáng và tối với cùng một tông màu nhấn chủ đạo.

- **Hiệu ứng Kính mờ (Acrylic Glassmorphism):**
  - **Làm mờ nền (Background Blur):** Sử dụng các lớp nền bán trong suốt kết hợp với hiệu ứng làm mờ nền phía sau ở mức độ cao (`-gtk-background-blur: 24`).
  - **Góc bo tròn (Border Radius):** Thống nhất các góc bo tròn mềm mại cho toàn bộ các cửa sổ popup và bảng điều khiển ở mức `20px`z để tạo vẻ ngoài thân thiện, mượt mà.

### 1.1. Chi tiết Hai Chế độ Nền (Light/Dark Schemes)

| Thành phần Thiết kế        | Chế độ Tối (Dark Scheme)                                        | Chế độ Sáng (Light Scheme)                              |
| :------------------------- | :-------------------------------------------------------------- | :------------------------------------------------------ |
| **Màu nền cơ bản**         | Nền Acrylic tối đục (`rgba(14, 14, 18, 0.96)`)                  | Nền Acrylic sáng ấm (`rgba(255, 255, 255, 0.98)`)       |
| **Độ mờ nền phía sau**     | `-gtk-background-blur: 24`                                      | `-gtk-background-blur: 24`                              |
| **Đường viền chung**       | Viền mỏng tối mờ (`rgba(255, 255, 255, 0.14)`)                  | Viền mỏng sáng mờ (`rgba(0, 0, 0, 0.08)`)               |
| **Cạnh viền trên (Bevel)** | Viền phản quang sáng (`rgba(255, 255, 255, 0.28)`)              | Viền chìm bóng nhẹ (`rgba(0, 0, 0, 0.06)`)              |
| **Màu chữ & Icon chính**   | Trắng sáng tinh khiết (`#ffffff` / `rgba(255, 255, 255, 0.95)`) | Tối sẫm hiện đại (`#1c1c1e` / `rgba(28, 28, 30, 0.95)`) |
| **Màu chữ & Icon phụ**     | Trắng mờ nhẹ (`rgba(255, 255, 255, 0.50)`)                      | Xám mờ nhẹ (`rgba(28, 28, 30, 0.50)`)                   |

---

## 2. Bảng Màu sắc Thống nhất (Unified Color Palette)

Để đảm bảo tính nhất quán thị giác và tránh sự lộn xộn, hệ thống áp dụng một bảng màu được chuẩn hóa với một tông màu nhấn chủ đạo duy nhất:

- **Màu nhấn hệ thống (Accent Color):** Sử dụng duy nhất màu **Xanh dương Neon (`#3b82f6`)** cho tất cả các chỉ báo trạng thái hoạt động, điểm nhấn thị giác, thanh tiến trình và nút bấm ở trạng thái được kích hoạt trên cả chế độ sáng và tối.
- **Màu nhấn phụ (Trạng thái thành công):** Sử dụng màu xanh lá cây dịu (`#10b981`) cho các tiến trình hoàn tất hoặc chỉ báo dung lượng lưu trữ an toàn.

---

## 3. Quy chuẩn Thiết kế các Thành phần Giao diện (Component Standards)

> [!IMPORTANT]
> **QUY TẮC PHẢN HỒI HOVER QUAN TRỌNG:**
> Giao diện của BabyDra hoàn toàn loại bỏ các hiệu ứng nhô lên hoặc dịch chuyển vật lý (như `translateY`, phóng to `scale` của nút hay nút trượt) khi rê chuột qua (hover). Mọi phản hồi tương tác hover chỉ được thực hiện thông qua việc **thay đổi màu sắc (Color Transition)** một cách mượt mà để giữ cho giao diện luôn phẳng, tinh tế và ổn định.

### 3.1. Các nút điều khiển & Quick Tiles

Toàn bộ các nút chức năng tương tác dạng toggle đều tuân thủ một quy chuẩn hiển thị duy nhất:

- **Trạng thái bình thường (Inactive/Normal):**
  - _Trong Dark Mode:_ Nền mờ tối (`rgba(255, 255, 255, 0.04)`) và viền mỏng. Hover chỉ chuyển màu nền sáng lên (`rgba(255, 255, 255, 0.08)`).
  - _Trong Light Mode:_ Nền mờ sáng (`rgba(0, 0, 0, 0.05)`) và viền mỏng. Hover chỉ chuyển màu nền đậm hơn (`rgba(0, 0, 0, 0.08)`).
  - Icon và văn bản có độ mờ nhẹ để biểu thị trạng thái không hoạt động.
- **Trạng thái kích hoạt (Active):**
  - Nền chuyển hoàn toàn sang màu xanh nhấn **`#3b82f6`** (hoặc chuyển sang màu xám/đen nhẹ làm chìm trong chế độ Light tùy cấu hình).
  - Icon và văn bản chuyển sang màu nổi bật tương ứng.
  - **Hiệu ứng Hover:** Khi di chuột qua ở trạng thái active, nút chỉ thay đổi màu nền chuyển sang màu xanh thẫm hơn (`#2563eb`) để phản hồi hành động.

---

### 3.2. Thiết kế Slider điều khiển (Volume & Brightness Sliders)

Thanh trượt điều khiển thông số (âm lượng, độ sáng) áp dụng kiểu thiết kế dạng thẻ slider thông minh:

- **Rãnh trượt mập mạp (Scale Trough):** Thiết kế rãnh kéo của thanh trượt dày dặn (chiều cao `24px`, bo tròn góc `12px`), nền mờ tối (`rgba(255, 255, 255, 0.10)` trong Dark hoặc `rgba(0, 0, 0, 0.06)` trong Light). Phần đã trượt qua được tô màu xanh nhấn `#3b82f6`.
- **Nút trượt ẩn (Slider Knob):** Chấm tròn trượt màu trắng tinh đường kính `12px`, nằm lọt hoàn toàn bên trong rãnh trượt. **Không sử dụng hiệu ứng scale phóng to khi hover**; nút trượt giữ nguyên kích thước để đảm bảo tính ổn định thị giác.
- **Chỉ báo giá trị:** Số phần trăm hiển thị ở góc phải phía trên luôn được tô màu xanh nhấn `#3b82f6`.
- **Nút Mute tích hợp:** Nút tắt âm (`slider-overlay-mute-btn`) được đặt nằm đè trực tiếp lên phần đầu bên trái của rãnh trượt để tối giản diện tích hiển thị.

---

### 3.3. Biểu đồ & Chỉ báo Tiến trình (Charts & Progress Indicators)

- **Biểu đồ Lịch sử:** Sử dụng nét vẽ mịn (line width = 2px) màu xanh nhấn `#3b82f6`. Vùng phía dưới đường biểu đồ được đổ màu xanh mờ nhạt (`rgba(59, 130, 246, 0.15)`) để tạo chiều sâu đồ họa. Các đường lưới ngang được làm cực mảnh và mờ đục.
- **Chỉ báo Tiến trình tròn:** Sử dụng cung tròn màu xanh nhấn `#3b82f6` chạy mịn từ 0% đến 100% trên rãnh tròn màu trắng đục nhẹ (`rgba(255, 255, 255, 0.06)`).
- **Chồng lớp Đa tầng (3D Stacked Layers):** Đối với các danh sách thông báo hoặc thẻ thông tin được thu gọn, hệ thống vẽ thêm 2 viền giả lập xếp lớp thụt dần về phía dưới ở chân thẻ để tạo hiệu ứng chiều sâu 3D của một chồng thẻ giấy chân thực.

---

## 4. Quy chuẩn Hiệu ứng Chuyển động (Animations System)

Mọi chuyển động của giao diện trên hệ thống đều phải diễn ra mượt mà, tự nhiên và phản hồi tức thì với hành vi người dùng:

- **Genie Animation (Hiệu ứng co giãn đèn thần):** Áp dụng khi đóng/mở các bảng điều khiển chính. Cửa sổ sẽ co giãn động và biến đổi kích thước hướng về phía nút bấm kích hoạt ban đầu trong vòng `450ms` thay vì phóng to thô cứng từ tâm màn hình.
- **Slide Animation (Trượt trơn tru):** Áp dụng cho các Popover thả xuống hoặc danh sách con khi mở rộng/thu gọn. Chuyển động trượt diễn ra theo hướng lên/xuống trong vòng `400ms` đến `450ms` với gia tốc mượt mà.
- **Transition Effect:** Tất cả nút bấm, icon trạng thái, và các điểm chấm hiển thị đều có thuộc tính transition `all 200ms ease` để phản hồi hover/active trơn tru nhất bằng sự chuyển đổi màu sắc (Color Transition).
