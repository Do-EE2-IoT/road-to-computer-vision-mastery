# Example: `yuv.rs`

This example demonstrates a practical YUV workflow:

1. Load an RGB image (`bike.jpg`)
2. Convert RGB -> **NV12** (Y plane + interleaved UV)
3. Convert NV12 -> RGB
4. Visualize:
   - Original RGB
   - Reconstructed RGB
   - Y channel (luma)
   - Bright mask thresholded from Y
5. Print reconstruction error (MAE per channel)

## Run

From workspace root:

```bash
cargo run -p day_1 --example yuv --release
```

## Why this is useful

- Shows exactly how camera-friendly NV12 works.
- Explains why Y can be used for fast structure/brightness tasks.
- Makes quality loss from chroma subsampling visible and measurable.
