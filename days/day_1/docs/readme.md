# Day 1 - Color Workspace Image Processing

> Learning target for this day: understand color spaces for vision tasks and apply them in practical image-processing pipelines.

---

## Goal

**Color Workspace Image Processing**

Build a clear mental model of color representations and use them correctly in real pipelines (offline images and camera input).

---

## Core

### RGB, HSV, YUV

- **RGB**: direct display color space, intuitive for rendering, less stable for segmentation under lighting changes.
- **HSV**: separates hue from brightness, better for color-based thresholding.
- **YUV**: separates luminance/chrominance, widely used in camera/video pipelines.

### Meaning of Each Channel

- **RGB**
  - `R`: red intensity
  - `G`: green intensity
  - `B`: blue intensity
- **HSV**
  - `H`: color type (hue angle)
  - `S`: color intensity (saturation)
  - `V`: brightness (value)
- **YUV**
  - `Y`: luma (brightness information)
  - `U`, `V`: chroma (color-difference information)

---

## Practical

### Threshold in HSV

- Use HSV thresholding for robust color segmentation.
- For red detection, use two hue ranges around hue wrap-around (near `0°` and near `360°`).
- Combine Hue with `S`/`V` thresholds to remove weak-color and dark noise.

### Grayscale vs Color

- **Grayscale**: useful for edges, shape, structure, and many classical operators.
- **Color**: required when hue/chroma semantics matter (object-by-color tasks).
- Choose based on task objective, not by habit.

### Range and Normalize

- Keep track of value ranges:
  - RGB often `0..255`
  - HSV usually `H: 0..360`, `S/V: 0..1` (or scaled forms)
- Normalize consistently before math/thresholding.
- Document conversion/scaling rules to avoid hidden bugs.

---

## System

### YUV420 / NV12

- Understand common camera frame formats.
- **YUV420**: reduced chroma resolution for bandwidth savings.
- **NV12**: one Y plane + interleaved UV plane (widely used in hardware pipelines).

### Chroma Subsampling

- Concept: human vision is more sensitive to luma than chroma.
- 4:2:0 reduces color bandwidth while keeping acceptable visual quality.
- Important for performance, memory, and real-time systems.

### Camera Pipeline

Typical flow:
1. Sensor frame capture (often YUV/NV12)
2. Color conversion to processing space (RGB/HSV/Lab as needed)
3. Preprocessing (normalize, denoise, ROI)
4. Algorithm/model inference
5. Postprocess + visualization/output

---

## Bonus

### Lab

- Perceptually motivated color space.
- Useful for color-distance tasks and illumination-robust comparisons.

### Gamma

- Brightness nonlinearity correction.
- Critical when converting between linear and display-referred data.

### Color Consistency

- Keep color handling consistent across datasets, devices, and preprocessing steps.
- Inconsistent color pipeline can silently degrade model quality.

---

## Suggested Completion Checklist

- [ ] Explain RGB/HSV/YUV roles in under 2 minutes each.
- [ ] Implement one HSV threshold demo (with mask output).
- [ ] Compare grayscale vs color pipeline on the same sample image.
- [ ] Convert one NV12/YUV420 frame to RGB and inspect channel behavior.
- [ ] Write a short note on where Lab/gamma help in production.

---

## Docs Index

- `rgb.md`: RGB formats, raw data layout, and raw-to-RGB conversion.
- `hsv.md`: HSV math, channel interpretation, and red detection notes.
- `yuv.md`: YUV formats, subsampling, system usage, and YUV<->RGB conversion.
