# Color Filtering Core

## 1) Color filtering là gì?

Color filtering là quá trình giữ lại pixel thuộc dải màu mục tiêu và loại phần còn lại.
Đầu ra thường là:
- Binary mask (`0/255`)
- Ảnh highlight vùng màu quan tâm

---

## 2) Chọn color space đúng là bước quyết định

- `RGB`: trực quan, nhưng nhạy ánh sáng
- `HSV`: tách màu (H) khỏi sáng (V), rất hợp threshold màu
- `Lab`: tốt khi cần ổn định cảm nhận màu
- `YCrCb`: hữu ích cho một số bài toán skin-color / broadcast

Quy tắc nhanh:
- Bài toán segmentation theo màu: bắt đầu với `HSV`
- Nếu ánh sáng phức tạp: thử `Lab` hoặc thêm normalize

---

## 3) Kỹ thuật cốt lõi

- Single-range threshold
- Multi-range threshold (màu wrap-around, như đỏ)
- Channel-wise constraints (`H in [a,b]`, `S > s_min`, `V > v_min`)
- Mask refinement

---

## 4) Sai lầm thường gặp

- Chỉ dùng 1 ngưỡng hue, bỏ qua S/V
- Dùng threshold cứng cho mọi camera
- Không test ở điều kiện ánh sáng khác nhau
- Không có bước hậu xử lý mask
