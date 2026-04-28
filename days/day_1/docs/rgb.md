# RGB Notes - Raw Data to RGB Image

## 1) What is RGB?

RGB represents color with 3 channels:
- `R` (Red)
- `G` (Green)
- `B` (Blue)

Each pixel usually has 3 color intensity values. For 8-bit/channel images, each channel is in `0..255`.

---

## 2) Common RGB Formats

### 2.1. RGB24 (RGB888)
- Per pixel: 3 bytes in `R, G, B` order
- Memory: `width * height * 3`
- Easy for processing and debugging

Example (2 pixels):
- Pixel 0 = red `(255,0,0)`
- Pixel 1 = green `(0,255,0)`
- Raw bytes: `[255,0,0, 0,255,0]`

### 2.2. BGR24
- Per pixel: `B, G, R`
- Common as OpenCV default
- If interpreted as RGB by mistake, colors are swapped

### 2.3. RGBA32 / BGRA32 / ARGB32
- 4 bytes/pixel with alpha channel
- Alpha is used for transparency/compositing

### 2.4. RGB565 (16-bit packed)
- Per pixel 16-bit:
  - `R`: 5 bits
  - `G`: 6 bits
  - `B`: 5 bits
- Common in embedded/display systems due to bandwidth savings
- Lower color quality than RGB888

### 2.5. Planar RGB
- Data is split into separate planes: `R-plane`, `G-plane`, `B-plane`
- Common in some ML/ISP pipelines for vectorization

#### Quick comparison with interleaved RGB
- **Interleaved RGB (HWC)**: `RGB RGB RGB ...`
- **Planar RGB (CHW)**: `RRR... GGG... BBB...`

Interleaved layout:
```text
[R G B][R G B][R G B]...
```

Example image `2 x 1`:
- `P1 = (10, 20, 30)`
- `P2 = (40, 50, 60)`

Memory:
```text
10 20 30   40 50 60
 R  G  B    R  G  B
```

Index for pixel `i`:
- `base = i * 3`
- `R = raw[base + 0]`
- `G = raw[base + 1]`
- `B = raw[base + 2]`

Planar layout:
```text
[R R R ...][G G G ...][B B B ...]
```

Same example:
```text
10 40   20 50   30 60
 R  R    G  G    B  B
```

With `plane_size = W * H`:
- `R[i] = raw[i]`
- `G[i] = raw[plane_size + i]`
- `B[i] = raw[2 * plane_size + i]`

#### Why planar is often SIMD-friendly

Each channel is contiguous in memory, so CPU/GPU can load vectors sequentially.

Example: doubling brightness of channel R.
- **Interleaved**: stride-3 access (`R` at `0, 3, 6, ...`)
- **Planar**: contiguous access over `R-plane[0..plane_size]`

---

## 3) What Raw Image Data Looks Like

Raw here means byte buffers without image file headers (unlike PNG/JPEG).

You must know:
- `width`
- `height`
- `pixel format` (RGB24/BGR24/RGB565/...)
- `stride` (bytes per row, may be larger than `width * bytes_per_pixel`)
- `endianness` (important for packed 16-bit formats like RGB565)

Any mismatch can cause color shifts or broken reconstruction.

---

## 4) Raw -> RGB Conversion Workflow

1. Identify source format and memory layout.
2. If stride/padding exists, repack rows into contiguous buffer.
3. Decode each pixel into 8-bit `R,G,B`.
4. Build a standard `RgbImage`/tensor for next steps.

---

## 5) Common Decode Examples

### 5.1. RGB24 -> RGB
- `r = raw[i]`
- `g = raw[i+1]`
- `b = raw[i+2]`

### 5.2. BGR24 -> RGB
- `r = raw[i+2]`
- `g = raw[i+1]`
- `b = raw[i]`

### 5.3. RGB565 -> RGB888
Given 16-bit value `p`:
- `r5 = (p >> 11) & 0x1F`
- `g6 = (p >> 5)  & 0x3F`
- `b5 = p & 0x1F`

Scale to 8-bit:
- `r8 = (r5 * 255) / 31`
- `g8 = (g6 * 255) / 63`
- `b8 = (b5 * 255) / 31`

---

## 6) When to Use Which Format

- `RGB24/RGBA32`: general image processing and debugging
- `BGR24`: OpenCV interoperability or BGR camera/SDK sources
- `RGB565`: embedded devices, displays, low-bandwidth transport
- `Planar RGB`: SIMD/NPU-optimized model pipelines

---

## 7) Common Pitfalls

- Confusing RGB and BGR
- Ignoring stride/padding
- Wrong width/height metadata
- Wrong RGB565 endianness
- Incorrect clamp/scale during bit-depth conversion

---

## 8) Link to `examples/rgb.rs`

This example demonstrates:
- RGB24 raw round-trip
- BGR24 raw -> RGB channel correction
- RGB565 pack/unpack with visible quality loss
- MAE comparison after each conversion stage

Run:

```bash
cargo run -p day_1 --example rgb --release
```
