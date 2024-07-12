mod utils;
use image::imageops::FilterType;
use image::Rgba;
use imageproc::region_labelling::{connected_components, Connectivity};
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
pub fn read_rgb(ctx: &CanvasRenderingContext2d, buffer: &[u8]) -> Vec<u8> {
    let src = image::load_from_memory_with_format(buffer, image::ImageFormat::Tga).expect("err");
    // let background_color = Rgba([255,255,255,255]);
    // let labels = connected_components(&image, Connectivity::Four, background_color);
    let w = src.width() / 5;
    let h = src.height() / 5;
    let src = src.resize(w, h, FilterType::CatmullRom);

    let image_data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut src.to_rgba8()), w, h).unwrap();

    ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();

    return src.to_rgb8().to_vec();
}
