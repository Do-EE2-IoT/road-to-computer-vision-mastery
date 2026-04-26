use std::error::Error;
use std::path::PathBuf;

use image::{Rgb, RgbImage};
use show_image::{ImageInfo, ImageView, create_window};

fn bgr24_to_rgb_image(raw_bgr: &[u8], width: u32, height: u32) -> RgbImage {
    let mut out = RgbImage::new(width, height);
    for (i, pixel) in out.pixels_mut().enumerate() {
        let idx = i * 3;
        let b = raw_bgr[idx];
        let g = raw_bgr[idx + 1];
        let r = raw_bgr[idx + 2];
        *pixel = Rgb([r, g, b]);
    }
    out
}

fn rgb_to_rgb565_le(rgb: &RgbImage) -> Vec<u8> {
    let mut out = Vec::with_capacity((rgb.width() * rgb.height() * 2) as usize);

    for p in rgb.pixels() {
        let r5 = (p[0] as u16 * 31 / 255) & 0x1f;
        let g6 = (p[1] as u16 * 63 / 255) & 0x3f;
        let b5 = (p[2] as u16 * 31 / 255) & 0x1f;

        let packed = (r5 << 11) | (g6 << 5) | b5;
        out.push((packed & 0xff) as u8); // little-endian low byte
        out.push((packed >> 8) as u8); // little-endian high byte
    }

    out
}

fn rgb565_le_to_rgb(raw565: &[u8], width: u32, height: u32) -> RgbImage {
    let mut out = RgbImage::new(width, height);

    for (i, pixel) in out.pixels_mut().enumerate() {
        let idx = i * 2;
        let packed = (raw565[idx] as u16) | ((raw565[idx + 1] as u16) << 8);

        let r5 = (packed >> 11) & 0x1f;
        let g6 = (packed >> 5) & 0x3f;
        let b5 = packed & 0x1f;

        let r8 = (r5 * 255 / 31) as u8;
        let g8 = (g6 * 255 / 63) as u8;
        let b8 = (b5 * 255 / 31) as u8;

        *pixel = Rgb([r8, g8, b8]);
    }

    out
}

fn mae_rgb(a: &RgbImage, b: &RgbImage) -> (f64, f64, f64) {
    let mut sum = [0_u64; 3];

    for (pa, pb) in a.pixels().zip(b.pixels()) {
        sum[0] += (pa[0] as i32 - pb[0] as i32).unsigned_abs() as u64;
        sum[1] += (pa[1] as i32 - pb[1] as i32).unsigned_abs() as u64;
        sum[2] += (pa[2] as i32 - pb[2] as i32).unsigned_abs() as u64;
    }

    let n = (a.width() as u64) * (a.height() as u64);
    (
        sum[0] as f64 / n as f64,
        sum[1] as f64 / n as f64,
        sum[2] as f64 / n as f64,
    )
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let img_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/images/bike.jpg");
    let img = image::open(&img_path)
        .unwrap_or_else(|e| panic!("Error opening image at {}: {}", img_path.display(), e))
        .to_rgb8();

    let (width, height) = img.dimensions();

    // 1) RGB24 raw bytes (packed: R,G,B,R,G,B,...)
    let raw_rgb24 = img.as_raw().clone();
    let rgb_from_rgb24 = image::ImageBuffer::from_raw(width, height, raw_rgb24.clone())
        .expect("Invalid RGB24 raw buffer size");

    // 2) Build BGR24 raw bytes from original, then decode back to RGB
    let mut raw_bgr24 = Vec::with_capacity(raw_rgb24.len());
    for p in img.pixels() {
        raw_bgr24.push(p[2]); // B
        raw_bgr24.push(p[1]); // G
        raw_bgr24.push(p[0]); // R
    }
    let rgb_from_bgr24 = bgr24_to_rgb_image(&raw_bgr24, width, height);

    // 3) RGB565 packed raw -> back to RGB888
    let raw_rgb565 = rgb_to_rgb565_le(&img);
    let rgb_from_rgb565 = rgb565_le_to_rgb(&raw_rgb565, width, height);

    // Report quick quality stats
    let (mae_r1, mae_g1, mae_b1) = mae_rgb(&img, &rgb_from_rgb24);
    let (mae_r2, mae_g2, mae_b2) = mae_rgb(&img, &rgb_from_bgr24);
    let (mae_r3, mae_g3, mae_b3) = mae_rgb(&img, &rgb_from_rgb565);

    println!("RGB24 -> RGB MAE: R={:.2}, G={:.2}, B={:.2}", mae_r1, mae_g1, mae_b1);
    println!("BGR24 -> RGB MAE: R={:.2}, G={:.2}, B={:.2}", mae_r2, mae_g2, mae_b2);
    println!("RGB565 -> RGB MAE: R={:.2}, G={:.2}, B={:.2}", mae_r3, mae_g3, mae_b3);

    // Print first bytes to make raw layout tangible
    println!("First 12 bytes RGB24: {:?}", &raw_rgb24[..12.min(raw_rgb24.len())]);
    println!("First 12 bytes BGR24: {:?}", &raw_bgr24[..12.min(raw_bgr24.len())]);
    println!("First 12 bytes RGB565: {:?}", &raw_rgb565[..12.min(raw_rgb565.len())]);

    let win_original = create_window("Original RGB", Default::default())?;
    let win_rgb24 = create_window("From RGB24 Raw", Default::default())?;
    let win_bgr24 = create_window("From BGR24 Raw", Default::default())?;
    let win_rgb565 = create_window("From RGB565 Raw", Default::default())?;

    win_original.set_image(
        "orig",
        ImageView::new(ImageInfo::rgb8(width, height), img.as_raw()),
    )?;
    win_rgb24.set_image(
        "rgb24",
        ImageView::new(ImageInfo::rgb8(width, height), rgb_from_rgb24.as_raw()),
    )?;
    win_bgr24.set_image(
        "bgr24",
        ImageView::new(ImageInfo::rgb8(width, height), rgb_from_bgr24.as_raw()),
    )?;
    win_rgb565.set_image(
        "rgb565",
        ImageView::new(ImageInfo::rgb8(width, height), rgb_from_rgb565.as_raw()),
    )?;

    win_original.wait_until_destroyed()?;
    Ok(())
}
