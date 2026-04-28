use std::error::Error;
use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb, RgbImage};
use show_image::{ImageInfo, ImageView, create_window};

struct BlueColorDetector {
    img: DynamicImage,
}

impl BlueColorDetector {
    fn new(path: PathBuf) -> Self {
        let img = image::open(&path).unwrap_or_else(|e| panic!("Error opening image at {}: {}", path.display(), e));
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

        let s = if cmax == 0.0 {
            0.0
        } else {
            delta / cmax
        };
        let v = cmax;

        (h, s, v)
    }

    fn is_blue(h: f32, s: f32, v: f32, r: u8, _g: u8, b: u8) -> bool {
        // Blue + cyan-blue band, useful for sea/sky/blue objects.
        let in_blue_hue = (170.0..=260.0).contains(&h);
        let good_sv = s >= 0.18 && v >= 0.12;
        let blue_dominant = b as i16 >= r as i16 + 8;

        in_blue_hue && good_sv && blue_dominant
    }

    fn erode_3x3(input: &GrayImage) -> GrayImage {
        let (w, h) = input.dimensions();
        let mut out = GrayImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                if x == 0 || y == 0 || x + 1 >= w || y + 1 >= h {
                    out.put_pixel(x, y, Luma([0]));
                    continue;
                }

                let mut all_white = true;
                for ny in (y - 1)..=(y + 1) {
                    for nx in (x - 1)..=(x + 1) {
                        if input.get_pixel(nx, ny)[0] == 0 {
                            all_white = false;
                            break;
                        }
                    }
                    if !all_white {
                        break;
                    }
                }

                out.put_pixel(
                    x,
                    y,
                    Luma([if all_white {
                        255
                    } else {
                        0
                    }]),
                );
            }
        }

        out
    }

    fn dilate_3x3(input: &GrayImage) -> GrayImage {
        let (w, h) = input.dimensions();
        let mut out = GrayImage::new(w, h);

        for y in 0..h {
            for x in 0..w {
                if x == 0 || y == 0 || x + 1 >= w || y + 1 >= h {
                    out.put_pixel(x, y, Luma([0]));
                    continue;
                }

                let mut any_white = false;
                for ny in (y - 1)..=(y + 1) {
                    for nx in (x - 1)..=(x + 1) {
                        if input.get_pixel(nx, ny)[0] > 0 {
                            any_white = true;
                            break;
                        }
                    }
                    if any_white {
                        break;
                    }
                }

                out.put_pixel(
                    x,
                    y,
                    Luma([if any_white {
                        255
                    } else {
                        0
                    }]),
                );
            }
        }

        out
    }

    fn opening_3x3(input: &GrayImage) -> GrayImage {
        let e = Self::erode_3x3(input);
        Self::dilate_3x3(&e)
    }

    fn closing_3x3(input: &GrayImage) -> GrayImage {
        let d = Self::dilate_3x3(input);
        Self::erode_3x3(&d)
    }

    fn count_white_pixels(mask: &GrayImage) -> u64 {
        mask.pixels().map(|p| (p[0] > 0) as u64).sum()
    }

    fn make_buffer_focus_blue_pixel(&self) -> (GrayImage, RgbImage) {
        let (width, height) = self.get_dimension();
        let rgb_img = self.get_all_rgb_pixel();

        let mut mask = GrayImage::new(width, height);
        let mut highlight = rgb_img.clone();

        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            let (h, s, v) = Self::turn_px_rgb_to_hsv(pixel[0], pixel[1], pixel[2]);

            if Self::is_blue(h, s, v, pixel[0], pixel[1], pixel[2]) {
                mask.put_pixel(x, y, Luma([255]));
                highlight.put_pixel(x, y, Rgb([0, 255, 255]));
            } else {
                mask.put_pixel(x, y, Luma([0]));
            }
        }

        (mask, highlight)
    }

    fn make_highlight_from_mask(rgb_img: &RgbImage, mask: &GrayImage) -> RgbImage {
        let mut highlight = rgb_img.clone();
        for (x, y, p) in mask.enumerate_pixels() {
            if p[0] > 0 {
                highlight.put_pixel(x, y, Rgb([0, 255, 255]));
            }
        }
        highlight
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let path_img = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../resources/images/Big_Blue_Ball.jpg");

    let blue_color_detector = BlueColorDetector::new(path_img);
    let rgb_img = blue_color_detector.get_all_rgb_pixel();
    let (width, height) = blue_color_detector.get_dimension();

    let (mask_raw, highlight_raw) = blue_color_detector.make_buffer_focus_blue_pixel();

    let mask_open = BlueColorDetector::opening_3x3(&mask_raw);
    let mask_clean = BlueColorDetector::closing_3x3(&mask_open);
    let highlight_clean = BlueColorDetector::make_highlight_from_mask(&rgb_img, &mask_clean);

    let raw_white = BlueColorDetector::count_white_pixels(&mask_raw);
    let clean_white = BlueColorDetector::count_white_pixels(&mask_clean);

    println!("Blue mask pixels (raw):   {}", raw_white);
    println!("Blue mask pixels (clean): {}", clean_white);

    let win_rgb = create_window("Original RGB", Default::default())?;
    let win_mask_raw = create_window("Blue Mask (Raw)", Default::default())?;
    let win_mask_clean = create_window("Blue Mask (Clean)", Default::default())?;
    let win_hl_raw = create_window("Highlight (Raw)", Default::default())?;
    let win_hl_clean = create_window("Highlight (Clean)", Default::default())?;

    win_rgb.set_image("rgb", ImageView::new(ImageInfo::rgb8(width, height), rgb_img.as_raw()))?;
    win_mask_raw.set_image("mask_raw", ImageView::new(ImageInfo::mono8(width, height), mask_raw.as_raw()))?;
    win_mask_clean.set_image("mask_clean", ImageView::new(ImageInfo::mono8(width, height), mask_clean.as_raw()))?;
    win_hl_raw.set_image("highlight_raw", ImageView::new(ImageInfo::rgb8(width, height), highlight_raw.as_raw()))?;
    win_hl_clean.set_image("highlight_clean", ImageView::new(ImageInfo::rgb8(width, height), highlight_clean.as_raw()))?;

    win_rgb.wait_until_destroyed()?;
    Ok(())
}
