# Mask Cleanup (Blur + Morphology)

## 1) Vì sao phải cleanup mask?

Mask từ threshold thường bị:
- lỗ nhỏ trong object
- nhiễu hạt lẻ tẻ
- biên răng cưa

Cleanup giúp contour ổn định hơn cho bước detect/measure.

---

## 2) Pre-filter trước threshold

- Gaussian blur: giảm nhiễu ngẫu nhiên
- Median blur: tốt với salt-pepper
- Bilateral: giữ biên tốt hơn nhưng chậm hơn

---

## 3) Morphology sau threshold

- Erode: loại nhiễu nhỏ
- Dilate: mở rộng vùng hợp lệ
- Opening (erode -> dilate): dọn noise
- Closing (dilate -> erode): lấp lỗ nhỏ

---

## 4) Chọn kernel hợp lý

- Kernel lớn: mask mượt hơn nhưng dễ mất chi tiết
- Kernel nhỏ: giữ chi tiết nhưng còn noise

Quy tắc nhanh:
- Bắt đầu `3x3`
- Nếu nhiễu cao tăng `5x5`
- Kiểm tra contour bị vỡ hay không

---

## 5) Chuỗi pipeline gợi ý

`blur -> HSV threshold -> opening -> closing -> connected components`
