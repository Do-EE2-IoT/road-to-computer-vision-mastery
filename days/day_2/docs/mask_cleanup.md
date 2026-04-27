# Mask Cleanup (Blur + Morphology)

## 1) Mask cleanup là gì?

Mask cleanup là bước làm sạch **binary mask** (`0/255`) sau color threshold.

Mục tiêu:
- Giảm noise rời rạc
- Lấp lỗ trong object
- Làm biên ổn định để contour/bbox/đếm object chính xác hơn

---

## 2) Vì sao cần cleanup?

Raw mask từ threshold thường bị:
- White noise lẻ tẻ (false positive)
- Lỗ đen trong object (false negative)
- Biên răng cưa
- Blob nhỏ không liên quan

Nếu bỏ qua cleanup, các bước sau (connected components, contour, tracking) dễ sai hoặc nhảy frame.

---

## 3) Hai lớp xử lý chính

## 3.1. Pre-filter trước threshold (trên ảnh màu)

- `Gaussian blur`: nhanh, giảm nhiễu ngẫu nhiên
- `Median blur`: tốt với salt-pepper
- `Bilateral`: giữ biên tốt, nhưng nặng

## 3.2. Morphology sau threshold (trên binary mask)

- `Erode`: co vùng trắng, xóa nhiễu trắng nhỏ
- `Dilate`: nở vùng trắng, nối vùng gần nhau
- `Opening = Erode -> Dilate`: dọn nhiễu trắng nhỏ
- `Closing = Dilate -> Erode`: lấp lỗ đen nhỏ

---

## 4) Ý nghĩa toán học ngắn gọn

Gọi mask là `M`, kernel là `K`:
- Erode giữ pixel trắng khi neighborhood theo `K` đều trắng
- Dilate giữ pixel trắng nếu có ít nhất 1 pixel trắng trong neighborhood

Intuition:
- Erode: "strict"
- Dilate: "lenient"

---

## 5) Kernel và iteration

- Kernel `3x3`: cleanup nhẹ, giữ chi tiết tốt
- Kernel `5x5`+: cleanup mạnh, dễ mất chi tiết nhỏ
- Iteration nhiều: tăng độ mạnh tương đương kernel lớn hơn

Khuyến nghị bắt đầu:
1. Kernel `3x3`
2. Iteration = 1
3. Tăng dần có kiểm soát

---

## 6) Pipeline gợi ý cho `color_filtering.rs`

```text
RGB -> HSV -> threshold -> raw_mask -> opening -> closing -> final_mask
```

Nếu bị mất object nhỏ:
- giảm kernel
- giảm iteration
- thử chỉ `closing`

Nếu còn nhiều nhiễu:
- tăng opening
- thêm pre-blur trước threshold

---

## 7) Ví dụ ma trận nhanh (3x3 concept)

## 7.1. Opening xóa noise trắng lẻ

Input:
```text
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 1 0 0
```

Sau `opening`:
```text
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 0 0 0
```

## 7.2. Closing lấp lỗ đen

Input:
```text
0 0 0 0 0
0 1 1 1 0
0 1 0 1 0
0 1 1 1 0
0 0 0 0 0
```

Sau `closing`:
```text
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 0 0 0
```

---

## 8) Đo hiệu quả cleanup như thế nào?

Nên log trước/sau cleanup:
- Số pixel trắng
- Số connected components
- Tổng diện tích blob hợp lệ
- IoU mask (nếu có ground truth)

Trong video:
- Độ ổn định bbox theo frame

---

## 9) Lỗi phổ biến khi tự code

- Ghi đè input khi đang quét kernel (nên output ra buffer mới)
- Không xử lý boundary
- Kernel quá lớn làm mất object mỏng
- Tune chỉ trên 1 ảnh, fail khi đổi ánh sáng

---

## 10) Checklist triển khai

- [ ] Có hiển thị `raw_mask` và `clean_mask`
- [ ] Có ít nhất `opening + closing`
- [ ] Có log pixel count trước/sau
- [ ] Test trên nhiều điều kiện sáng/tối
- [ ] Tuning theo dữ liệu thực, không hardcode cảm tính
