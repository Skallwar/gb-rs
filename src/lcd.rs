extern crate minifb;

use minifb::{Window, WindowOptions};

const LCD_WIDTH: usize = 144;
const LCD_HEIGHT: usize = 160;

pub struct Lcd {
    window: Window,
}

impl Lcd {
    pub fn new() -> Self {
        Lcd {
            window: Window::new("GB-rs", LCD_WIDTH, LCD_HEIGHT, WindowOptions::default())
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                }),
        }
    }

    pub fn update(&mut self, frame: &Vec<u32>) {
        self.window.update_with_buffer(frame).unwrap();
    }
}
