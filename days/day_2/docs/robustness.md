# Robustness for Color Filtering

## 1) Real-world issues

Color filtering often fails because of:
- strong lighting changes
- shadow/reflection
- frame-to-frame auto white balance shifts
- camera-to-camera color response differences

---

## 2) What to prioritize for stability

1. Lock exposure/white balance if possible
2. Normalize inputs before thresholding
3. Use soft ranges and test across scene diversity
4. Add geometric constraints (area/aspect ratio) after masking

---

## 3) Techniques to improve robustness

- White-balance correction
- Color constancy (Gray-World, Shades-of-Gray)
- Histogram matching for domain alignment
- Adaptive thresholding by scene brightness

---

## 4) Evaluate correctly

- Pixel-level Precision/Recall/F1
- IoU on target-color regions
- False-positive analysis by scene type

---

## 5) Pre-release checklist

- [ ] Test indoor + outdoor + low-light
- [ ] Test at least 2 camera/devices
- [ ] Define fallback behavior for over-dark/over-exposed frames
- [ ] Version and log threshold settings for each release
