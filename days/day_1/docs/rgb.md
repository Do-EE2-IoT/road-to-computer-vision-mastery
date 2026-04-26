# RGB Notes - Raw Data to RGB Image

## 1) RGB là gì?

RGB biểu diễn màu bằng 3 kênh:
- `R` (Red)
- `G` (Green)
- `B` (Blue)

Mỗi pixel thường là 3 giá trị cường độ màu. Với ảnh 8-bit/kênh, mỗi kênh nằm trong `0..255`.

---

## 2) Các loại RGB thường gặp

## 2.1. RGB24 (RGB888)

- Mỗi pixel: 3 bytes theo thứ tự `R, G, B`
- Dung lượng: `width * height * 3`
- Dễ dùng trong xử lý ảnh, debug trực quan

Ví dụ 2 pixel:
- Pixel 0 = đỏ `(255,0,0)`
- Pixel 1 = xanh lá `(0,255,0)`
- Raw bytes: `[255,0,0, 0,255,0]`

## 2.2. BGR24

- Mỗi pixel: `B, G, R`
- OpenCV hay gặp BGR mặc định
- Nếu đọc nhầm là RGB, màu sẽ sai (đỏ thành xanh, ...)

## 2.3. RGBA32 / BGRA32 / ARGB32

- 4 bytes/pixel, có thêm alpha
- Alpha dùng cho độ trong suốt/compositing

## 2.4. RGB565 (16-bit packed)

- Mỗi pixel 16-bit:
  - `R`: 5 bits
  - `G`: 6 bits
  - `B`: 5 bits
- Dùng nhiều ở embedded/display vì tiết kiệm băng thông
- Chất lượng màu thấp hơn RGB888

## 2.5. Planar RGB

- Dữ liệu tách thành 3 plane riêng: `R-plane`, `G-plane`, `B-plane`
- Một số pipeline ML/ISP dùng kiểu này để vectorize tốt hơn

---

## 3) Raw image data trông như nào?

Raw ở đây là byte buffer chưa có metadata ảnh chuẩn (không phải PNG/JPEG có header).

Bạn cần biết chính xác:
- `width`
- `height`
- `pixel format` (RGB24/BGR24/RGB565/...)
- `stride` (bytes mỗi dòng, có thể lớn hơn `width * bytes_per_pixel`)
- `endianness` (quan trọng với packed 16-bit như RGB565)

Nếu sai bất kỳ mục nào, ảnh decode sẽ lệch màu hoặc vỡ hình.

---

## 4) Từ raw -> RGB làm như nào?

Quy trình tổng quát:

1. Xác định format gốc và layout memory.
2. Nếu có `stride/padding`, copy từng dòng về buffer liên tục.
3. Decode từng pixel sang `R,G,B` 8-bit.
4. Tạo `RgbImage`/tensor chuẩn để xử lý tiếp.

---

## 5) Decode ví dụ phổ biến

## 5.1. RGB24 -> RGB

Mỗi 3 byte là 1 pixel:
- `r = raw[i]`
- `g = raw[i+1]`
- `b = raw[i+2]`

## 5.2. BGR24 -> RGB

Mỗi 3 byte là `b,g,r`, cần đảo:
- `r = raw[i+2]`
- `g = raw[i+1]`
- `b = raw[i]`

## 5.3. RGB565 -> RGB888

Giả sử 16-bit value là `p`:
- `r5 = (p >> 11) & 0x1F`
- `g6 = (p >> 5)  & 0x3F`
- `b5 = p & 0x1F`

Scale lên 8-bit:
- `r8 = (r5 * 255) / 31`
- `g8 = (g6 * 255) / 63`
- `b8 = (b5 * 255) / 31`

---

## 6) Khi nào dùng format nào?

- `RGB24/RGBA32`: xử lý ảnh tổng quát, debug dễ.
- `BGR24`: giao tiếp với OpenCV hoặc nguồn camera/SDK theo BGR.
- `RGB565`: thiết bị nhúng, màn hình, truyền dữ liệu nhẹ.
- `Planar RGB`: một số mô hình hoặc pipeline tối ưu SIMD/NPU.

---

## 7) Pitfalls rất hay gặp

- Nhầm RGB với BGR
- Quên stride/padding dòng
- Sai width/height metadata
- Sai endianness khi đọc RGB565
- Không clamp/scale đúng khi đổi bit-depth

---

## 8) Liên kết với example `examples/rgb.rs`

Example này minh họa trực tiếp:
- RGB24 raw round-trip
- BGR24 raw -> RGB đúng thứ tự kênh
- RGB565 pack/unpack để thấy mất mát màu
- So sánh MAE sau mỗi bước

Run:

```bash
cargo run -p day_1 --example rgb --release
```
