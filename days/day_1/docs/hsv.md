# HSV Notes + `examples/hsv.rs`

## 1) What is HSV?

HSV separates color information into 3 channels:
- **H (Hue)**: color type (red, green, blue...), usually in `0..360` degrees
- **S (Saturation)**: color intensity, usually `0..1`
- **V (Value)**: brightness, usually `0..1`

Main benefit: for color segmentation, HSV is usually more stable than RGB under changing lighting.

---

## 2) RGB -> HSV Formula

Assume an RGB pixel `(R, G, B)` in `0..255`.

### Step 1: Normalize
`r = R / 255`, `g = G / 255`, `b = B / 255`

### Step 2: Intermediate values
- `cmax = max(r, g, b)`
- `cmin = min(r, g, b)`
- `delta = cmax - cmin`

### Step 3: Compute H (Hue)
- If `delta == 0` -> `H = 0`
- If `cmax == r` -> `H = 60 * (((g - b) / delta) mod 6)`
- If `cmax == g` -> `H = 60 * (((b - r) / delta) + 2)`
- If `cmax == b` -> `H = 60 * (((r - g) / delta) + 4)`

If `H < 0`, add `360` to map it into `[0, 360)`.

### Step 4: Compute S (Saturation)
- If `cmax == 0` -> `S = 0`
- Else -> `S = delta / cmax`

### Step 5: Compute V (Value)
`V = cmax`

---

## 3) Channel Mapping for Grayscale Visualization

In `hsv.rs`, channels are mapped as:
- `h_val = (H / 360 * 255) as u8`
- `s_val = (S * 255) as u8`
- `v_val = (V * 255) as u8`

Then saved into `Luma([value])` to display H/S/V as grayscale images.

---

## 4) Red Detection Logic in `hsv.rs`

`is_red(...)` uses 3 combined conditions:

1. **Hue band for red**
   - `(0..=12)` or `(345..=360)`
   - Reason: red is near the Hue wrap-around boundary

2. **Saturation/Value thresholds**
   - `s >= 0.35`
   - `v >= 0.20`
   - Purpose: remove weak-color and very dark pixels

3. **RGB red dominance**
   - `R > G + 15` and `R > B + 15`
   - Purpose: reduce orange/brown false positives

If conditions are satisfied:
- pixel is painted red on highlight image
- pixel is set to white in binary mask

---

## 5) Active Pipeline in `examples/hsv.rs`

1. Read image from `resources/images/red_ball.jpg`
2. Convert each pixel RGB -> HSV
3. Generate grayscale H/S/V images
4. Build `red_mask` (binary)
5. Build `highlighted_img` (red overlay)
6. Display windows:
   - RGB
   - Red Highlight
   - Red Mask
   - Hue
   - Saturation
   - Value

---

## 6) Run

From workspace root:

```bash
cargo run -p day_1 --example hsv --release
```

---

## 7) Fast Tuning Guide

If red is missed:
- widen hue bands (for example `0..15` and `340..360`)
- lower `s` or `v` thresholds slightly

If noise is high:
- increase `s` threshold
- increase red-dominance margin (`+15 -> +20/+25`)
- narrow hue bands

Always test on multiple conditions: bright, low-light, mixed-color backgrounds.

---

## 8) Important Notes

- There is no universal threshold set for all cameras/scenes.
- Detection quality strongly depends on lighting and white balance.
- Always debug with `mask + highlight + H/S/V views`.

---

## 9) Why HSV is Often Preferred

HSV is popular in color-based CV because it separates:
- **Hue**: color identity
- **Value**: brightness

This makes color rules easier to keep stable under lighting changes.

### Relation to edge/shape detection

HSV does not directly detect edges like Sobel/Canny.
But it produces cleaner color masks, which makes contour/edge/shape extraction more stable.

Common pipeline:
1. HSV threshold for color segmentation
2. Morphology for mask cleanup
3. Canny/Contour for shape extraction

### Small practical examples

- **Red ball on noisy background**
  - RGB threshold is often sensitive to shadows.
  - HSV threshold with S/V limits is usually more stable.
  - Better contour -> better radius estimation.

- **Red traffic sign outdoors**
  - Bright noon vs late-afternoon lighting differs a lot.
  - HSV usually keeps red regions more consistent before polygon detection.

- **Color-based counting on conveyor belt**
  - Segment by Hue first.
  - Then use connected components/contours for counting.
  - Often less noisy than direct RGB processing.
