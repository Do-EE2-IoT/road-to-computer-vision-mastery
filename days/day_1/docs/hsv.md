# HSV Notes + `examples/hsv.rs`

## 1) HSV là gì?

HSV tách thông tin màu theo 3 kênh:
- **H (Hue)**: loại màu (đỏ, xanh lá, xanh dương...), thường trong khoảng `0..360` độ
- **S (Saturation)**: độ đậm nhạt của màu, thường `0..1`
- **V (Value)**: độ sáng, thường `0..1`

Lợi ích chính: khi làm color segmentation, HSV ổn định hơn RGB trong nhiều điều kiện ánh sáng.

---

## 2) Công thức RGB -> HSV

Giả sử pixel RGB là `(R, G, B)` trong khoảng `0..255`.

### Bước 1: Chuẩn hóa

`r = R / 255`, `g = G / 255`, `b = B / 255`

### Bước 2: Giá trị trung gian

- `cmax = max(r, g, b)`
- `cmin = min(r, g, b)`
- `delta = cmax - cmin`

### Bước 3: Tính H (Hue)

- Nếu `delta == 0` -> `H = 0`
- Nếu `cmax == r` -> `H = 60 * (((g - b) / delta) mod 6)`
- Nếu `cmax == g` -> `H = 60 * (((b - r) / delta) + 2)`
- Nếu `cmax == b` -> `H = 60 * (((r - g) / delta) + 4)`

Sau đó nếu `H < 0` thì cộng `360` để đưa về `[0, 360)`.

### Bước 4: Tính S (Saturation)

- Nếu `cmax == 0` -> `S = 0`
- Ngược lại -> `S = delta / cmax`

### Bước 5: Tính V (Value)

`V = cmax`

---

## 3) Mapping kênh để hiển thị ảnh xám

Trong `hsv.rs`, bạn đang map để xem trực quan từng kênh:
- `h_val = (H / 360 * 255) as u8`
- `s_val = (S * 255) as u8`
- `v_val = (V * 255) as u8`

Rồi ghi vào `Luma([value])` để tạo ảnh xám cho H/S/V.

---

## 4) Logic detect đỏ trong `hsv.rs`

Hàm `is_red(...)` hiện dùng 3 điều kiện kết hợp:

1. **Hue band cho màu đỏ**
   - `(0..=12)` hoặc `(345..=360)`
   - Lý do: đỏ nằm ở vùng wrap-around của Hue (gần 0 và gần 360)

2. **Ngưỡng Saturation/Value**
   - `s >= 0.35`
   - `v >= 0.20`
   - Mục tiêu: loại màu quá nhạt hoặc quá tối

3. **Red dominance theo RGB**
   - `R > G + 15` và `R > B + 15`
   - Mục tiêu: giảm false positive với cam/nâu

Nếu thỏa điều kiện, pixel được:
- tô đỏ trong ảnh highlight
- set trắng trong ảnh mask

---

## 5) Pipeline đang chạy trong `examples/hsv.rs`

1. Đọc ảnh từ:
   - `resources/images/red_ball.jpg`
2. Convert từng pixel RGB -> HSV
3. Tạo 3 ảnh xám cho H/S/V
4. Sinh `red_mask` (nhị phân)
5. Sinh `highlighted_img` (overlay điểm đỏ)
6. Hiển thị 6 cửa sổ:
   - RGB
   - Red Highlight
   - Red Mask
   - Hue
   - Saturation
   - Value

---

## 6) Cách chạy

Từ workspace root:

```bash
cargo run -p day_1 --example hsv --release
```

---

## 7) Tuning nhanh khi detect chưa chuẩn

- Bị miss vùng đỏ:
  - Mở rộng hue band, ví dụ `0..15` và `340..360`
  - Giảm `s` hoặc `v` threshold nhẹ

- Bị nhiễu nhiều:
  - Tăng `s` threshold
  - Tăng red-dominance margin (`+15 -> +20/+25`)
  - Thu hẹp hue band

Test trên nhiều điều kiện: sáng mạnh, thiếu sáng, nền nhiều màu.

---

## 8) Ghi chú quan trọng

- Không có một bộ threshold cố định cho mọi ảnh/camera.
- Chất lượng detect phụ thuộc nhiều vào ánh sáng và cân bằng trắng.
- Luôn dùng `mask + highlight + H/S/V views` để debug thay vì đoán.

---

## 9) Vì sao thường ưa chuộng HSV?

HSV được dùng nhiều trong color-based CV vì tách được:
- **Hue**: thông tin màu
- **Value**: độ sáng

Nên khi ánh sáng thay đổi, ta vẫn dễ giữ điều kiện theo màu hơn so với RGB.

### Liên quan tới detect góc cạnh/shape

HSV không trực tiếp “phát hiện cạnh” như Sobel/Canny, nhưng nó giúp tạo **mask sạch hơn** theo màu.
Khi mask sạch, bước detect contour/cạnh/góc sau đó sẽ ổn định hơn.

Pipeline phổ biến:
1. HSV threshold để tách object theo màu
2. Morphology để làm sạch mask
3. Canny/Contour để lấy cạnh và hình dạng

### Ví dụ nhỏ, cụ thể

- **Ví dụ 1: Bóng đỏ trên nền nhiễu**
  - RGB threshold thường nhạy với bóng đổ.
  - HSV threshold theo Hue đỏ + ngưỡng S/V cho ra mask ổn định hơn.
  - Từ mask này, contour của quả bóng rõ hơn để đo bán kính.

- **Ví dụ 2: Biển báo đỏ ngoài trời**
  - Cùng biển báo nhưng sáng trưa và chiều muộn có độ sáng khác nhau.
  - Dùng HSV giúp giữ vùng đỏ tốt hơn, rồi mới tìm polygon/góc của biển báo.

- **Ví dụ 3: Đếm vật thể theo màu trên băng chuyền**
  - Tách từng màu bằng Hue trước.
  - Sau đó dùng connected components/contour để đếm.
  - Ít nhiễu hơn xử lý trực tiếp trên RGB.
