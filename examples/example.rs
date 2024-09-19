use std::time::Instant;

use image::Rgba;
use tgaviewer;

fn main() {
  let image = image::open("examples/30.tga").unwrap().to_rgba8();
  let start = Instant::now();
  tgaviewer::cv::flood_fill(&image, 0, 0, Rgba([255,0,0,255]));
  let duration = start.elapsed();
  println!("Execution time: {:?}", duration);
}
