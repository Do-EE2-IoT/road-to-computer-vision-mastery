# YUV Notes - Formats, Usage, and RGB Conversion

## 1) What is YUV?

YUV (in digital video, often close to YCbCr) separates image information into:
- **Y**: luma (brightness)
- **U/V** (or Cb/Cr): chroma (color)

Core idea: the human visual system is more sensitive to luma detail than chroma detail, so chroma can be subsampled to save bandwidth.

### How YUV saves memory

For a `2x2` block (4 pixels):
- RGB24 uses `4 * 3 = 12 bytes`
- YUV420 uses `4Y + 1U + 1V = 6 bytes`

That is about **50%** less data than RGB24.

---

## 2) Why camera/video pipelines prefer YUV

- Lower storage/transmission cost (especially 4:2:0)
- Better fit for ISP/codec/decoder hardware
- Efficient video coding pipelines (H.264/H.265/AV1)
- Keeps luma detail strong for CV tasks (edge/shape)

---

## 3) Common subsampling types

Notation `4:a:b` describes chroma sampling rate.

- **4:4:4**: no chroma reduction
- **4:2:2**: horizontal chroma reduction
- **4:2:0**: horizontal + vertical chroma reduction (most common)

---

## 4) Common YUV formats in systems

### Planar vs Semi-planar
- **Planar**: Y, U, V in separate planes
- **Semi-planar**: Y plane + interleaved chroma plane

### Typical formats
- **I420 (YUV420p)**: `Y -> U -> V`
- **YV12**: `Y -> V -> U`
- **NV12**: `Y` + interleaved `UVUV...`
- **NV21**: `Y` + interleaved `VUVU...`

A very common bug is mixing up NV12 and NV21.

---

## 5) When to use which format

- **Camera/mobile/embedded capture**: often `NV12` or `NV21`
- **Offline training pipelines**: often converted to RGB for augmentation/debug
- **Realtime CV**: often process `Y` directly for structure-heavy tasks
- **Video encode/decode internals**: mostly YUV 4:2:0

---

## 6) YUV <-> RGB conversion

### 6.1. Critical prerequisites

You must know:
- Color matrix: **BT.601** vs **BT.709**
- Value range: **Full range** vs **Limited range**

Wrong assumptions cause visible color shifts.

### 6.2. BT.601 full-range (reference)

Given:
- `Y, U, V in [0,255]`
- `u = U - 128`, `v = V - 128`

Then:
- `R = Y + 1.402 * v`
- `G = Y - 0.344136 * u - 0.714136 * v`
- `B = Y + 1.772 * u`

Clamp final values to `[0,255]`.

### 6.3. BT.601 limited-range (reference)

Common in broadcast/video:
- `Y in [16,235]`
- `U/V in [16,240]`

Typical integer form:
- `C = Y - 16`
- `D = U - 128`
- `E = V - 128`
- `R = clip((298*C + 409*E + 128) >> 8)`
- `G = clip((298*C - 100*D - 208*E + 128) >> 8)`
- `B = clip((298*C + 516*D + 128) >> 8)`

---

## 7) NV12 / NV21 -> RGB indexing logic

For image `w x h`:
- Y plane size: `w*h`
- chroma plane size: `w*h/2`

For pixel `(x,y)`:
- `Y = y_plane[y*w + x]`
- `row = y/2`
- `col = (x/2)*2`

NV12:
- `U = uv_plane[row*w + col]`
- `V = uv_plane[row*w + col + 1]`

NV21:
- `V = vu_plane[row*w + col]`
- `U = vu_plane[row*w + col + 1]`

---

## 8) Rust pseudo-code (NV12/NV21)

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
```

---

## 9) Common project mistakes

- Mixing up `NV12` and `NV21`
- Wrong color matrix (BT.601 vs BT.709)
- Wrong range assumption (full vs limited)
- Missing clamp after conversion
- Ignoring frame stride/padding

---

## 10) Practical CV guidance

- For edge/structure tasks, try processing only channel **Y** first.
- For color semantics, convert carefully to RGB/HSV and validate ranges.
- Save debug frames and visually validate color correctness early.

---

## 11) Relation to Day 1 docs

- `hsv.md`: color segmentation logic
- `yuv.md` (this file): system/camera format and conversion logic

Together they cover algorithm and system viewpoints.

---

## 12) Relation to `examples/yuv.rs`

Current example includes:
- `RGB -> NV12 -> RGB`
- `RGB -> NV21 -> RGB`

It also prints MAE so you can compare reconstruction loss.
