# YUV Notes - Formats, Usage, and RGB Conversion

## 1) YUV là gì?

YUV (thực tế hệ video số thường gần với YCbCr) tách ảnh thành:
- **Y**: độ sáng (luma)
- **U/V** (hoặc Cb/Cr): thành phần màu (chroma)

Ý tưởng cốt lõi: mắt người nhạy với chi tiết độ sáng hơn chi tiết màu. Vì vậy có thể giảm độ phân giải chroma để tiết kiệm băng thông mà chất lượng vẫn chấp nhận được.

### YUV giúp giảm memory như thế nào?

Ví dụ block `2x2` (4 pixel):

- RGB24 cần `4 * 3 = 12 bytes`
- YUV420 cần `4Y + 1U + 1V = 6 bytes`

Nghĩa là giảm khoảng **50%** dữ liệu so với RGB24.

Lý do: Y (độ sáng) giữ theo từng pixel, còn U/V (màu) được chia sẻ theo block.
---

## 2) Tại sao camera/video ưa dùng YUV?

- Giảm dung lượng truyền và lưu trữ (đặc biệt với 4:2:0)
- Tối ưu phần cứng ISP, codec, decoder
- Dễ encode video (H.264/H.265/AV1 pipelines đều tận dụng luma/chroma)
- Giữ đủ thông tin độ sáng cho tác vụ CV (edge/shape thường phụ thuộc mạnh vào Y)

---

## 3) Các kiểu subsampling phổ biến

Ký hiệu `4:a:b` mô tả tỉ lệ lấy mẫu chroma theo khối 2x2.

- **4:4:4**: không giảm chroma (chất lượng màu cao, tốn băng thông)
- **4:2:2**: giảm chroma theo chiều ngang
- **4:2:0**: giảm chroma cả ngang và dọc (phổ biến nhất cho camera/video)

Tóm tắt nhanh:
- CV realtime/camera feed: thường gặp `4:2:0`
- Post-production chất lượng cao: có thể dùng `4:4:4`

---

## 4) Các format YUV hay gặp trong system

## Planar vs Semi-planar

- **Planar**: Y, U, V nằm ở các plane riêng
- **Semi-planar**: Y riêng, UV/VU interleaved

## Cụ thể

- **I420 (YUV420p)**
  - Plane thứ tự: `Y` -> `U` -> `V`
  - Mỗi chroma plane có kích thước bằng `1/4` Y

- **YV12**
  - Giống I420 nhưng thứ tự chroma đổi: `Y` -> `V` -> `U`

- **NV12**
  - `Y` plane đầy đủ
  - Plane thứ 2 interleaved `UVUV...`
  - Rất phổ biến trong camera/decoder/hardware acceleration

- **NV21**
  - Giống NV12 nhưng interleaved `VUVU...`

Chú ý: nhầm NV12/NV21 là lỗi rất hay gặp (ảnh bị ám màu mạnh).

---

## 5) Khi nào dùng format nào?

- **Camera capture / mobile / embedded**: thường `NV12` hoặc `NV21`
- **Training data pipeline offline**: thường convert về RGB để debug/augment dễ
- **Realtime CV**: có thể xử lý trực tiếp trên `Y` cho tác vụ edge/motion, chỉ convert RGB khi thật sự cần màu
- **Encode/decode video**: phần lớn internal là YUV 4:2:0

---

## 6) Convert YUV <-> RGB

## 6.1. Điều quan trọng trước khi convert

Phải biết rõ:
- Chuẩn ma trận màu: **BT.601** hay **BT.709**
- Dải giá trị: **Limited range** hay **Full range**

Nếu chọn sai, màu sẽ lệch (washed out hoặc ám màu).

## 6.2. Công thức tham khảo (BT.601, Full range)

Giả sử:
- `Y` trong `[0,255]`
- `U`, `V` trong `[0,255]`
- `u = U - 128`, `v = V - 128`

Khi đó:
- `R = Y + 1.402 * v`
- `G = Y - 0.344136 * u - 0.714136 * v`
- `B = Y + 1.772 * u`

Sau cùng clamp về `[0,255]`.

## 6.3. Công thức tham khảo (BT.601, Limited range)

Thường gặp trong video broadcast:
- `Y` khoảng `[16,235]`
- `U/V` khoảng `[16,240]`

Dạng thường dùng:
- `C = Y - 16`
- `D = U - 128`
- `E = V - 128`
- `R = clip((298*C + 409*E + 128) >> 8)`
- `G = clip((298*C - 100*D - 208*E + 128) >> 8)`
- `B = clip((298*C + 516*D + 128) >> 8)`

---

## 7) NV12 / NV21 -> RGB: index logic cơ bản

Giả sử ảnh kích thước `w x h`:
- Y plane: `w*h` bytes đầu
- UV plane: `w*h/2` bytes sau (interleaved UV)

Pixel `(x,y)`:
- `Y = y_plane[y*w + x]`
- `uv_row = y/2`
- `uv_col = (x/2)*2`
- `U = uv_plane[uv_row*w + uv_col]`
- `V = uv_plane[uv_row*w + uv_col + 1]`

Rồi áp công thức YUV->RGB.

### Với NV21

NV21 giống NV12 nhưng thứ tự chroma đảo lại:

- `V = vu_plane[vu_row*w + vu_col]`
- `U = vu_plane[vu_row*w + vu_col + 1]`

Chỉ khác thứ tự `UV` vs `VU`, nhưng nếu đọc nhầm thì ảnh sẽ ám màu rất rõ.

---

## 8) Rust pseudo-code (NV12/NV21 to RGB)

```rust
fn clamp_u8(v: f32) -> u8 {
    v.max(0.0).min(255.0) as u8
}

fn yuv_to_rgb_bt601_full(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let y = y as f32;
    let u = u as f32 - 128.0;
    let v = v as f32 - 128.0;

    let r = y + 1.402 * v;
    let g = y - 0.344_136 * u - 0.714_136 * v;
    let b = y + 1.772 * u;

    (clamp_u8(r), clamp_u8(g), clamp_u8(b))
}

// NV12: ... U V U V ...
fn nv12_sample_uv(uv_plane: &[u8], idx: usize) -> (u8, u8) {
    (uv_plane[idx], uv_plane[idx + 1]) // (U, V)
}

// NV21: ... V U V U ...
fn nv21_sample_vu(vu_plane: &[u8], idx: usize) -> (u8, u8) {
    (vu_plane[idx + 1], vu_plane[idx]) // return (U, V)
}
```

---

## 9) Lỗi thường gặp trong project thật

- Nhầm `NV12` với `NV21` (UV đảo nhau)
- Dùng sai ma trận (BT.601 vs BT.709)
- Dùng sai range (full vs limited)
- Không clamp sau convert
- Quên xử lý stride/padding của frame camera

---

## 10) Practical guidance cho CV

- Nếu tác vụ chủ yếu là edge/structure, thử dùng riêng kênh **Y** trước để tăng tốc.
- Nếu cần semantics theo màu (đỏ/xanh/vàng), convert chính xác sang RGB/HSV và kiểm range rõ ràng.
- Luôn lưu 1 frame debug và so sánh màu bằng mắt để phát hiện pipeline sai sớm.

---

## 11) Quan hệ với tài liệu Day 1

- `hsv.md`: tập trung color segmentation theo HSV
- `yuv.md` (file này): tập trung format system/camera và chuyển đổi màu

Kết hợp 2 tài liệu này sẽ cover được cả góc nhìn thuật toán và góc nhìn hệ thống.

---

## 12) Liên hệ với `examples/yuv.rs`

Example hiện tại đã có cả 2 luồng:

- `RGB -> NV12 -> RGB`
- `RGB -> NV21 -> RGB`

và in ra MAE để bạn so sánh mức sai khác giữa ảnh gốc và ảnh reconstruct.
