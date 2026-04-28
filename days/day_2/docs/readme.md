# Day 2 - Color Filtering in Image Processing

> Day 2 goal: master color filtering techniques to create stable masks under real-world conditions (noise, lighting changes, shadows/reflections).

---

## Goal

**Color Filtering Pipeline**

You should master this standard flow:
1. Choose the right color space (`HSV/Lab/YCrCb`)
2. Set proper color thresholds (`threshold/range`)
3. Clean the mask (`blur + morphology`)
4. Evaluate quality (`precision/recall/IoU of mask`)

---

## Core (Most Important - Learn First)

1. Color thresholding by range (`inRange` logic)
2. Multi-range filtering (for example, red has 2 Hue bands)
3. Channel-wise filtering (constraints per channel)
4. Color masking and mask post-processing

**If time is limited, focus on these 3 first:**
- Choose the right color space
- Choose the right range
- Clean the mask to reduce false positives

---

## Practical (Required Hands-on)

- Compare thresholding between `RGB` vs `HSV`
- Filter red/green/yellow in at least 3 lighting conditions
- Create binary mask + original-image overlay
- Use erosion/dilation/opening/closing to reduce noise
- Tune thresholds with clear logs (no guesswork)

---

## System Notes (High-impact Real-world Factors)

- Lighting conditions
- Camera noise
- Shadow / reflection
- White balance drift
- Different sensor color responses across devices

---

## Bonus (Advanced)

- Color normalization
- White balance correction
- Color constancy
- Color clustering (K-means, GMM)

---

## Suggested Completion Checklist

- [ ] Segment objects by color with clean masks in at least 3 lighting conditions.
- [ ] Build a pipeline `preprocess -> threshold -> morphology -> evaluation`.
- [ ] Write a threshold-selection guideline for key colors.
- [ ] Reduce false positives using at least 2 different techniques.

---

## Docs Index

- `color_filtering_core.md`: color filtering foundation and color-space selection
- `threshold_and_masking.md`: thresholding, multi-range logic, mask construction
- `mask_cleanup.md`: blur + morphology + mask post-processing
- `robustness.md`: handling lighting noise, reflections, and white-balance shifts
