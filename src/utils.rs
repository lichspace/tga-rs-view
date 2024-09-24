pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use std::cmp;

pub struct BoundingRect {
    min_x: u32,
    min_y: u32,
    max_x: u32,
    max_y: u32,
}

impl BoundingRect {
    pub fn new() -> Self {
        BoundingRect {
            min_x: u32::MAX,
            min_y: u32::MAX,
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn update(&mut self, x: u32, y: u32) {
        self.min_x = cmp::min(self.min_x, x);
        self.min_y = cmp::min(self.min_y, y);
        self.max_x = cmp::max(self.max_x, x);
        self.max_y = cmp::max(self.max_y, y);
    }

    pub fn get_rect(&self) -> Option<(u32, u32, u32, u32)> {
        if self.min_x <= self.max_x && self.min_y <= self.max_y {
            Some((
                self.min_x,
                self.min_y,
                self.max_x - self.min_x + 1,
                self.max_y - self.min_y + 1,
            ))
        } else {
            None
        }
    }
}