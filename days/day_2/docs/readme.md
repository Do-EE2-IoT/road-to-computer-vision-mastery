# Day 2 - Color Filtering in Image Processing

> Mục tiêu ngày 2: nắm chắc kỹ thuật lọc màu để tạo mask ổn định trong điều kiện ảnh thực tế (noise, ánh sáng thay đổi, bóng/reflect).

---

## Goal

**Color Filtering Pipeline**

Bạn cần làm chủ luồng chuẩn:
1. Chọn color space phù hợp (`HSV/Lab/YCrCb`)
2. Đặt ngưỡng màu đúng (`threshold/range`)
3. Làm sạch mask (`blur + morphology`)
4. Đánh giá chất lượng (`precision/recall/IoU của mask`)

---

## Core (Quan trọng nhất - học trước)

1. Color thresholding theo range (`inRange` logic)
2. Multi-range filtering (ví dụ màu đỏ có 2 dải Hue)
3. Channel-wise filtering (lọc theo từng kênh)
4. Color masking và hậu xử lý mask

**Nếu thiếu thời gian, tập trung 3 thứ này trước:**
- Chọn đúng color space
- Chọn đúng range
- Làm sạch mask để giảm false positive

---

## Practical (Thực hành bắt buộc)

- So sánh threshold giữa `RGB` vs `HSV`
- Lọc đỏ/xanh/vàng trong 3 điều kiện ánh sáng khác nhau
- Tạo binary mask + overlay ảnh gốc
- Dùng erosion/dilation/opening/closing để giảm noise
- Tuning threshold có log rõ ràng (không tuning cảm tính)

---

## System Notes (Yếu tố thực tế ảnh hưởng mạnh)

- Lighting conditions
- Camera noise
- Shadow / reflection
- White balance drift
- Sensor color response khác nhau giữa thiết bị

---

## Bonus (Nâng cao)

- Color normalization
- White balance correction
- Color constancy
- Color clustering (K-means, GMM)

---

## Suggested Completion Checklist

- [ ] Tách được object theo màu với mask rõ ràng trong ít nhất 3 điều kiện ánh sáng.
- [ ] Có pipeline `preprocess -> threshold -> morphology -> evaluation`.
- [ ] Viết được guideline chọn threshold cho từng màu chính.
- [ ] Giảm false positive bằng ít nhất 2 kỹ thuật khác nhau.

---

## Docs Index

- `color_filtering_core.md`: nền tảng lọc màu và chọn color space
- `threshold_and_masking.md`: threshold, multi-range, logic tạo mask
- `mask_cleanup.md`: blur + morphology + hậu xử lý mask
- `robustness.md`: chống nhiễu ánh sáng, reflection, white balance
