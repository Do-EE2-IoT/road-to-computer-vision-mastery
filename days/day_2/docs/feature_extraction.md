# Feature Extraction - Bounding Box on Blue Ball

## 1) Objective

Build a practical pipeline to extract a blue object and draw a **red bounding box** around it.

This day focuses on:
- Color-based mask extraction (HSV)
- Mask cleanup (opening + closing)
- Connected-component based feature extraction
- Bounding box drawing on RGB image

---

## 2) Why feature extraction matters

Raw image = many pixels, but model/system needs compact meaningful signals.

From mask, we can extract:
- Area
- Bounding box
- Centroid
- Perimeter
- Circularity

For this exercise, priority is:
1. Area (object size)
2. Bounding box (location + size)

---

## 3) Pipeline used in `examples/feature_extraction.rs`

```text
RGB image
-> HSV threshold (blue)
-> Raw mask
-> Morphology cleanup (opening + closing)
-> Largest connected component
-> Bounding box (x_min, y_min, x_max, y_max)
-> Draw red rectangle on original image
```

---

## 4) Core concepts

## 4.1. Bounding Box

Axis-aligned rectangle around object.

Given object pixels, define:
- `x_min = min(x)`
- `y_min = min(y)`
- `x_max = max(x)`
- `y_max = max(y)`

Width/Height:
- `w = x_max - x_min + 1`
- `h = y_max - y_min + 1`

## 4.2. Largest connected component

After cleanup, there may still be multiple blobs.
To target the main ball, select the component with largest area.

---

## 5) Why cleanup before bbox?

Without cleanup:
- noisy pixels can create wrong bbox
- small random blobs may be selected

Opening removes small white noise.
Closing fills small holes in object.
Result: more stable bbox.

---

## 6) Run the example

From workspace root:

```bash
cargo run -p day_2 --example feature_extraction --release
```

Expected windows:
- `Original RGB`
- `Blue Mask (Clean)`
- `Bounding Box (Red)`

Terminal output includes bbox coordinates and detected area.

---

## 7) Tuning tips

If ball is not detected:
- widen blue hue range
- lower S/V thresholds slightly

If bbox includes too much background:
- tighten hue range
- increase opening strength slightly

If object is fragmented:
- add or strengthen closing

---

## 8) Next recommended features

After bbox works, add:
- centroid (`cx`, `cy`)
- bounding box aspect ratio
- circularity (`4πA / P²`) to verify object is near circular
