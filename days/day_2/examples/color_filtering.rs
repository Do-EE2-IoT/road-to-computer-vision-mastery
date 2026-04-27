use std::error::Error;
use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb, RgbImage};
use show_image::{create_window, ImageInfo, ImageView};

struct BlueColorDetector {
    img: DynamicImage,
}

impl BlueColorDetector {
    fn new(path: PathBuf) -> Self {
        let img = image::open(&path)
            .unwrap_or_else(|e| panic!("Error opening image at {}: {}", path.display(), e));
        Self { img }
    }

    fn get_all_rgb_pixel(&self) -> RgbImage {
        self.img.to_rgb8()
    }

    fn get_dimension(&self) -> (u32, u32) {
        self.img.dimensions()
    }

    fn turn_px_rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

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

        let s = if cmax == 0.0 { 0.0 } else { delta / cmax };
        let v = cmax;

        (h, s, v)
    }

    fn is_blue(h: f32, s: f32, v: f32, r: u8, _g: u8, b: u8) -> bool {
        // Blue in HSV is typically around 200-240, and sea/sky often drifts toward cyan.
        let in_blue_hue = (170.0..=260.0).contains(&h);
        let good_sv = s >= 0.18 && v >= 0.12;
        // Keep a light RGB check to avoid obvious non-blue false positives.
        let blue_dominant = b as i16 >= r as i16 + 8;

        in_blue_hue && good_sv && blue_dominant
    }

    fn make_buffer_focus_blue_pixel(&self) -> (GrayImage, RgbImage, u64) {
        let (width, height) = self.get_dimension();
        let rgb_img = self.get_all_rgb_pixel();

        let mut mask = GrayImage::new(width, height);
        let mut highlight = rgb_img.clone();
        let mut count_white = 0_u64;

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let (h, s, v) = Self::turn_px_rgb_to_hsv(pixel[0], pixel[1], pixel[2]);

            if Self::is_blue(h, s, v, pixel[0], pixel[1], pixel[2]) {
                mask.put_pixel(x, y, Luma([255]));
                highlight.put_pixel(x, y, Rgb([0, 255, 255]));
                count_white += 1;
            } else {
                mask.put_pixel(x, y, Luma([0]));
            }
        }

        (mask, highlight, count_white)
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let path_img = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/images/Big_Blue_Ball.jpg");

    let blue_color_detector = BlueColorDetector::new(path_img);
    let rgb_img = blue_color_detector.get_all_rgb_pixel();
    let (width, height) = blue_color_detector.get_dimension();

    let (mask_raw, highlight_raw, raw_white) = blue_color_detector.make_buffer_focus_blue_pixel();

    println!("Blue mask pixels (raw): {}", raw_white);

    let win_rgb = create_window("Original RGB", Default::default())?;
    let win_mask = create_window("Blue Mask (Raw)", Default::default())?;
    let win_highlight = create_window("Highlight (Raw)", Default::default())?;

    win_rgb.set_image(
        "rgb",
        ImageView::new(ImageInfo::rgb8(width, height), rgb_img.as_raw()),
    )?;
    win_mask.set_image(
        "mask_raw",
        ImageView::new(ImageInfo::mono8(width, height), mask_raw.as_raw()),
    )?;
    win_highlight.set_image(
        "highlight_raw",
        ImageView::new(ImageInfo::rgb8(width, height), highlight_raw.as_raw()),
    )?;

    win_rgb.wait_until_destroyed()?;
    Ok(())
}
