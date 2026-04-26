# 100-Day Roadmap to Become a **Computer Vision Engineer** (Rust-First)

> Goal: finish a focused, job-ready computer vision track in **100 days**

---

## 1) 100-Day Master Timeline

```text
Day 1-5     : Fast Foundation Refresh (Rust + Math + Tooling)
Day 6-25    : Image Processing (Core Focus #1)
Day 26-45   : Supervised ML for Vision (Core Focus #2)
Day 46-70   : Deep Learning Classification (Core Focus #3)
Day 71-90   : Object Detection + Deployment (Core Focus #4)
Day 91-100  : Portfolio Finalization + Interview Readiness
```

- Weekly target: **14-18 hours/week** minimum.
- If possible, 2-3 hours/day on weekdays + 4-6 hours/day on weekends.

---

## 2) What to De-Emphasize

Since you already have basics:
- No deep dive into advanced math proofs.
- No deep dive into low-level Rust internals.
- No broad exploration of unrelated ML areas.

Your success in 100 days depends on:
- Practical pipelines
- Correct metrics
- Model debugging ability
- Clear project outputs on GitHub

---

## 3) Phase-by-Phase Plan (Detailed)

## Phase 0 (Day 1-5) - Fast Refresh Only

### Focus
- Quick Rust productivity refresh
- Quick optimization/math intuition refresh
- Environment setup for fast iteration

### Must Do
- Setup one workspace with reusable structure:
  - `data/`, `src/bin/`, `models/`, `reports/`
- Prepare Rust crates:
  - `opencv`, `image`, `imageproc`, `ndarray`, `nalgebra`, `tch`, `onnxruntime`
- Build 2 tiny utilities:
  - Image read/write + resize
  - Batch folder preprocessing script

### Important Notes
- Do not spend more than 5 days here.
- If blocked by tooling, fix once and template it for the rest of roadmap.

---

## Phase 1 (Day 6-25) - Image Processing

### Focus Areas
- Color spaces: RGB/BGR, HSV, LAB
- Filtering: Gaussian, Median, Bilateral, Canny
- Morphology: erosion, dilation, opening, closing
- Feature extraction: contour, HOG, ORB

### What Matters Most
- Choosing the right preprocessing chain for noisy real images
- Understanding tradeoff between denoising and edge/detail loss
- Building reusable preprocessing modules

### Deliverables
- Repo: `cv-image-processing-rust-100d`
- Minimum binaries:
  - `color_pipeline.rs`
  - `filter_benchmark.rs`
  - `morphology_cleaning.rs`
  - `orb_matching.rs`
- Report with:
  - Before/after examples
  - PSNR/SSIM (where relevant)
  - Failure cases and fixes

### Critical Attention Points
- Kernel size selection can destroy useful details.
- HSV thresholds are very sensitive to lighting.
- Always test on at least 3 different image conditions.

---

## Phase 2 (Day 26-45) - Supervised ML for Vision Tasks

### Focus Areas
- Logistic regression and softmax baseline
- Data splitting and leakage prevention
- Metrics: precision, recall, F1, confusion matrix
- Basic MLP with `tch`

### What Matters Most
- Baseline discipline: build a simple baseline first
- Correct evaluation on imbalanced data
- Feature quality and preprocessing impact on model stability

### Deliverables
- Repo: `ml-supervised-rust-100d`
- Minimum binaries/notebooks-equivalent:
  - `logistic_baseline.rs`
  - `softmax_multiclass.rs`
  - `mlp_baseline_tch.rs`
- `evaluation.md` with metric comparison and model errors

### Critical Attention Points
- Accuracy alone is misleading for imbalanced labels.
- Data leakage invalidates your entire experiment.
- Keep train/val/test protocol fixed and documented.

---

## Phase 3 (Day 46-70) - Deep Learning Classification

### Focus Areas
- CNN fundamentals in practice
- ResNet, MobileNet, VGG (priority order: ResNet -> MobileNet -> VGG)
- Transfer learning and augmentation strategy
- Training diagnostics and overfitting control

### What Matters Most
- Convergence behavior (loss curves, validation gap)
- Throughput vs accuracy tradeoff
- Choosing architecture based on deployment constraints

### Deliverables
- Repo: `dl-classification-rust-100d`
- Train and compare at least 3 backbones
- `experiment_tracker.md` including:
  - Accuracy/F1
  - Params
  - Inference latency
  - Main failure categories

### Critical Attention Points
- Poor augmentation causes brittle models.
- Overfitting usually appears early; monitor val metrics each epoch.
- MobileNet is often best for real-time constraints.

---

## Phase 4 (Day 71-90) - Object Detection + Rust Inference

### Focus Areas
- Detection metrics: IoU, mAP@0.5, mAP@0.5:0.95
- YOLO workflow concepts (training + error analysis)
- Export and deploy inference in Rust (ONNX Runtime + OpenCV)

### What Matters Most
- Annotation quality (more important than model tweaking in many cases)
- False positive/false negative analysis
- Stable real-time inference pipeline

### Deliverables
- Repo: `object-detection-rust-100d`
- One custom labeled dataset (target 500+ images)
- One fine-tuned detector
- One Rust app for webcam/video inference
- `results.md` with mAP/FPS and failure analysis

### Critical Attention Points
- Bad labels will cap performance hard.
- NMS/confidence thresholds need scenario-specific tuning.
- Evaluate in real conditions, not just curated samples.

---

## Phase 5 (Day 91-100) - Portfolio and Job Packaging

### Focus Areas
- Turn technical work into hiring evidence
- Prepare demo-ready projects
- Final cleanup for reproducibility

### Required Portfolio Projects
- Project A: `Smart Quality Inspection`
- Project B: `Traffic/People Detection Analytics`

### Must Include for Each Project
- Problem statement
- Dataset details and label policy
- Method and architecture choice
- Metrics + latency + hardware context
- Error analysis and next improvements
- Exact run commands

### Critical Attention Points
- Recruiters care about measurable impact, not architecture name-dropping.
- Reproducibility and clarity in README are mandatory.
- Demo video greatly increases portfolio quality.

---

## 4) Weekly Execution Template (for 100 Days)

```text
Mon:    New concept + implementation start
Tue:    Continue implementation + unit checks
Wed:    Evaluation + error analysis
Thu:    Iteration on weak points
Fri:    Metrics review + short technical write-up
Sat:    Deep build session (feature/model improvements)
Sun:    Consolidate, cleanup, and next-week plan
```

---

## 5) Top Priorities to Focus (Most Important)

1. Data quality and labeling consistency.
2. Correct metrics and evaluation protocol.
3. Error analysis discipline after every training run.
4. Inference speed and deployment constraints.
5. Reproducible project structure and documentation.

---

## 6) 100-Day Success Checklist

- [ ] Completed all 6 phases on schedule.
- [ ] Published at least 4 high-quality Rust CV repos.
- [ ] Built 2 end-to-end portfolio projects.
- [ ] Demonstrated real-time inference in at least 1 project.
- [ ] Documented metrics, failure analysis, and reproducible run steps.

---

## 7) Final Rule

If time is limited, prioritize in this order:
1. Object Detection pipeline quality
2. Classification reliability
3. Image preprocessing robustness
4. Model variety
5. Extra theory

In a 100-day sprint, shipping strong, measurable systems beats broad but shallow coverage.
