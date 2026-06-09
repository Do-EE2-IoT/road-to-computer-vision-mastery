# Roadmap to Become a **Computer Vision Engineer**

This roadmap is organized by chapters, not by days.

The goal is to become practical enough to build, debug, and explain computer vision systems. The focus is Rust-based implementation, image processing, supervised machine learning, CNN image classification, and object detection.

You do not need to go too deep into mathematical proofs or Rust internals. You need enough foundation to reason about images, transformations, models, metrics, and inference pipelines.

---

## Core Direction

```text
Math for images
-> Rust implementation foundation
-> Image representation and color spaces
-> Filtering, morphology, and masks
-> Feature extraction
-> Supervised machine learning
-> Neural networks
-> CNN image classification
-> Object detection
-> Rust inference and portfolio projects
```

What to prioritize:

- Image processing
- Supervised learning
- CNN-based classification
- Object detection
- Practical Rust inference
- Debugging by visual outputs and metrics

What to de-prioritize:

- Deep mathematical proofs
- Reinforcement learning
- Unsupervised learning as a main track
- Too many architectures without experiments
- Low-level Rust topics that do not help CV work

---

## Chapter 1 - Mathematics for Images

### Objective

Build the minimum math foundation needed to understand images as numeric data.

The goal is to become comfortable treating an image as numbers, coordinates, vectors, matrices, channels, and transformations. This chapter stays focused on image math only.

### Minimum Math You Must Know

#### 1. Numbers, Ranges, and Clamping

You must understand:

- Integer vs floating-point values
- Pixel range: `0..255`
- Normalized range: `0.0..1.0`
- Overflow and underflow
- Clamping
- Rounding
- Type conversion: `u8 -> f32`, `f32 -> u8`

Why it matters:

- Image pixels are usually stored as `u8`.
- Many calculations need `f32` or `f64`.
- If you increase brightness without clamping, values can overflow.

Practical Rust exercises:

- Load an image and print pixel values.
- Convert `u8` pixels into `f32`.
- Normalize pixels from `[0, 255]` to `[0.0, 1.0]`.
- Convert normalized pixels back to `u8`.
- Implement brightness adjustment:

```text
new_pixel = clamp(pixel + value, 0, 255)
```

- Implement contrast adjustment:

```text
new_pixel = clamp((pixel - 128) * factor + 128, 0, 255)
```

Expected output:

- Original image.
- Brighter image.
- Darker image.
- Higher contrast image.
- Lower contrast image.

#### 2. Coordinate System

You must understand:

- Image width and height
- Pixel coordinate `(x, y)`
- Top-left origin
- Row-major layout
- Index formula
- Neighbor pixels
- Boundary handling

Important coordinate rule:

```text
x = column
y = row
index = y * width + x
```

Why it matters:

- Almost every image algorithm loops over pixels.
- Filters need neighbor pixels.
- Bounding boxes and masks depend on correct coordinates.

Practical Rust exercises:

- Print the `(x, y)` coordinate of selected pixels.
- Convert `(x, y)` into a 1D buffer index.
- Draw a red point at a given coordinate.
- Draw a horizontal line.
- Draw a vertical line.
- Draw a rectangle border.
- Crop an image using `(x_min, y_min, x_max, y_max)`.

Expected output:

- Image with points and lines.
- Image with a manually drawn rectangle.
- Cropped image.

#### 3. Vectors

You must understand:

- Vector as a list of numbers
- Vector length
- Dot product
- Magnitude / norm
- Distance between two vectors
- Vector as a feature representation

Minimum formulas:

```text
dot(a, b) = sum(a[i] * b[i])
norm(a) = sqrt(sum(a[i]^2))
distance(a, b) = sqrt(sum((a[i] - b[i])^2))
```

Why it matters:

- RGB pixel can be treated as a vector: `[R, G, B]`.
- Color similarity can be measured with distance.
- Feature extraction later produces vectors.

Practical Rust exercises:

- Implement `dot(a, b)`.
- Implement `norm(a)`.
- Implement `euclidean_distance(a, b)`.
- Compute distance between two RGB colors.
- Find pixels close to a target color using RGB distance.

Example:

```text
target = [255, 0, 0]
pixel = [240, 20, 10]
distance = color_distance(target, pixel)
```

Expected output:

- A color similarity mask.
- A highlighted image showing pixels close to the target color.

#### 4. Matrices

You must understand:

- Matrix rows and columns
- Matrix shape
- Transpose
- Matrix multiplication
- Identity matrix
- Matrix as grayscale image
- Matrix as transformation

Why it matters:

- A grayscale image is a matrix.
- Many image filters operate on a small matrix called a kernel.
- Matrix operations make image flipping, cropping, and geometric transformations easier to reason about.

Practical Rust exercises:

- Represent a grayscale image as `Vec<Vec<f32>>`.
- Implement `transpose(matrix)`.
- Implement `matmul(a, b)`.
- Convert RGB image to grayscale matrix.
- Convert grayscale matrix back to image.
- Flip image horizontally using matrix indexing.
- Flip image vertically using matrix indexing.

Expected output:

- Grayscale image.
- Horizontally flipped image.
- Vertically flipped image.
- Matrix utility tests.

#### 5. Channels and Tensors

You must understand:

- Grayscale image: `height x width`
- RGB image: `height x width x 3`
- Channel order
- Interleaved layout
- Planar layout
- HWC layout
- CHW layout

Important layouts:

```text
Interleaved RGB:
[R G B][R G B][R G B]...

Planar RGB:
[R R R ...][G G G ...][B B B ...]

HWC:
height, width, channel

CHW:
channel, height, width
```

Why it matters:

- Image files often use interleaved RGB.
- Some image pipelines use planar channel layouts.
- Wrong layout gives wrong colors or corrupted reconstructed images.

Practical Rust exercises:

- Split RGB image into `R`, `G`, and `B` channel images.
- Merge `R`, `G`, and `B` back into RGB.
- Convert interleaved RGB to planar RGB.
- Convert HWC layout to CHW layout.
- Verify conversion by reconstructing the image.

Expected output:

- Red channel image.
- Green channel image.
- Blue channel image.
- Reconstructed RGB image.

#### 6. Weighted Sum

You must understand:

- Weighted average
- Weighted sum over channels
- Weighted sum over neighbor pixels

Why it matters:

- Grayscale conversion is a weighted sum.
- Blur and edge detection are weighted sums over neighborhoods.
- Many image operations are just repeated weighted sums.

Practical Rust exercises:

- Implement grayscale conversion:

```text
gray = 0.299 * R + 0.587 * G + 0.114 * B
```

- Implement simple average grayscale:

```text
gray = (R + G + B) / 3
```

- Compare both outputs.

Expected output:

- Weighted grayscale image.
- Average grayscale image.
- Short note explaining the visual difference.

#### 7. Convolution and Kernels

You must understand:

- Kernel
- Kernel size
- Center pixel
- Neighbor pixels
- Padding
- Stride
- Convolution as weighted sum

Core idea:

```text
output_pixel = sum(neighbor_pixel * kernel_weight)
```

Why it matters:

- Blur uses kernels.
- Sharpening uses kernels.
- Edge detection uses kernels.
- Many classical image operations are built from small kernels.

Practical Rust exercises:

- Implement a generic 3x3 convolution on grayscale images.
- Apply box blur kernel.
- Apply sharpen kernel.
- Apply Sobel X kernel.
- Apply Sobel Y kernel.
- Combine Sobel X and Sobel Y into edge magnitude.

Example kernels:

```text
Box blur:
1/9 * [
  [1, 1, 1],
  [1, 1, 1],
  [1, 1, 1],
]

Sharpen:
[
  [ 0, -1,  0],
  [-1,  5, -1],
  [ 0, -1,  0],
]

Sobel X:
[
  [-1, 0, 1],
  [-2, 0, 2],
  [-1, 0, 1],
]
```

Expected output:

- Blurred image.
- Sharpened image.
- Horizontal edge map.
- Vertical edge map.
- Combined edge map.

#### 8. Basic Statistics

You must understand:

- Mean
- Variance
- Standard deviation
- Min / max
- Histogram
- Normalization

Why it matters:

- Image brightness can be measured by mean pixel value.
- Contrast can be measured by standard deviation.
- Histograms describe intensity distribution.
- Normalization is used before model input.

Practical Rust exercises:

- Compute mean intensity of a grayscale image.
- Compute min and max pixel value.
- Compute standard deviation of pixel values.
- Build a 256-bin grayscale histogram.
- Normalize image values to `[0.0, 1.0]`.
- Apply min-max normalization.

Expected output:

- Printed image statistics.
- Histogram data.
- Normalized image.

#### 9. Geometric Transformations

You must understand:

- Translation
- Scaling
- Rotation intuition
- Nearest-neighbor sampling
- Bilinear interpolation concept

Why it matters:

- Image augmentation uses geometric transformations.
- Object position and scale affect detection.
- Incorrect sampling creates artifacts.

Practical Rust exercises:

- Translate image by `(dx, dy)`.
- Resize image using nearest-neighbor sampling.
- Rotate image by 90 degrees.
- Implement center crop.
- Compare resize artifacts.

Expected output:

- Translated image.
- Resized image.
- Rotated image.
- Center-cropped image.

### Chapter 1 Completion Criteria

You are ready to move on when you can:

- Treat an image as numeric arrays.
- Loop over pixels safely.
- Convert between `u8` and `f32`.
- Implement grayscale conversion manually.
- Split and merge RGB channels.
- Apply a 3x3 kernel.
- Compute image statistics.
- Draw simple geometry on an image.
- Explain what changed in the output image and why.

---

## Chapter 2 - Rust Foundation for Computer Vision Work

### Objective

Use Rust productively for image and model experiments without spending too much time on unrelated language depth.

### Minimum Rust Knowledge

Focus on:

- `Vec<T>`
- `Vec<Vec<T>>`
- Slices: `&[T]`
- Structs
- `impl`
- `Result`
- `Option`
- `PathBuf`
- Cargo workspace
- Cargo examples
- Unit tests

Practical exercises:

- Create a reusable image-loading helper.
- Create a small matrix utility module.
- Write self-tests for math functions.
- Build one runnable example per concept.
- Print intermediate values while learning.

Expected output:

- Clear Rust examples.
- Small reusable helper functions.
- Unit tests for math and preprocessing logic.

---

## Chapter 3 - Image Representation and Color Spaces

### Objective

Understand how images are stored and choose the right color space for each task.

### Core Topics

Focus on:

- RGB
- BGR
- HSV
- YUV
- YUV420
- NV12
- NV21
- LAB as a bonus
- Grayscale vs color image
- Channel meaning
- Range and normalization
- Raw bytes to image buffer

Why it matters:

- Color filtering depends heavily on color space.
- Camera pipelines often use YUV.
- Many libraries use BGR while image files may use RGB.
- Wrong channel order silently breaks results.

Practical Rust exercises:

- Load an image and inspect width, height, and channel count.
- Convert raw RGB bytes into an image.
- Split RGB channels.
- Convert RGB to HSV manually.
- Visualize H, S, and V channels.
- Convert RGB to YUV.
- Build a simple NV12 or NV21 buffer.
- Save all intermediate outputs.

Expected output:

- RGB channel visualization.
- HSV channel visualization.
- YUV channel visualization.
- Color-space comparison notes.

What to pay attention to:

- RGB vs BGR channel order.
- Hue wrap-around for red.
- Full range vs limited range in YUV.
- UV vs VU order in NV12/NV21.
- Lighting changes can break color thresholds.

---

## Chapter 4 - Color Filtering and Mask Processing

### Objective

Build practical color-based detection pipelines using thresholds, masks, and cleanup operations.

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

- Many practical CV systems start with segmentation.
- A mask converts pixels into object candidates.
- Morphology turns noisy masks into usable regions.
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

What to pay attention to:

- Always inspect the raw mask before cleanup.
- Do not tune thresholds on only one image.
- Kernel size must match object size.
- Shadows and reflections often create false positives.
- Mask cleanup should improve object shape, not hide threshold mistakes.

---

## Chapter 5 - Classical Image Processing

### Objective

Learn practical filters and transformations used before ML or deep learning.

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

- Preprocessing can make a simple system work.
- Bad preprocessing can destroy useful details.
- Edge and contour logic are still useful in inspection systems.

Practical Rust exercises:

- Compare Gaussian vs Median blur on noisy images.
- Run edge detection before and after blur.
- Extract contours from a cleaned mask.
- Measure area, bounding box, aspect ratio, and center point.
- Build a small rule-based inspection demo.

Expected output:

- Filter comparison grid.
- Edge map images.
- Contour and bounding box visualization.
- Rule-based inspection output.

What to pay attention to:

- Gaussian blur reduces noise but softens edges.
- Median blur is useful for salt-and-pepper noise.
- Bilateral filter preserves edges better but costs more.
- Canny thresholds must be tuned per image condition.

---

## Chapter 6 - Feature Extraction

### Objective

Convert images or masks into measurable features that can be used by rules or supervised learning models.

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

- Features create strong baselines for simple tasks.
- Feature thinking helps debug deep learning failures later.
- Measurements are often enough for inspection systems.

Practical Rust exercises:

- Extract bounding box from a binary mask.
- Compute object center.
- Compute aspect ratio.
- Compute object area.
- Build a feature vector from shape and color statistics.
- Normalize feature vectors.

Expected output:

- Feature extraction report.
- Object measurement demo.
- Printed feature vectors.

What to pay attention to:

- Features must be stable across lighting, scale, and position.
- Avoid features that only work on one sample image.
- Normalize features before training models.

---

## Chapter 7 - Supervised Machine Learning for Vision

### Objective

Learn the supervised ML concepts needed before neural networks and CNNs.

The focus is supervised learning. Unsupervised learning and reinforcement learning are not the main track.

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

- Logistic Regression and Softmax are the foundation of classifiers.
- Neural networks reuse the same training loop concepts.
- Evaluation discipline matters more than trying many models.

Practical Rust exercises:

- Implement Linear Regression from scratch.
- Implement Logistic Regression from scratch.
- Implement Binary Cross Entropy.
- Implement Softmax.
- Implement Cross Entropy.
- Train a classifier using handcrafted image features.
- Compare training with and without feature scaling.
- Print confusion matrix, precision, recall, and F1.

Expected output:

- Linear Regression demo.
- Logistic Regression demo.
- Softmax classifier demo.
- Evaluation report.

What to pay attention to:

- Data leakage invalidates results.
- Accuracy alone is not enough.
- Scaling affects Gradient Descent strongly.
- Loss decreasing does not always mean the model generalizes.
- Always inspect wrong predictions.

---

## Chapter 8 - Neural Networks

### Objective

Understand neural networks as trainable function approximators before moving into CNNs.

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
- Train on handcrafted image features.
- Compare MLP vs Logistic Regression.
- Log train loss and validation loss.
- Add early stopping logic.

Expected output:

- MLP classifier.
- Training log.
- Validation metric comparison.
- Short error analysis.

What to pay attention to:

- Start with a simple model first.
- If a simple model fails, a bigger model may only hide the real issue.
- Watch validation metrics, not only training loss.

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
- Wrong augmentation can hurt performance.
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

---

## Skill Checklist

### Math and Image Basics

- [ ] Convert pixels between `u8` and `f32`.
- [ ] Normalize and clamp pixel values.
- [ ] Work with image coordinates safely.
- [ ] Treat grayscale images as matrices.
- [ ] Treat RGB pixels as vectors.
- [ ] Split and merge image channels.
- [ ] Apply 3x3 convolution kernels.
- [ ] Compute image statistics and histograms.
- [ ] Perform basic geometric transformations.

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
1. Math applied directly to images
2. Image preprocessing robustness
3. Supervised ML fundamentals
4. CNN image classification
5. Object detection inference
6. Deployment and portfolio polish
```

The main target is not to know every algorithm deeply. The main target is to build reliable computer vision pipelines, understand the failure modes, and explain engineering decisions clearly.
