
use std::path::PathBuf;

fn main() {
    let img_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../resources/images/bike.jpg");

    let img = image::open(&img_path)
        .unwrap_or_else(|e| panic!("Error opening image at {}: {}", img_path.display(), e));

    println!(
        "Loaded image successfully: {}x{}",
        img.width(),
        img.height()
    );
}
