use std::error::Error;
use std::path::PathBuf;

use image::{ImageBuffer, Luma, Rgb, RgbImage};
use show_image::{ImageInfo, ImageView, create_window};

fn clamp_u8(v: f32) -> u8 {
    v.max(0.0).min(255.0) as u8
}

// BT.601 full-range RGB -> YUV
fn rgb_to_yuv_bt601_full(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let r = r as f32;
    let g = g as f32;
    let b = b as f32;

    let y = 0.299 * r + 0.587 * g + 0.114 * b;
    let u = -0.168_736 * r - 0.331_264 * g + 0.5 * b + 128.0;
    let v = 0.5 * r - 0.418_688 * g - 0.081_312 * b + 128.0;

    (clamp_u8(y), clamp_u8(u), clamp_u8(v))
}

// BT.601 full-range YUV -> RGB
fn yuv_to_rgb_bt601_full(y: u8, u: u8, v: u8) -> (u8, u8, u8) {
    let y = y as f32;
    let u = u as f32 - 128.0;
    let v = v as f32 - 128.0;

    let r = y + 1.402 * v;
    let g = y - 0.344_136 * u - 0.714_136 * v;
    let b = y + 1.772 * u;

    (clamp_u8(r), clamp_u8(g), clamp_u8(b))
}

fn rgb_to_nv12(rgb: &RgbImage) -> (Vec<u8>, Vec<u8>) {
    let (width, height) = rgb.dimensions();
    let w = width as usize;
    let h = height as usize;

    let mut y_plane = vec![0_u8; w * h];
    let mut uv_plane = vec![0_u8; w * h / 2];

    // Fill Y plane
    for y in 0..height {
        for x in 0..width {
            let p = rgb.get_pixel(x, y);
            let (yy, _, _) = rgb_to_yuv_bt601_full(p[0], p[1], p[2]);
            y_plane[y as usize * w + x as usize] = yy;
        }
    }

    // 4:2:0 chroma subsampling (average U/V over 2x2 block)
    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            let mut sum_u = 0.0_f32;
            let mut sum_v = 0.0_f32;

            for dy in 0..2 {
                for dx in 0..2 {
                    let p = rgb.get_pixel(x + dx, y + dy);
                    let (_, u, v) = rgb_to_yuv_bt601_full(p[0], p[1], p[2]);
                    sum_u += u as f32;
                    sum_v += v as f32;
                }
            }

            let u_avg = clamp_u8(sum_u / 4.0);
            let v_avg = clamp_u8(sum_v / 4.0);

            let uv_row = (y / 2) as usize;
            let uv_col = x as usize;
            let uv_idx = uv_row * w + uv_col;

            uv_plane[uv_idx] = u_avg;
            uv_plane[uv_idx + 1] = v_avg;
        }
    }

    (y_plane, uv_plane)
}

fn rgb_to_nv21(rgb: &RgbImage) -> (Vec<u8>, Vec<u8>) {
    let (width, height) = rgb.dimensions();
    let w = width as usize;
    let h = height as usize;

    let mut y_plane = vec![0_u8; w * h];
    let mut vu_plane = vec![0_u8; w * h / 2];

    for y in 0..h {
        for x in 0..w {
            let pixel = rgb.get_pixel(x as u32, y as u32);
            let (yy, _, _) = rgb_to_yuv_bt601_full(pixel[0], pixel[1], pixel[2]);
            y_plane[y * w + x] = yy;
        }
    }

    for y in (0..h).step_by(2) {
        for x in (0..w).step_by(2) {
            let mut sum_v = 0.0_f32;
            let mut sum_u = 0.0_f32;

            for dy in 0..2 {
                for dx in 0..2 {
                    let pixel = rgb.get_pixel((x + dx) as u32, (y + dy) as u32);
                    let (_, u, v) = rgb_to_yuv_bt601_full(pixel[0], pixel[1], pixel[2]);
                    sum_u += u as f32;
                    sum_v += v as f32;
                }
            }

            let v_avg = clamp_u8(sum_v / 4.0);
            let u_avg = clamp_u8(sum_u / 4.0);

            let vu_row = y / 2;
            let vu_col = x;
            let vu_idx = vu_row * w + vu_col;

            // NV21 stores VU interleaved (not UV).
            vu_plane[vu_idx] = v_avg;
            vu_plane[vu_idx + 1] = u_avg;
        }
    }

    (y_plane, vu_plane)
}

fn nv21_to_rgb(width: u32, height: u32, y_plane: &[u8], vu_plane: &[u8]) -> RgbImage {
    let w = width as usize;
    let mut out = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let yy = y_plane[y as usize * w + x as usize];

            let vu_row = (y / 2) as usize;
            let vu_col = ((x / 2) * 2) as usize;
            let vu_idx = vu_row * w + vu_col;

            let v = vu_plane[vu_idx];
            let u = vu_plane[vu_idx + 1];

            let (r, g, b) = yuv_to_rgb_bt601_full(yy, u, v);
            out.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    out
}

fn nv12_to_rgb(width: u32, height: u32, y_plane: &[u8], uv_plane: &[u8]) -> RgbImage {
    let w = width as usize;
    let mut out = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let yv = y_plane[y as usize * w + x as usize];

            let uv_row = (y / 2) as usize;
            let uv_col = ((x / 2) * 2) as usize;
            let uv_idx = uv_row * w + uv_col;

            let u = uv_plane[uv_idx];
            let v = uv_plane[uv_idx + 1];

            let (r, g, b) = yuv_to_rgb_bt601_full(yv, u, v);
            out.put_pixel(x, y, Rgb([r, g, b]));
        }
    }

    out
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let img_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/images/bike.jpg");
    let img = image::open(&img_path).unwrap_or_else(|e| panic!("Error opening image at {}: {}", img_path.display(), e));

    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    // NV12 is 4:2:0, so use even dimensions for simple demo.
    let even_w = width - (width % 2);
    let even_h = height - (height % 2);
    let rgb = image::imageops::crop_imm(&rgb, 0, 0, even_w, even_h).to_image();

    let (y_plane, uv_plane) = rgb_to_nv12(&rgb);
    let recon = nv12_to_rgb(even_w, even_h, &y_plane, &uv_plane);
    let (y_plane_nv21, vu_plane_nv21) = rgb_to_nv21(&rgb);
    let recon_nv21 = nv21_to_rgb(even_w, even_h, &y_plane_nv21, &vu_plane_nv21);

    let mut y_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(even_w, even_h);
    let mut bright_mask: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(even_w, even_h);

    for y in 0..even_h {
        for x in 0..even_w {
            let yy = y_plane[y as usize * even_w as usize + x as usize];
            y_img.put_pixel(x, y, Luma([yy]));
            bright_mask.put_pixel(
                x,
                y,
                Luma([if yy > 170 {
                    255
                } else {
                    0
                }]),
            );
        }
    }

    let mut sum_abs = [0_u64; 3];
    for (a, b) in rgb.pixels().zip(recon.pixels()) {
        sum_abs[0] += (a[0] as i32 - b[0] as i32).unsigned_abs() as u64;
        sum_abs[1] += (a[1] as i32 - b[1] as i32).unsigned_abs() as u64;
        sum_abs[2] += (a[2] as i32 - b[2] as i32).unsigned_abs() as u64;
    }
    let mut sum_abs_nv21 = [0_u64; 3];
    for (a, b) in rgb.pixels().zip(recon_nv21.pixels()) {
        sum_abs_nv21[0] += (a[0] as i32 - b[0] as i32).unsigned_abs() as u64;
        sum_abs_nv21[1] += (a[1] as i32 - b[1] as i32).unsigned_abs() as u64;
        sum_abs_nv21[2] += (a[2] as i32 - b[2] as i32).unsigned_abs() as u64;
    }

    let n = (even_w as u64) * (even_h as u64);
    println!(
        "Mean Absolute Error after RGB -> NV12 -> RGB: R={:.2}, G={:.2}, B={:.2}",
        sum_abs[0] as f64 / n as f64,
        sum_abs[1] as f64 / n as f64,
        sum_abs[2] as f64 / n as f64
    );
    println!(
        "Mean Absolute Error after RGB -> NV21 -> RGB: R={:.2}, G={:.2}, B={:.2}",
        sum_abs_nv21[0] as f64 / n as f64,
        sum_abs_nv21[1] as f64 / n as f64,
        sum_abs_nv21[2] as f64 / n as f64
    );

    let win_rgb = create_window("Original RGB", Default::default())?;
    let win_recon = create_window("Reconstructed from NV12", Default::default())?;
    let win_recon_nv21 = create_window("Reconstructed from NV21", Default::default())?;
    let win_y = create_window("Y channel (Luma)", Default::default())?;
    let win_mask = create_window("Bright Mask from Y", Default::default())?;

    win_rgb.set_image("rgb", ImageView::new(ImageInfo::rgb8(even_w, even_h), rgb.as_raw()))?;
    win_recon.set_image("recon", ImageView::new(ImageInfo::rgb8(even_w, even_h), recon.as_raw()))?;
    win_recon_nv21.set_image("recon_nv21", ImageView::new(ImageInfo::rgb8(even_w, even_h), recon_nv21.as_raw()))?;
    win_y.set_image("y", ImageView::new(ImageInfo::mono8(even_w, even_h), y_img.as_raw()))?;
    win_mask.set_image("mask", ImageView::new(ImageInfo::mono8(even_w, even_h), bright_mask.as_raw()))?;

    win_rgb.wait_until_destroyed()?;
    Ok(())
}
