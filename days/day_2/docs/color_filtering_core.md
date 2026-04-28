# Color Filtering Core

## 1) What is color filtering?

Color filtering keeps pixels within a target color range and removes the rest.
Typical outputs:
- Binary mask (`0/255`)
- Highlighted image of selected regions

---

## 2) Color-space choice is the key decision

- `RGB`: intuitive for display, but sensitive to illumination
- `HSV`: separates hue from brightness, strong default for color thresholding
- `Lab`: useful when perceptual color consistency matters
- `YCrCb`: useful in some skin/broadcast workflows

Quick rule:
- Start with `HSV` for color segmentation
- If lighting is difficult, evaluate `Lab` or normalization

---

## 3) Core techniques

- Single-range threshold
- Multi-range threshold (for wrap-around colors like red)
- Channel-wise constraints (`H`, `S`, `V` ranges)
- Mask refinement

---

## 4) Common mistakes

- Using Hue alone without S/V constraints
- Reusing one threshold set across all cameras
- Not testing across multiple lighting conditions
- Skipping mask post-processing
