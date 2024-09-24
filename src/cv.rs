use image::{math::Rect, ImageBuffer, Luma, Rgba};

use crate::utils::BoundingRect;

pub fn flood_fill(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    seed_x: u32,
    seed_y: u32,
    fill_color: Rgba<u8>,
) -> (ImageBuffer<Rgba<u8>, Vec<u8>>, Rect) {
    let mut visited = ImageBuffer::from_pixel(image.width(), image.height(), Luma([0]));
    let mut result = ImageBuffer::from_pixel(image.width(), image.height(), Rgba([0, 0, 0, 0]));
    let mut stack = vec![(seed_x, seed_y)];
    let mut bbox = BoundingRect::new();

    let width = image.width();
    let height = image.height();
    let seed_color = image.get_pixel(seed_x, seed_y);

    while let Some((x, y)) = stack.pop() {
        if visited.get_pixel(x, y)[0] > 0 {
            continue;
        }

        visited.put_pixel(x, y, Luma([1]));
        result.put_pixel(x, y, fill_color);
        bbox.update(x, y);

        // 检查相邻的像素
        for &(dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx = nx as u32;
                let ny = ny as u32;

                if visited.get_pixel(nx, ny)[0] == 0 && image.get_pixel(nx, ny) == seed_color {
                    stack.push((nx, ny));
                }
            }
        }
    }

    let (x, y, width, height) = bbox.get_rect().unwrap();
    let rect = Rect {
        x,
        y,
        width,
        height,
    };
    
    return (result, rect);
}
