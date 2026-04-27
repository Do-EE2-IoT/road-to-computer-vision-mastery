# Mask Cleanup (Blur + Morphology)

## 1) Cleanup mask là gì?

**Mask cleanup** là bước làm sạch ảnh nhị phân sau threshold màu.
Mục tiêu: biến mask thô (nhiễu, lỗ, biên xấu) thành mask ổn định để contour/detect/count chính xác hơn.

Đầu vào:
- Binary mask (`0`/`255`) từ color threshold

Đầu ra:
- Binary mask sạch hơn, ít false positive/false negative hơn

---

## 2) Tại sao phải cleanup?

Mask thô thường gặp các lỗi sau:
- Nhiễu hạt rời rạc (isolated white pixels)
- Vùng object bị thủng lỗ
- Biên răng cưa và đứt đoạn
- Nhiều blob nhỏ không liên quan

Nếu bỏ qua cleanup:
- contour bị sai
- bounding box nhảy
- đo diện tích/đếm object không ổn định

---

## 3) Cấu phần cleanup gồm những gì?

## 3.1. Pre-filter trước threshold (trên ảnh màu)

Dùng để giảm nhiễu trước khi tạo mask:
- **Gaussian blur**: nhanh, tốt cho nhiễu ngẫu nhiên
- **Median blur**: tốt cho salt-pepper
- **Bilateral filter**: giữ biên tốt hơn nhưng chậm

## 3.2. Morphology sau threshold (trên mask nhị phân)

Các phép cơ bản:
- **Erode**: co vùng trắng, xóa điểm nhiễu nhỏ
- **Dilate**: nở vùng trắng, nối vùng gần nhau
- **Opening = Erode -> Dilate**: dọn nhiễu trắng nhỏ
- **Closing = Dilate -> Erode**: lấp lỗ đen nhỏ trong object

---

## 4) Bản chất toán học ngắn gọn

Gọi mask nhị phân là `M`, kernel là `K`:
- Erode giữ pixel trắng khi vùng lân cận theo `K` đều thỏa điều kiện trắng.
- Dilate giữ pixel trắng nếu tồn tại ít nhất một pixel trắng trong lân cận theo `K`.

Intuition:
- Erode = "strict" (lọc nhiễu)
- Dilate = "lenient" (bù vùng)

---

## 5) Kernel và iteration ảnh hưởng ra sao?

- Kernel nhỏ (`3x3`): giữ chi tiết, cleanup nhẹ
- Kernel lớn (`5x5`, `7x7`): cleanup mạnh, dễ mất chi tiết nhỏ
- Nhiều iteration: hiệu ứng giống tăng độ mạnh phép toán

Quy tắc thực hành:
1. Bắt đầu `3x3`, 1 iteration
2. Tăng dần nếu còn nhiễu
3. Mỗi lần tăng phải kiểm tra object nhỏ có bị mất không

---

## 6) Pipeline gợi ý để bạn code trong `color_filtering.rs`

Pipeline cơ bản:

`RGB -> HSV -> threshold -> raw_mask -> opening -> closing -> final_mask`

Nếu vùng detect quá nhỏ/đứt:
- thử `closing` trước rồi `opening`

Nếu nhiễu quá nhiều:
- tăng blur nhẹ trước threshold
- tăng kernel opening

---

## 7) Nên đo gì để biết cleanup tốt hơn?

- Số lượng connected components (nên giảm blob nhiễu)
- Tỉ lệ diện tích vùng quan tâm trước/sau cleanup
- IoU với ground truth (nếu có nhãn)
- Độ ổn định bbox qua nhiều frame

---

## 8) Lỗi thường gặp khi tự code cleanup

- Quên clone input trước khi ghi output (ghi đè sai)
- Dùng index biên không xử lý boundary
- Kernel quá lớn làm mất object thật
- Cleanup tốt trên 1 ảnh nhưng fail khi đổi ánh sáng

---

## 9) Checklist triển khai nhanh

- [ ] Có raw mask để so sánh trước/sau
- [ ] Có ít nhất opening + closing
- [ ] Tuning kernel theo dữ liệu thật, không hardcode cảm tính
- [ ] Có log số pixel trắng/contour count để đánh giá
