# Threshold and Masking

## 1) Threshold theo range

Một pixel được giữ lại nếu thỏa điều kiện:
- `H in [h_low, h_high]`
- `S in [s_low, s_high]`
- `V in [v_low, v_high]`

Mask nhị phân:
- Thỏa điều kiện -> `255`
- Không thỏa -> `0`

---

## 2) Multi-range (quan trọng)

Một số màu cần nhiều khoảng.
Ví dụ màu đỏ trong HSV:
- `H in [0, 12]` hoặc `H in [345, 360]`

Mask cuối = OR của các mask thành phần.

---

## 3) Grayscale vs Color mask

- Grayscale threshold: tốt cho độ sáng/shape
- Color threshold: tốt cho semantic theo màu
- Nhiều bài toán nên kết hợp cả 2 để giảm nhiễu

---

## 4) Quy trình tuning threshold

1. Thu mẫu ảnh đa điều kiện
2. Chọn ROI chuẩn cho màu mục tiêu
3. Thống kê histogram H/S/V
4. Đặt range ban đầu
5. Tuning bằng metric (precision/recall/IoU mask)

---

## 5) Mẹo thực chiến

- Luôn khóa range `S` tối thiểu để tránh pixel xám/trắng nhiễu
- Dùng `V` để loại vùng quá tối
- Đừng tune bằng 1 ảnh duy nhất
