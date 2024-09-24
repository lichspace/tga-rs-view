pub mod cv;
mod utils;
use image::imageops;
use image::math::Rect;
use image::DynamicImage;
use image::ImageBuffer;
use image::Rgba;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Rimage {
    id: i32,
    dimage: ImageBuffer<Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
    flood_fill_rect: Rect,
}

#[wasm_bindgen]
impl Rimage {
    #[wasm_bindgen(constructor)]
    pub fn new(id: i32) -> Rimage {
        let dimage = DynamicImage::new_rgba8(1, 1).to_rgba8();
        Rimage {
            id,
            dimage,
            width: 0,
            height: 0,
            flood_fill_rect: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        }
    }

    #[wasm_bindgen]
    pub fn process_data(&mut self) {
        self.id += 1;
    }

    #[wasm_bindgen]
    pub fn open_tga(&mut self, buffer: &[u8]) {
        let img =
            image::load_from_memory_with_format(buffer, image::ImageFormat::Tga).expect("err");
        self.dimage = img.to_rgba8();
        self.width = img.width();
        self.height = img.height();
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn data(&mut self) -> js_sys::Uint8Array {
        let data = self.dimage.to_vec();
        unsafe {
            return js_sys::Uint8Array::view(&data);
        };
    }

    #[wasm_bindgen(getter)]
    pub fn size(&mut self) -> Vec<u32> {
        let dimensions = self.dimage.dimensions();
        let res = vec![dimensions.0, dimensions.1];
        return res;
    }

    #[wasm_bindgen(getter)]
    pub fn flood_fill_rect(&mut self) -> Vec<u32> {
        let rect = &self.flood_fill_rect;
        return [rect.x, rect.y, rect.width, rect.height].to_vec();
    }

    #[wasm_bindgen]
    pub fn flood_fill(&mut self, x: u32, y: u32, color: Vec<u8>) -> js_sys::Uint8Array {
        println!("{:?}", color);
        let (mut mask, rect) = cv::flood_fill(
            &self.dimage,
            x,
            y,
            Rgba::<u8>([color[0], color[1], color[2], color[3]]),
        );

        self.flood_fill_rect = rect;
        let roi = imageops::crop(&mut mask, rect.x, rect.y, rect.width, rect.height).to_image();
        let data = roi.to_vec();
        unsafe {
            return js_sys::Uint8Array::view(&data);
        };
    }
}