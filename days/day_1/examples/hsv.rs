use std::error::Error;
use std::path::PathBuf;

use image::{ImageBuffer, Luma, Rgb};
use show_image::{ImageInfo, ImageView, create_window};

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r / 255.0;
    let g = g / 255.0;
    let b = b / 255.0;

    let cmax = r.max(g.max(b));
    let cmin = r.min(g.min(b));
    let delta = cmax - cmin;

    let mut h = if delta == 0.0 {
        0.0
    } else if cmax == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if cmax == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    if h < 0.0 {
        h += 360.0;
    }

    let s = if cmax == 0.0 {
        0.0
    } else {
        delta / cmax
    };

    let v = cmax;

    (h, s, v)
}

fn is_red(h: f32, s: f32, v: f32, r: u8, g: u8, b: u8) -> bool {
    // Red wraps around hue boundary, so we use 2 hue bands.
    let in_red_hue = (0.0..=12.0).contains(&h) || (345.0..=360.0).contains(&h);
    // Filter low-saturation / very dark pixels (common false positives).
    let good_sv = s >= 0.35 && v >= 0.20;
    // Additional RGB dominance check to reduce orange/brown leakage.
    let red_dominant = r as i16 > g as i16 + 15 && r as i16 > b as i16 + 15;

    in_red_hue && good_sv && red_dominant
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let img_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/images/red_ball.jpg");

    let img = image::open(&img_path).unwrap_or_else(|e| panic!("Error opening image at {}: {}", img_path.display(), e));
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();
    let mut highlighted_img = rgb_img.clone();
    let mut red_mask = ImageBuffer::new(width, height);

    let mut h_image = ImageBuffer::new(width, height);
    let mut s_image = ImageBuffer::new(width, height);
    let mut v_image = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgb_img.enumerate_pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;

        let (h, s, v) = rgb_to_hsv(r, g, b);

        let h_val = (h / 360.0 * 255.0) as u8;
        let s_val = (s * 255.0) as u8;
        let v_val = (v * 255.0) as u8;

        h_image.put_pixel(x, y, Luma([h_val]));
        s_image.put_pixel(x, y, Luma([s_val]));
        v_image.put_pixel(x, y, Luma([v_val]));

        if is_red(h, s, v, pixel[0], pixel[1], pixel[2]) {
            highlighted_img.put_pixel(x, y, Rgb([255, 0, 0]));
            red_mask.put_pixel(x, y, Luma([255]));
        } else {
            red_mask.put_pixel(x, y, Luma([0]));
        }
    }

    let win_rgb = create_window("RGB", Default::default())?;
    let win_highlight = create_window("Red Highlight", Default::default())?;
    let win_mask = create_window("Red Mask", Default::default())?;
    let win_h = create_window("Hue (H)", Default::default())?;
    let win_s = create_window("Saturation (S)", Default::default())?;
    let win_v = create_window("Value (V)", Default::default())?;

    let view_rgb = ImageView::new(ImageInfo::rgb8(width, height), rgb_img.as_raw());
    win_rgb.set_image("rgb", view_rgb)?;

    let highlighted_view = ImageView::new(ImageInfo::rgb8(width, height), highlighted_img.as_raw());
    win_highlight.set_image("highlighted", highlighted_view)?;

    let mask_view = ImageView::new(ImageInfo::mono8(width, height), red_mask.as_raw());
    win_mask.set_image("mask", mask_view)?;

    let view_h = ImageView::new(ImageInfo::mono8(width, height), h_image.as_raw());
    win_h.set_image("h", view_h)?;

    let view_s = ImageView::new(ImageInfo::mono8(width, height), s_image.as_raw());
    win_s.set_image("s", view_s)?;

    let view_v = ImageView::new(ImageInfo::mono8(width, height), v_image.as_raw());
    win_v.set_image("v", view_v)?;

    win_highlight.wait_until_destroyed()?;
    Ok(())
}
