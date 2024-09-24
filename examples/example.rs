use std::time::Instant;

use image::Rgba;
use tgaviewer;

fn main() {
    let image = image::open("dist/D0009.png").unwrap().to_rgba8();
    let start = Instant::now();
    let x = 3062;
    let y = 1102;
    tgaviewer::cv::flood_fill(&image, x, y, Rgba([255, 0, 0, 255]));
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
