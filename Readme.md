# Roadmap to Become a **Computer Vision Engineer**

This repository is a practical learning path for becoming a Computer Vision Engineer with Rust as the main implementation language.

The roadmap is not organized by days anymore. It is organized by chapters. Each chapter has a clear focus, minimum theory, concrete Rust exercises, and expected outputs. The goal is not to become a math researcher or a framework collector. The goal is to build useful computer vision systems, understand why they work, and debug them when they fail.

---

## Learning Philosophy

### What to Focus On

- Build practical pipelines that work on real images.
- Understand enough math to implement and debug models.
- Prefer supervised learning, image classification, and object detection.
- Use Rust to write clear preprocessing, inference, and experiment code.
- Keep every topic connected to an observable output: image, mask, metric, model, or demo.

### What to Avoid Going Too Deep Into

- Advanced mathematical proofs.
- Low-level Rust internals that do not help build CV systems.
- Unsupervised learning as a main track.
- Reinforcement learning as a main track.
- Too many architectures without understanding training, metrics, and failure cases.

### Main Direction

```text
Math foundation
-> Image processing
-> Feature extraction
-> Supervised machine learning
-> Neural networks
-> CNN image classification
-> Object detection
-> Rust inference and portfolio projects
```

---

## Chapter 1 - Essential Math for Computer Vision

### Objective

Build the minimum mathematical foundation needed to understand image operations, model training, and neural networks.

You do not need to master proofs. You need to understand the shapes, operations, and effects well enough to implement them and debug mistakes.

### Minimum Knowledge

#### Linear Algebra

Focus on:

- Vector
- Matrix
- Dot product
- Matrix multiplication
- Transpose
- Identity matrix
- Inverse matrix
- Norm / distance
- Basic geometric meaning of linear transformation

Why it matters:

- Images are matrices.
- RGB images are 3D tensors: `height x width x channels`.
- Linear Regression uses dot product.
- Normal Equation uses transpose, matrix multiplication, and inverse.
- Neural networks are chains of matrix operations.

Practical Rust exercises:

- Implement `dot(a, b)`.
- Implement `transpose(matrix)`.
- Implement `matmul(a, b)`.
- Implement `add_bias_column(x)`.
- Implement simple Normal Equation for Linear Regression.
- Create tests for all matrix utilities.

Expected output:

- A small matrix utility module.
- A Linear Regression implementation using both Gradient Descent and Normal Equation.

Current related work:

- `days/day_3/examples/linear_regression_gradient_decent.rs`
- `days/day_3/examples/linear_regression_normal_equation.rs`
- `days/day_3/docs/Normal_Equation.md`

#### Calculus and Optimization

Focus on:

- Function
- Slope
- Derivative intuition
- Gradient
- Loss function
- Gradient Descent update rule
- Learning rate
- Dynamic learning rate

Minimum formula intuition:

```text
prediction = dot(w, x) + b
error = prediction - target
loss = mean(error^2)
w = w - learning_rate * gradient_w
b = b - learning_rate * gradient_b
```

Why it matters:

- Model training is optimization.
- Gradient tells the direction to update parameters.
- Learning rate controls update size.
- Feature scaling makes optimization easier.

Practical Rust exercises:

- Implement MSE loss.
- Implement gradients for Linear Regression.
- Train with fixed learning rate.
- Train with dynamic learning rate.
- Compare training with and without feature scaling.

Expected output:

- A working Gradient Descent training loop.
- Logs showing `epoch`, `loss`, and optionally `learning_rate`.
- A comparison report showing why scaling improves convergence.

Current related work:

- `days/day_3/docs/Linear_regression.md`
- `days/day_4/docs/Feature_Scaling.md`
- `days/day_4/docs/Learning_rate_dynamic.md`
- `days/day_4/examples/feature_scale.rs`

#### Probability and Statistics

Focus on:

- Mean
- Variance
- Standard deviation
- Distribution intuition
- Train / validation / test split
- Overfitting and underfitting
- Confusion matrix
- Precision
- Recall
- F1-score

Why it matters:

- Feature scaling uses mean and standard deviation.
- Metrics decide whether a model is actually useful.
- Computer vision datasets are often imbalanced.
- Accuracy alone is often misleading.

Practical Rust exercises:

- Implement mean and standard deviation.
- Implement train/test split.
- Implement confusion matrix.
- Implement accuracy, precision, recall, and F1.
- Evaluate a binary classifier manually.

Expected output:

- A reusable evaluation module.
- A small metrics report for classification tasks.

---

## Chapter 2 - Rust Foundation for Computer Vision Work

### Objective

Use Rust productively for computer vision experiments without over-focusing on advanced Rust internals.

### Minimum Knowledge

Focus on:

- `Vec<T>`
- `Vec<Vec<f64>>`
- Slices: `&[T]`
- Structs and `impl`
- Error handling with `Result`
- File paths with `PathBuf`
- Cargo workspace and examples
- Basic unit tests

Why it matters:

- You need clean, repeatable experiments.
- CV code often handles matrices, image buffers, paths, and batch files.
- Tests catch silent math bugs early.

Practical Rust exercises:

- Create one example per topic under `days/day_x/examples`.
- Add `#[cfg(test)]` self-tests for math-heavy code.
- Build small command-line demos that print intermediate values.
- Keep preprocessing functions separate from model code.

Expected output:

- A consistent Rust workspace.
- Examples that can be run using `cargo run -p day_x --example example_name`.
- Tests that can be run using `cargo test -p day_x --example example_name`.

Recommended crates later:

- `image`
- `imageproc`
- `opencv`
- `ndarray`
- `nalgebra`
- `tch`
- `ort` or ONNX Runtime bindings

---

## Chapter 3 - Image Representation and Color Spaces

### Objective

Understand how images are stored and how to choose the right color space for a vision task.

This is the first real Computer Vision chapter. Start here after the minimum math foundation.

### Core Topics

Focus on:

- RGB
- BGR
- HSV
- YUV
- YUV420 / NV12 / NV21
- LAB as a bonus
- Grayscale vs color image
- Channel meaning
- Range and normalization
- Raw bytes to image buffer

Why it matters:

- Color filtering depends heavily on color space.
- Camera pipelines often use YUV, not RGB.
- Deep learning models usually expect normalized RGB tensors.
- Wrong channel order silently breaks models.

Practical Rust exercises:

- Load an image and inspect width, height, and channels.
- Convert raw RGB bytes into an image.
- Split RGB channels into separate grayscale images.
- Convert RGB to HSV manually.
- Threshold a color in HSV.
- Convert RGB to YUV.
- Build a simple NV21 or NV12 buffer from RGB.
- Save intermediate outputs for visual debugging.

Expected output:

- RGB channel visualization.
- HSV channel visualization.
- YUV channel visualization.
- A color thresholding demo.

Current related work:

- `days/day_1/docs/rgb.md`
- `days/day_1/docs/hsv.md`
- `days/day_1/docs/yuv.md`
- `days/day_1/examples/rgb.rs`
- `days/day_1/examples/hsv.rs`
- `days/day_1/examples/yuv.rs`

What to pay attention to:

- RGB vs BGR channel order.
- Hue wrap-around for red objects.
- Full range vs limited range in YUV.
- UV vs VU order in NV12/NV21.
- Lighting changes can break simple thresholds.

---

## Chapter 4 - Color Filtering and Mask Processing

### Objective

Build practical color-based detection pipelines using thresholding, masks, and cleanup operations.

This chapter is directly useful for simple industrial inspection, object isolation, traffic lights, balls, signs, color markers, and preprocessing before feature extraction.

### Core Topics

Focus on:

- Thresholding
- Binary mask
- HSV range selection
- Multi-range thresholding
- Mask overlay
- Morphological transformations
- Erosion
- Dilation
- Opening
- Closing
- Connected components
- Bounding box

Why it matters:

- Many real CV pipelines start with segmentation.
- A mask converts messy pixels into object candidates.
- Morphology turns noisy masks into usable object regions.
- Bounding boxes turn masks into measurable detections.

Practical Rust exercises:

- Detect a blue object with HSV thresholds.
- Detect red using two hue ranges.
- Generate a raw binary mask.
- Count mask pixels.
- Overlay mask on the original image.
- Apply opening to remove noise.
- Apply closing to fill holes.
- Find object bounding box from mask pixels.
- Draw a rectangle around the detected object.

Expected output:

- Raw mask image.
- Cleaned mask image.
- Highlighted object image.
- Bounding box output.
- Pixel count logs before and after cleanup.

Current related work:

- `days/day_2/docs/color_filtering_core.md`
- `days/day_2/docs/threshold_and_masking.md`
- `days/day_2/docs/mask_cleanup.md`
- `days/day_2/docs/feature_extraction.md`
- `days/day_2/examples/color_filtering.rs`
- `days/day_2/examples/mask_clean_up.rs`
- `days/day_2/examples/feature_extraction.rs`

What to pay attention to:

- Always inspect the raw mask before cleanup.
- Do not tune thresholds on only one image.
- Kernel size must match object size.
- Shadows and reflections often create false positives.
- Mask cleanup should improve object shape, not hide threshold mistakes.

---

## Chapter 5 - Classical Image Processing

### Objective

Learn the practical filters and transformations needed before ML or deep learning.

You do not need to implement every algorithm from scratch. You need to understand what each operation does, when to use it, and how it affects downstream detection.

### Core Topics

Focus on:

- Grayscale conversion
- Gaussian blur
- Median blur
- Bilateral filter
- Sharpening
- Histogram
- Histogram equalization
- Edge detection
- Sobel
- Canny
- Contour extraction

Why it matters:

- Preprocessing can make a simple method work.
- Bad preprocessing can destroy useful details.
- Edge and contour logic are still useful in inspection systems.

Practical Rust exercises:

- Compare Gaussian vs Median blur on noisy images.
- Run edge detection before and after blur.
- Extract contours from a cleaned mask.
- Measure area, bounding box, aspect ratio, and center point.
- Build a small inspection rule such as "reject object if area is too small".

Expected output:

- Filter comparison grid.
- Edge map images.
- Contour and bounding box visualization.
- A rule-based inspection demo.

What to pay attention to:

- Gaussian blur reduces noise but also softens edges.
- Median blur is useful for salt-and-pepper noise.
- Bilateral filter preserves edges better but costs more.
- Canny thresholds must be tuned per image condition.

---

## Chapter 6 - Feature Extraction

### Objective

Convert images or masks into measurable features that can be used by rules or machine learning models.

### Core Topics

Focus on:

- Area
- Perimeter
- Bounding box
- Aspect ratio
- Centroid
- Color histogram
- Edge histogram
- HOG concept
- ORB concept

Why it matters:

- Before deep learning, many CV systems used handcrafted features.
- Even with deep learning, feature thinking helps with debugging.
- Features can create strong baselines for simple classification tasks.

Practical Rust exercises:

- Extract bounding box from a binary mask.
- Compute object center.
- Compute aspect ratio.
- Build a small feature vector from mask and color statistics.
- Train a simple classifier using handcrafted features.

Expected output:

- Feature extraction report.
- Object measurement demo.
- Simple feature vector printed for each image.

What to pay attention to:

- A feature must be stable across lighting, scale, and position.
- Avoid features that only work on one sample image.
- Normalize features before training ML models.

---

## Chapter 7 - Supervised Machine Learning for Vision

### Objective

Learn the supervised ML concepts that are actually needed before deep learning.

The focus is not unsupervised learning or reinforcement learning. The focus is supervised learning for classification and prediction.

### Core Topics

Focus on:

- Linear Regression
- Logistic Regression
- Binary Classification
- Multiclass Classification
- Softmax
- Cross Entropy
- Gradient Descent
- Feature Scaling
- Dynamic Learning Rate
- Train / validation / test split
- Classification metrics

Why it matters:

- Logistic Regression and Softmax explain the foundation of classifiers.
- Neural networks are built from the same training loop ideas.
- Evaluation discipline matters more than trying many models.

Practical Rust exercises:

- Implement Linear Regression from scratch.
- Implement Logistic Regression from scratch.
- Implement Binary Cross Entropy.
- Implement Softmax.
- Implement Cross Entropy.
- Implement a small classifier using handcrafted image features.
- Compare performance with and without feature scaling.
- Print confusion matrix, precision, recall, and F1.

Expected output:

- Linear Regression demo.
- Logistic Regression demo.
- Softmax multiclass classifier demo.
- Evaluation report.

Current related work:

- `days/day_3/docs/Linear_regression.md`
- `days/day_3/docs/Normal_Equation.md`
- `days/day_4/docs/Feature_Scaling.md`
- `days/day_4/docs/Learning_rate_dynamic.md`
- `days/day_4/examples/feature_scale.rs`

What to pay attention to:

- Data leakage invalidates results.
- Accuracy alone is not enough.
- Scaling affects Gradient Descent strongly.
- Loss decreasing does not always mean the model generalizes.
- Always inspect wrong predictions.

---

## Chapter 8 - Neural Networks from the Practical View

### Objective

Understand neural networks as trainable function approximators before moving into CNNs.

You only need enough theory to build, train, and debug models.

### Core Topics

Focus on:

- Neuron
- Layer
- Weight and bias
- Activation function
- ReLU
- Sigmoid
- Softmax
- Forward pass
- Loss
- Backpropagation intuition
- Overfitting
- Regularization
- Batch training

Why it matters:

- CNNs are neural networks specialized for images.
- Training issues such as overfitting, bad learning rate, and poor normalization appear here first.

Practical Rust exercises:

- Build a small MLP using a Rust ML crate.
- Train on simple feature vectors.
- Compare MLP vs Logistic Regression.
- Plot or log train loss and validation loss.
- Add early stopping logic.

Expected output:

- MLP classifier.
- Training log.
- Validation metric comparison.
- Short error analysis.

What to pay attention to:

- Start with a simple model first.
- If a simple model fails, a bigger model may only hide the real problem.
- Watch the validation gap, not only training loss.

---

## Chapter 9 - CNN and Image Classification

### Objective

Learn image classification with convolutional neural networks and transfer learning.

### Core Topics

Focus on:

- Convolution
- Kernel
- Padding
- Stride
- Pooling
- Feature map
- CNN classifier
- LeNet-5
- VGG
- ResNet
- MobileNet
- Inception
- Transfer learning
- Data augmentation

Priority order:

```text
LeNet-5 for understanding
ResNet for practical accuracy
MobileNet for lightweight inference
VGG for historical understanding
Inception as optional architecture awareness
```

Why it matters:

- Classification is the base skill before object detection.
- Transfer learning is the practical path for many real projects.
- Model choice depends on accuracy, latency, and hardware.

Practical Rust exercises:

- Train a tiny CNN on a small dataset.
- Fine-tune a pretrained classifier.
- Compare ResNet and MobileNet.
- Measure inference latency.
- Save predictions and inspect mistakes.

Expected output:

- Image classification training script.
- Metrics table.
- Failure case gallery.
- Latency measurement.

What to pay attention to:

- Normalize images exactly as the model expects.
- Keep train and validation transforms separate.
- Strong augmentation can help, but wrong augmentation can hurt.
- MobileNet is often better when deployment speed matters.

---

## Chapter 10 - Object Detection

### Objective

Move from classifying whole images to locating objects inside images.

### Core Topics

Focus on:

- Bounding box
- IoU
- Confidence score
- Class probability
- Non-Max Suppression
- Precision / Recall for detection
- mAP
- SSD
- YOLO
- Detection dataset annotation

Why it matters:

- Object detection is one of the most job-relevant CV skills.
- Real projects often need both accuracy and real-time inference.
- Annotation quality often matters more than model tweaking.

Practical Rust exercises:

- Parse bounding box annotations.
- Implement IoU.
- Implement simple NMS.
- Run inference with a pretrained YOLO or SSD model.
- Draw detected boxes on images.
- Run detection on video or webcam.
- Tune confidence and NMS thresholds.

Expected output:

- Object detection inference demo.
- Annotated output images.
- Video or webcam demo.
- Metrics and FPS report.

What to pay attention to:

- Bad labels cap performance.
- False positives and false negatives need separate analysis.
- A model that works on curated images may fail in real lighting.
- Threshold tuning is scenario-specific.

---

## Chapter 11 - Rust Inference and Deployment

### Objective

Build usable Rust applications that run computer vision models reliably.

### Core Topics

Focus on:

- Image preprocessing pipeline
- Tensor layout
- NCHW vs NHWC
- RGB vs BGR
- Batch dimension
- ONNX Runtime
- OpenCV video capture
- FPS measurement
- Latency measurement
- Postprocessing

Why it matters:

- A Computer Vision Engineer must move beyond notebooks.
- Deployment bugs often come from preprocessing mismatch.
- Rust is useful for building fast and reliable inference tools.

Practical Rust exercises:

- Load an ONNX model.
- Preprocess an image into the exact input tensor format.
- Run inference.
- Decode output.
- Draw predictions.
- Run on a folder of images.
- Run on video or webcam.
- Measure average latency and FPS.

Expected output:

- Rust image inference CLI.
- Rust video inference demo.
- Reproducible run commands.
- Latency/FPS report.

What to pay attention to:

- Match training preprocessing exactly.
- Check channel order.
- Check tensor shape.
- Check normalization.
- Time preprocessing, inference, and postprocessing separately.

---

## Chapter 12 - Portfolio Projects

### Objective

Turn learning into evidence that you can build practical CV systems.

### Project 1 - Color-Based Object Inspection

Build a rule-based inspection system using:

- Color space conversion
- HSV thresholding
- Mask cleanup
- Bounding box extraction
- Area/aspect-ratio rules
- Failure analysis

Expected output:

- Input images.
- Raw masks.
- Cleaned masks.
- Final annotated outputs.
- Short report explaining thresholds, failure cases, and improvements.

### Project 2 - Image Classification System

Build an image classifier using:

- Dataset split
- Preprocessing
- Transfer learning
- Metrics
- Error analysis
- Rust inference

Expected output:

- Training results.
- Confusion matrix.
- Wrong prediction gallery.
- Rust inference command.

### Project 3 - Object Detection System

Build an object detector using:

- Labeled dataset
- YOLO or SSD workflow
- Detection metrics
- NMS threshold tuning
- Rust video inference

Expected output:

- Detection demo.
- mAP or practical metric report.
- FPS report.
- False positive / false negative analysis.

---

## Practical Study Loop

Use this loop for every chapter:

```text
1. Learn only the minimum theory.
2. Implement the smallest working version in Rust.
3. Save visual or metric output.
4. Break the pipeline intentionally and observe failure.
5. Fix the failure and document the rule.
6. Reuse the module in a more realistic task.
```

This keeps learning close to real engineering work.

---

## Repository Structure

Current practical modules:

```text
days/day_1  -> Color spaces: RGB, HSV, YUV
days/day_2  -> Color filtering, mask cleanup, feature extraction
days/day_3  -> Linear Regression, Gradient Descent, Normal Equation
days/day_4  -> Feature Scaling, Dynamic Learning Rate
```

Future modules should follow the same pattern:

```text
days/day_x/docs      -> theory and notes
days/day_x/examples  -> runnable Rust examples
days/day_x/src       -> reusable code if needed
```

Even though the folder names still use `day_x`, the learning path is now chapter-based. Treat each folder as a module, not a strict calendar day.

---

## Skill Checklist

### Image Processing

- [ ] Understand RGB, HSV, YUV, and LAB use cases.
- [ ] Build stable color thresholding pipelines.
- [ ] Clean masks with morphology.
- [ ] Extract bounding boxes and object measurements.
- [ ] Debug lighting, reflection, and noise issues.

### Supervised Machine Learning

- [ ] Implement Linear Regression.
- [ ] Implement Logistic Regression.
- [ ] Implement Softmax classification.
- [ ] Understand Gradient Descent and learning rate.
- [ ] Use feature scaling correctly.
- [ ] Evaluate with precision, recall, F1, and confusion matrix.

### Deep Learning

- [ ] Understand CNN basics.
- [ ] Train or fine-tune an image classifier.
- [ ] Compare ResNet and MobileNet.
- [ ] Use augmentation correctly.
- [ ] Track loss curves and validation metrics.

### Object Detection

- [ ] Understand IoU and NMS.
- [ ] Run pretrained detection inference.
- [ ] Fine-tune or use YOLO/SSD workflow.
- [ ] Analyze false positives and false negatives.
- [ ] Build real-time Rust inference.

### Engineering

- [ ] Keep reproducible commands.
- [ ] Write focused docs for every experiment.
- [ ] Add tests for math and preprocessing functions.
- [ ] Save visual outputs for debugging.
- [ ] Measure latency and FPS for inference.

---

## Final Priority Order

If time or energy is limited, focus in this order:

```text
1. Image preprocessing robustness
2. Supervised ML fundamentals
3. CNN image classification
4. Object detection inference
5. Deployment and portfolio polish
```

The main target is not to know every algorithm deeply. The main target is to build reliable computer vision pipelines, understand the failure modes, and explain engineering decisions clearly.
