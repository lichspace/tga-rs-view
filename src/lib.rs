mod utils;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

#[wasm_bindgen]
pub fn read_tga(ctx: &CanvasRenderingContext2d, buffer: &[u8]) {
    let img = image::load_from_memory_with_format(buffer, image::ImageFormat::Tga).expect("err");
    // let img = img.blur(5.0);

    let width = img.width();
    let height = img.height();

    let image_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut img.to_rgba8()), width, height)
            .unwrap();

    let canvas = ctx.canvas().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);

    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
}

#[wasm_bindgen]
pub fn rgba_to_rgb(ctx: &CanvasRenderingContext2d, sw: f64, sh: f64) {
    let data = ctx.get_image_data(0.0, 0.0, sw, sh).unwrap();
    let raw_pixels = data.data().to_vec();
    let img_buffer = image::ImageBuffer::from_vec(sw as u32, sh as u32, raw_pixels).unwrap();
    let image = image::DynamicImage::ImageRgb8(img_buffer);
}
