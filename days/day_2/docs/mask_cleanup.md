# Mask Cleanup (Blur + Morphology)

## 1) What is mask cleanup?

Mask cleanup is the process of cleaning a **binary mask** (`0/255`) after color thresholding.

Goals:
- Reduce isolated noise
- Fill holes inside objects
- Stabilize edges so contour/bbox/counting is more reliable

---

## 2) Why cleanup is needed

Raw threshold masks often contain:
- isolated white noise (false positives)
- black holes inside objects (false negatives)
- jagged boundaries
- small irrelevant blobs

If cleanup is skipped, downstream steps (connected components, contour, tracking) become unstable.

---

## 3) Two processing layers

### 3.1. Pre-filter before threshold (on color image)

- `Gaussian blur`: fast, good for random noise
- `Median blur`: good for salt-and-pepper noise
- `Bilateral`: preserves edges better but is slower

### 3.2. Morphology after threshold (on binary mask)

- `Erode`: shrinks white regions, removes small white noise
- `Dilate`: expands white regions, connects nearby areas
- `Opening = Erode -> Dilate`: removes small white noise
- `Closing = Dilate -> Erode`: fills small black holes

---

## 4) Short mathematical intuition

Let mask be `M`, kernel be `K`:
- Erode keeps a white pixel only when all neighborhood pixels under `K` are white
- Dilate keeps a white pixel if at least one neighborhood pixel under `K` is white

Intuition:
- Erode = strict
- Dilate = lenient

---

## 5) Kernel size and iterations

- Kernel `3x3`: light cleanup, preserves details
- Kernel `5x5`+: stronger cleanup, may remove thin details
- More iterations: stronger effect, similar to larger kernel

Recommended start:
1. Kernel `3x3`
2. Iteration `1`
3. Increase gradually with validation

---

## 6) Suggested pipeline for `color_filtering.rs`

```text
RGB -> HSV -> threshold -> raw_mask -> opening -> closing -> final_mask
```

If small objects disappear:
- reduce kernel size
- reduce iterations
- try only `closing`

If noise remains high:
- strengthen opening
- add pre-blur before threshold

---

## 7) Quick matrix examples (3x3 concept)

### 7.1. Opening removes isolated white noise

Input:
```text
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 1 0 0
```

After `opening`:
```text
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 0 0 0
```

### 7.2. Closing fills black holes

Input:
```text
0 0 0 0 0
0 1 1 1 0
0 1 0 1 0
0 1 1 1 0
0 0 0 0 0
```

After `closing`:
```text
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 0 0 0
```

---

## 8) How to measure cleanup quality

Track before/after metrics:
- number of white pixels
- number of connected components
- total area of valid blobs
- mask IoU (if ground truth exists)

For video:
- bbox stability across frames

---

## 9) Common implementation mistakes

- Writing output into input buffer during kernel scan
- Missing boundary handling
- Oversized kernels that remove thin objects
- Tuning on one image only

---

## 10) Implementation checklist

- [ ] Visualize both `raw_mask` and `clean_mask`
- [ ] Include at least `opening + closing`
- [ ] Log white-pixel count before/after
- [ ] Test across bright/low-light conditions
- [ ] Tune on real data, not by guesswork
