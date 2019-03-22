extern crate minifb;

use minifb::{Window, WindowOptions};

pub struct Lcd {
    window: Window,
}

impl Lcd {
    pub fn new(width: usize, height: usize) -> Self {
        Lcd {
            window: Window::new("GB-rs", width, height, WindowOptions::default()).unwrap_or_else(
                |e| {
                    panic!("{}", e);
                },
            ),
        }
    }

    pub fn update(&mut self, frame: &Vec<u32>) {
        self.window.update_with_buffer(frame).unwrap();
    }
}
