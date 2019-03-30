extern crate minifb;

use minifb::{Window, WindowOptions};

const LCD_WIDTH: usize = 160;
const LCD_HEIGHT: usize = 144;

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
        // self.frame_print(frame);
    }

    pub fn frame_print(&mut self, frame: &Vec<u32>) {
        for j in 0..LCD_HEIGHT {
            for x in 0..LCD_WIDTH {
                if frame[j as usize * LCD_WIDTH as usize + x as usize] == 0 {
                    print!(" ");
                } else {
                    print!("1");
                }
            }

            println!();
        }
    }
}
