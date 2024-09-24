use std::time::Instant;

use image::{ImageBuffer, Rgba};
use ndarray::Array3;
use tgaviewer;

fn main() {
    let image = image::open("dist/D0009.png").unwrap().to_rgba8();
    let arr = image_to_ndarray(image.clone());
    let start = Instant::now();
    let x = 3062;
    let y = 1102;
    tgaviewer::cv::flood_fill(&image, x, y, Rgba([255, 0, 0, 255]));
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn image_to_ndarray(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Array3<u8> {
    let (width, height) = img.dimensions();

    // Create an ndarray from the raw pixel data
    let res = Array3::from_shape_vec((height as usize, width as usize, 4), img.into_raw()).unwrap();

    println!("{:?}", res[[300, 300, 3]]);
    
    return res;
}
