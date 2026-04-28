# Threshold and Masking

## 1) Range-based thresholding

A pixel is kept if it satisfies:
- `H in [h_low, h_high]`
- `S in [s_low, s_high]`
- `V in [v_low, v_high]`

Binary mask rule:
- condition true -> `255`
- condition false -> `0`

---

## 2) Multi-range (important)

Some colors require multiple ranges.
Example for red in HSV:
- `H in [0, 12]` or `H in [345, 360]`

Final mask = OR of sub-masks.

---

## 3) Grayscale vs color masks

- Grayscale threshold: strong for brightness/shape cues
- Color threshold: strong for semantic color segmentation
- Many practical pipelines combine both

---

## 4) Threshold tuning process

1. Collect samples across multiple conditions
2. Define ROI for target color
3. Inspect H/S/V histograms
4. Set initial ranges
5. Tune with metrics (precision/recall/IoU)

---

## 5) Practical tips

- Set a minimum `S` to avoid gray/white noise
- Use `V` threshold to reject very dark regions
- Never tune on one image only
