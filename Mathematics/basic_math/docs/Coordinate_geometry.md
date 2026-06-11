# Chapter 2: Coordinate Geometry for Computer Vision (Rust)

## Introduction

Coordinate Geometry is one of the most important foundations of Computer Vision and Image Processing.

Every image consists of pixels, and every pixel has a coordinate. Operations such as:

* Drawing points
* Drawing lines
* Drawing rectangles
* Cropping images
* Resizing images
* Object detection
* Face detection
* Image segmentation

all rely on coordinate systems.

Before working with OpenCV, neural networks, or advanced Computer Vision algorithms, you must have a solid understanding of coordinate geometry.

---

# Learning Objectives

By the end of this chapter, you should be able to:

* Understand the 2D image coordinate system
* Work with points and rectangles
* Convert between coordinates and pixel indices
* Calculate distances between points
* Perform translation and scaling operations
* Draw geometric primitives on images
* Crop images using rectangle coordinates
* Avoid common coordinate-related bugs

---

# 1. The 2D Coordinate System

A digital image can be viewed as a two-dimensional grid of pixels.

Example:

```text
Width = 5
Height = 4
```

```text
(0,0) (1,0) (2,0) (3,0) (4,0)

(0,1) (1,1) (2,1) (3,1) (4,1)

(0,2) (1,2) (2,2) (3,2) (4,2)

(0,3) (1,3) (2,3) (3,3) (4,3)
```

Each location in the image is identified by:

```text
(x, y)
```

where:

* x represents the horizontal position
* y represents the vertical position

---

# 2. Image Origin

Unlike traditional mathematics, images use the top-left corner as the origin.

Traditional Cartesian coordinate system:

```text
y ↑

|
|
+------→ x
```

Image coordinate system:

```text
(0,0)
+------------→ x
|
|
|
↓ y
```

This means:

* x increases from left to right
* y increases from top to bottom

This convention is used by:

* OpenCV
* Image crate
* Pillow
* TensorFlow
* PyTorch

---

# 3. Points

A point represents a single location in the image.

Mathematical representation:

```text
(x, y)
```

Example:

```text
(120, 75)
```

Rust implementation:

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}
```

---

# 4. Image Width and Height

Every image has dimensions:

```text
width × height
```

Example:

```text
1920 × 1080
```

where:

```text
width  = 1920 pixels
height = 1080 pixels
```

Important:

```text
width  = number of columns
height = number of rows
```

---

# 5. Rows and Columns

Images are often stored as rows and columns.

Relationship:

```text
row    = y
column = x
```

Example:

```text
Pixel (100, 50)

x = 100
y = 50

column = 100
row = 50
```

A common mistake is confusing:

```rust
image[y][x]
```

with:

```rust
image[x][y]
```

Always remember:

```text
row = y
column = x
```

---

# 6. Rectangles

Rectangles are used heavily in Computer Vision.

Examples:

* Bounding boxes
* Cropping regions
* Detection results
* Tracking regions

A rectangle is represented as:

```text
(x, y, width, height)
```

Example:

```text
x = 100
y = 50
width = 200
height = 100
```

Visualization:

```text
(100,50)
+--------------------+
|                    |
|                    |
+--------------------+
```

Rust implementation:

```rust
#[derive(Debug)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
```

---

# 7. Distance Between Two Points

Suppose we have:

```text
A(x1, y1)
B(x2, y2)
```

The Euclidean distance is:

```text
distance = sqrt((x2 - x1)^2 + (y2 - y1)^2)
```

Example:

```text
A(0,0)
B(3,4)
```

```text
distance = 5
```

Rust implementation:

```rust
fn point_distance(a: Point, b: Point) -> f64 {
    let dx = (b.x - a.x) as f64;
    let dy = (b.y - a.y) as f64;

    (dx * dx + dy * dy).sqrt()
}
```

---

# 8. Translation

Translation means moving a point by a fixed offset.

Formula:

```text
x_new = x + tx
y_new = y + ty
```

where:

```text
tx = horizontal offset
ty = vertical offset
```

Example:

```text
Point = (10, 20)

tx = 5
ty = -2
```

Result:

```text
(15, 18)
```

Rust implementation:

```rust
fn translate(p: Point, tx: i32, ty: i32) -> Point {
    Point {
        x: p.x + tx,
        y: p.y + ty,
    }
}
```

---

# 9. Scaling

Scaling changes the size of coordinates.

Formula:

```text
x_new = x × sx
y_new = y × sy
```

Example:

```text
Point = (10, 20)

sx = 2
sy = 2
```

Result:

```text
(20, 40)
```

Rust implementation:

```rust
fn scale(p: Point, sx: f32, sy: f32) -> Point {
    Point {
        x: (p.x as f32 * sx) as i32,
        y: (p.y as f32 * sy) as i32,
    }
}
```

---

# 10. Pixel Indexing

Images are frequently stored as one-dimensional arrays.

Example:

```text
width = 5
```

```text
0  1  2  3  4
5  6  7  8  9
10 11 12 13 14
```

To convert:

```text
(x, y) → index
```

use:

```text
index = y * width + x
```

Example:

```text
width = 5
x = 2
y = 1
```

```text
index = 1 * 5 + 2
      = 7
```

Rust implementation:

```rust
fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}
```

---

# 11. Converting Index to Coordinates

To convert:

```text
index → (x, y)
```

use:

```text
x = index % width
y = index / width
```

Example:

```text
index = 7
width = 5
```

Result:

```text
x = 2
y = 1
```

Rust implementation:

```rust
fn index_to_xy(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}
```

---

# 12. Checking Whether a Point Is Inside a Rectangle

Condition:

```text
x >= rect.x
x < rect.x + rect.width

y >= rect.y
y < rect.y + rect.height
```

Rust implementation:

```rust
fn contains(rect: &Rect, point: Point) -> bool {
    point.x >= rect.x
        && point.x < rect.x + rect.width
        && point.y >= rect.y
        && point.y < rect.y + rect.height
}
```

---

# 13. Why Coordinate Geometry Matters

Coordinate geometry is used everywhere in Computer Vision.

## Object Detection

Bounding boxes:

```text
(x, y, width, height)
```

represent detected objects.

---

## Face Detection

Each detected face is represented by a rectangle.

---

## Image Cropping

Cropping requires selecting a rectangular region.

---

## Segmentation

Masks are indexed using:

```text
mask[y][x]
```

---

## Feature Detection

Feature points are stored as coordinates:

```text
(x, y)
```

Examples:

* Eye corners
* Nose tip
* Mouth corners
* Keypoints

---

# 14. Common Bugs

## Bug 1: Mixing x and y

Incorrect:

```rust
image[x][y]
```

Correct:

```rust
image[y][x]
```

---

## Bug 2: Mixing Width and Height

Incorrect:

```rust
for x in 0..height
```

Correct:

```rust
for x in 0..width
```

---

## Bug 3: Out-of-Bounds Rectangle

Always validate:

```text
x + width <= image_width

y + height <= image_height
```

before accessing pixels.

---

# 15. Rust Practice Exercises

## Exercise 1

Create:

```rust
struct Point
```

---

## Exercise 2

Create:

```rust
struct Rect
```

---

## Exercise 3

Implement:

```rust
point_distance()
```

---

## Exercise 4

Implement:

```rust
contains()
```

---

## Exercise 5

Implement:

```rust
xy_to_index()
```

---

## Exercise 6

Implement:

```rust
index_to_xy()
```

---

# 16. Image Processing Exercises

Use the Rust image crate:

```toml
image = "0.25"
```

## Exercise 1: Draw a Point

Draw a single colored pixel.

Expected output:

```text
Image with one point
```

---

## Exercise 2: Draw a Horizontal Line

Draw a line from left to right.

Expected output:

```text
Image with a horizontal line
```

---

## Exercise 3: Draw a Vertical Line

Draw a line from top to bottom.

Expected output:

```text
Image with a vertical line
```

---

## Exercise 4: Draw a Rectangle Border

Draw the border of a rectangle.

Expected output:

```text
Image with a rectangle
```

---

## Exercise 5: Crop an Image

Crop a rectangular region from an image.

Expected output:

```text
Cropped image
```

---

## Exercise 6: Translate an Image

Move all pixels by:

```text
(tx, ty)
```

Expected output:

```text
Translated image
```

---

# Summary

Coordinate Geometry is the foundation of Computer Vision.

Before moving to image filtering, feature extraction, OpenCV, object detection, or deep learning, you should be comfortable with:

* Coordinate systems
* Points
* Rectangles
* Distances
* Translation
* Scaling
* Pixel indexing
* Bounding boxes
* Cropping operations
* Drawing primitives

Mastering these concepts will make all future Computer Vision topics significantly easier to understand.
