# Basic Algebra for Computer Vision

This note explains the minimum algebra needed before working with image processing and computer vision.

The goal is not to study algebra deeply. The goal is to understand simple mathematical tools that appear again and again when manipulating pixels, coordinates, colors, masks, and image values.

---

## 1. Variables

### What is a variable?

A variable is a name that stores a value.

Example:

```text
x = 10
y = 20
```

Here:

- `x` stores `10`
- `y` stores `20`

In programming, variables help us reuse values instead of writing raw numbers everywhere.

Rust example:

```rust
let x = 10;
let y = 20;
let sum = x + y;
```

### Computer vision example

For an image:

```text
width = 640
height = 480
x = 100
y = 50
```

Here:

- `width` is image width
- `height` is image height
- `(x, y)` is a pixel position

Another example:

```text
r = 255
g = 120
b = 30
```

These variables represent one RGB pixel.

### What to remember

Variables make formulas readable.

Instead of writing:

```text
(255 + 120 + 30) / 3
```

we can write:

```text
(r + g + b) / 3
```

This is easier to understand.

---

## 2. Functions

### What is a function?

A function takes input and returns output.

Simple form:

```text
output = f(input)
```

Example:

```text
f(x) = x + 2
```

If:

```text
x = 5
```

Then:

```text
f(5) = 5 + 2 = 7
```

Rust example:

```rust
fn add_two(x: i32) -> i32 {
    x + 2
}
```

### Computer vision example

Brightness adjustment can be seen as a function:

```text
new_pixel = f(pixel)
f(pixel) = pixel + brightness_value
```

Example:

```text
pixel = 100
brightness_value = 30

new_pixel = 100 + 30 = 130
```

Grayscale conversion is also a function:

```text
gray = f(r, g, b)
gray = (r + g + b) / 3
```

Example:

```text
r = 90
g = 120
b = 150

gray = (90 + 120 + 150) / 3
gray = 120
```

### What to remember

In image processing, almost every operation is a function:

```text
input pixel -> output pixel
input image -> output image
input color -> output mask value
```

---

## 3. Linear Equation

### What is a linear equation?

A linear equation has this common form:

```text
y = a * x + b
```

Where:

- `x` is input
- `y` is output
- `a` controls how strongly `x` affects `y`
- `b` shifts the output up or down

Example:

```text
y = 2 * x + 5
```

If:

```text
x = 10
```

Then:

```text
y = 2 * 10 + 5
y = 25
```

### Computer vision example: contrast and brightness

A simple pixel transformation can be written as:

```text
new_pixel = contrast * pixel + brightness
```

Example:

```text
pixel = 100
contrast = 1.2
brightness = 10

new_pixel = 1.2 * 100 + 10
new_pixel = 130
```

This is a linear equation.

### What does `a` do?

In:

```text
y = a * x + b
```

If `a` is bigger, output changes faster.

Example:

```text
y = 1 * x
x = 100 -> y = 100

y = 2 * x
x = 100 -> y = 200
```

In images, this is similar to changing contrast.

### What does `b` do?

`b` adds a constant shift.

Example:

```text
y = x + 30
x = 100 -> y = 130
```

In images, this is similar to increasing brightness.

---

## 4. Slope

### What is slope?

Slope measures how fast `y` changes when `x` changes.

Formula:

```text
slope = change_in_y / change_in_x
```

Or:

```text
slope = (y2 - y1) / (x2 - x1)
```

Example:

```text
point1 = (0, 10)
point2 = (5, 20)

slope = (20 - 10) / (5 - 0)
slope = 10 / 5
slope = 2
```

This means:

```text
when x increases by 1, y increases by 2
```

### Computer vision example: intensity change

Imagine two neighboring pixels:

```text
pixel_left = 50
pixel_right = 200
```

The change is:

```text
change = 200 - 50 = 150
```

This is a strong change.

In image processing, a strong local change often means an edge.

Example:

```text
left pixel  = dark
right pixel = bright
```

The boundary between them may be an edge.

### What to remember

Slope is the idea behind:

- edge detection
- gradients
- contrast changes
- local intensity change

You do not need advanced calculus yet. You only need to understand:

```text
big change between nearby values = important visual change
```

---

## 5. Ratio

### What is a ratio?

A ratio compares two quantities.

Example:

```text
ratio = a / b
```

If:

```text
a = 10
b = 5
```

Then:

```text
ratio = 10 / 5 = 2
```

This means `a` is 2 times `b`.

### Computer vision example: aspect ratio

For a bounding box:

```text
width = 200
height = 100
```

Aspect ratio:

```text
aspect_ratio = width / height
aspect_ratio = 200 / 100
aspect_ratio = 2.0
```

This means the box is twice as wide as it is tall.

Aspect ratio is useful for object analysis.

Example:

```text
ball-like object -> aspect_ratio close to 1.0
wide object      -> aspect_ratio > 1.0
tall object      -> aspect_ratio < 1.0
```

### Another example: resize ratio

If original image width is `1000`, and new width is `500`:

```text
scale_ratio = 500 / 1000
scale_ratio = 0.5
```

The image is resized to 50% of its original width.

---

## 6. Percentage

### What is percentage?

Percentage means "per 100".

Formula:

```text
percentage = part / total * 100
```

Example:

```text
part = 25
total = 100

percentage = 25 / 100 * 100
percentage = 25%
```

### Computer vision example: mask coverage

Suppose an image has:

```text
total_pixels = 10000
mask_pixels = 2500
```

Mask coverage:

```text
coverage = mask_pixels / total_pixels * 100
coverage = 2500 / 10000 * 100
coverage = 25%
```

This means the mask covers 25% of the image.

This is useful when checking if an object is too small or too large.

### Another example: detection threshold

Suppose a model or rule gives:

```text
confidence = 0.85
```

As percentage:

```text
confidence_percent = 0.85 * 100 = 85%
```

### What to remember

Percentage helps make values easier to understand:

```text
0.25 -> 25%
0.85 -> 85%
1.00 -> 100%
```

---

## 7. Range Mapping

### What is range mapping?

Range mapping converts a value from one range into another range.

Common example:

```text
old range: 0..255
new range: 0.0..1.0
```

Formula:

```text
normalized = (x - old_min) / (old_max - old_min)
mapped = normalized * (new_max - new_min) + new_min
```

### Example: map pixel to normalized value

```text
x = 128
old_min = 0
old_max = 255
new_min = 0.0
new_max = 1.0
```

Step 1:

```text
normalized = (128 - 0) / (255 - 0)
normalized = 128 / 255
normalized ≈ 0.502
```

Step 2:

```text
mapped = 0.502 * (1.0 - 0.0) + 0.0
mapped = 0.502
```

So:

```text
128 in 0..255 becomes about 0.502 in 0.0..1.0
```

### Example: map normalized value back to pixel

```text
x = 0.5
old range = 0.0..1.0
new range = 0..255
```

```text
mapped = 0.5 * (255 - 0) + 0
mapped = 127.5
```

After rounding:

```text
pixel = 128
```

### Why it matters in CV

Range mapping is used for:

- pixel normalization
- visualization
- contrast stretching
- converting image data to model input
- converting floating-point output back to displayable image

---

## 8. Clamping

### What is clamping?

Clamping limits a value to stay inside a range.

Formula:

```text
clamped = min(max(value, low), high)
```

If the value is too small, it becomes `low`.

If the value is too large, it becomes `high`.

### Example

Range:

```text
0..255
```

Value:

```text
300
```

After clamping:

```text
clamp(300, 0, 255) = 255
```

Another value:

```text
-20
```

After clamping:

```text
clamp(-20, 0, 255) = 0
```

### Computer vision example: brightness

Suppose:

```text
pixel = 240
brightness = 30
```

Without clamping:

```text
new_pixel = 240 + 30 = 270
```

But pixel values must stay in `0..255`.

With clamping:

```text
new_pixel = clamp(270, 0, 255)
new_pixel = 255
```

### What to remember

Always clamp after operations that can exceed pixel range.

Common examples:

- brightness
- contrast
- sharpen
- convolution
- adding noise

---

## 9. Absolute Value

### What is absolute value?

Absolute value removes the sign of a number.

Formula:

```text
abs(x)
```

Examples:

```text
abs(10) = 10
abs(-10) = 10
abs(0) = 0
```

### Why it matters

Sometimes we only care about the size of a difference, not the direction.

Example:

```text
a = 50
b = 200

difference = b - a = 150
```

Reverse order:

```text
a = 200
b = 50

difference = b - a = -150
```

But the size of the difference is the same:

```text
abs(-150) = 150
```

### Computer vision example: edge strength

Two neighboring pixels:

```text
left = 50
right = 200
```

Difference:

```text
diff = right - left = 150
```

Another case:

```text
left = 200
right = 50
diff = right - left = -150
```

For edge strength, both are strong changes.

So we use:

```text
edge_strength = abs(right - left)
```

Result:

```text
edge_strength = 150
```

### What to remember

Use absolute value when direction does not matter, only magnitude matters.

---

## 10. Power and Square Root

### What is power?

Power means multiplying a number by itself.

Example:

```text
x^2 = x * x
```

If:

```text
x = 5
```

Then:

```text
x^2 = 5 * 5 = 25
```

### What is square root?

Square root is the reverse of squaring.

Example:

```text
sqrt(25) = 5
```

Because:

```text
5 * 5 = 25
```

### Computer vision example: distance between colors

RGB colors can be treated as 3D points:

```text
color_a = [r1, g1, b1]
color_b = [r2, g2, b2]
```

Distance:

```text
dr = r1 - r2
dg = g1 - g2
db = b1 - b2

distance = sqrt(dr^2 + dg^2 + db^2)
```

Example:

```text
color_a = [255, 0, 0]
color_b = [250, 10, 10]

dr = 255 - 250 = 5
dg = 0 - 10 = -10
db = 0 - 10 = -10

distance = sqrt(5^2 + (-10)^2 + (-10)^2)
distance = sqrt(25 + 100 + 100)
distance = sqrt(225)
distance = 15
```

This means `color_b` is close to pure red.

### Another example: distance between points

Two points:

```text
p1 = (10, 20)
p2 = (13, 24)
```

Distance:

```text
dx = 13 - 10 = 3
dy = 24 - 20 = 4

distance = sqrt(dx^2 + dy^2)
distance = sqrt(3^2 + 4^2)
distance = sqrt(9 + 16)
distance = sqrt(25)
distance = 5
```

### What to remember

Power and square root are used in:

- vector length
- color distance
- point distance
- edge magnitude
- standard deviation

---

## Summary

Minimum things to remember:

- `Variables` store values.
- `Functions` convert input to output.
- `Linear equations` describe simple transformations.
- `Slope` describes how fast values change.
- `Ratio` compares two values.
- `Percentage` expresses a ratio per 100.
- `Range mapping` converts values from one range to another.
- `Clamping` keeps values inside a valid range.
- `Absolute value` measures difference size without direction.
- `Power and square root` are used for distance and magnitude.

In computer vision, these ideas appear constantly because an image is just structured numeric data.
