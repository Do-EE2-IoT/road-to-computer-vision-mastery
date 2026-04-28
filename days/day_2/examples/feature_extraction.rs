use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb, RgbImage};
use show_image::{create_window, ImageInfo, ImageView};

#[derive(Clone, Copy, Debug)]
struct BoundingBox {
    x_min: u32,
    y_min: u32,
    x_max: u32,
    y_max: u32,
    area_px: u32,
}

struct BlueColorDetector {
    img: DynamicImage,
}

impl BlueColorDetector {
    fn new(path: PathBuf) -> Self {
        let img = image::open(&path)
            .unwrap_or_else(|e| panic!("Error opening image at {}: {}", path.display(), e));
        Self { img }
    }

    fn rgb_image(&self) -> RgbImage {
        self.img.to_rgb8()
    }

    fn dimensions(&self) -> (u32, u32) {
        self.img.dimensions()
    }

    fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
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

    fn is_blue(h: f32, s: f32, v: f32, r: u8, b: u8) -> bool {
        let in_blue_hue = (170.0..=260.0).contains(&h);
        let good_sv = s >= 0.18 && v >= 0.12;
        let blue_dominant = b as i16 >= r as i16 + 8;

        in_blue_hue && good_sv && blue_dominant
    }

    fn raw_blue_mask(&self) -> GrayImage {
        let (width, height) = self.dimensions();
        let rgb = self.rgb_image();

        let mut mask = GrayImage::new(width, height);
        for (x, y, p) in rgb.enumerate_pixels() {
            let (h, s, v) = Self::rgb_to_hsv(p[0], p[1], p[2]);
            if Self::is_blue(h, s, v, p[0], p[2]) {
                mask.put_pixel(x, y, Luma([255]));
            } else {
                mask.put_pixel(x, y, Luma([0]));
            }
        }

        mask
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

                out.put_pixel(x, y, Luma([if all_white { 255 } else { 0 }]));
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

                out.put_pixel(x, y, Luma([if any_white { 255 } else { 0 }]));
            }
        }

        out
    }

    fn clean_mask(input: &GrayImage) -> GrayImage {
        let opened = Self::dilate_3x3(&Self::erode_3x3(input));
        Self::erode_3x3(&Self::dilate_3x3(&opened))
    }

    fn largest_component_bbox(mask: &GrayImage) -> Option<BoundingBox> {
        let (w, h) = mask.dimensions();
        let w_usize = w as usize;
        let h_usize = h as usize;
        let mut visited = vec![false; w_usize * h_usize];

        let idx = |x: u32, y: u32| -> usize { y as usize * w_usize + x as usize };

        let mut best: Option<BoundingBox> = None;

        for y in 0..h {
            for x in 0..w {
                if mask.get_pixel(x, y)[0] == 0 || visited[idx(x, y)] {
                    continue;
                }

                let mut q = VecDeque::new();
                q.push_back((x, y));
                visited[idx(x, y)] = true;

                let mut min_x = x;
                let mut min_y = y;
                let mut max_x = x;
                let mut max_y = y;
                let mut area = 0_u32;

                while let Some((cx, cy)) = q.pop_front() {
                    area += 1;
                    min_x = min_x.min(cx);
                    min_y = min_y.min(cy);
                    max_x = max_x.max(cx);
                    max_y = max_y.max(cy);

                    let neighbors = [
                        (cx.wrapping_sub(1), cy),
                        (cx + 1, cy),
                        (cx, cy.wrapping_sub(1)),
                        (cx, cy + 1),
                    ];

                    for (nx, ny) in neighbors {
                        if nx >= w || ny >= h {
                            continue;
                        }
                        let ni = idx(nx, ny);
                        if visited[ni] || mask.get_pixel(nx, ny)[0] == 0 {
                            continue;
                        }
                        visited[ni] = true;
                        q.push_back((nx, ny));
                    }
                }

                let candidate = BoundingBox {
                    x_min: min_x,
                    y_min: min_y,
                    x_max: max_x,
                    y_max: max_y,
                    area_px: area,
                };

                match best {
                    None => best = Some(candidate),
                    Some(prev) if candidate.area_px > prev.area_px => best = Some(candidate),
                    _ => {}
                }
            }
        }

        best
    }

    fn draw_bbox(img: &mut RgbImage, bbox: BoundingBox, color: Rgb<u8>, thickness: u32) {
        let (w, h) = img.dimensions();
        if w == 0 || h == 0 {
            return;
        }

        let t_max = thickness.max(1);
        for t in 0..t_max {
            let x0 = bbox.x_min.saturating_sub(t);
            let y0 = bbox.y_min.saturating_sub(t);
            let x1 = (bbox.x_max + t).min(w - 1);
            let y1 = (bbox.y_max + t).min(h - 1);

            for x in x0..=x1 {
                img.put_pixel(x, y0, color);
                img.put_pixel(x, y1, color);
            }
            for y in y0..=y1 {
                img.put_pixel(x0, y, color);
                img.put_pixel(x1, y, color);
            }
        }
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/images/blue_balls.png");

    let detector = BlueColorDetector::new(path);
    let rgb = detector.rgb_image();
    let (w, h) = detector.dimensions();

    let mask_raw = detector.raw_blue_mask();
    let mask_clean = BlueColorDetector::clean_mask(&mask_raw);

    let mut bbox_img = rgb.clone();
    let bbox = BlueColorDetector::largest_component_bbox(&mask_clean);
    if let Some(b) = bbox {
        BlueColorDetector::draw_bbox(&mut bbox_img, b, Rgb([255, 0, 0]), 3);
        println!(
            "Bounding box: x_min={}, y_min={}, x_max={}, y_max={}, area_px={}",
            b.x_min, b.y_min, b.x_max, b.y_max, b.area_px
        );
    } else {
        println!("No blue object found to draw bounding box.");
    }

    let win_rgb = create_window("Original RGB", Default::default())?;
    let win_mask = create_window("Blue Mask (Clean)", Default::default())?;
    let win_bbox = create_window("Bounding Box (Red)", Default::default())?;

    win_rgb.set_image("rgb", ImageView::new(ImageInfo::rgb8(w, h), rgb.as_raw()))?;
    win_mask.set_image(
        "mask_clean",
        ImageView::new(ImageInfo::mono8(w, h), mask_clean.as_raw()),
    )?;
    win_bbox.set_image(
        "bbox",
        ImageView::new(ImageInfo::rgb8(w, h), bbox_img.as_raw()),
    )?;

    win_bbox.wait_until_destroyed()?;
    Ok(())
}
