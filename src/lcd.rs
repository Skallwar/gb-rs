extern crate minifb;

use minifb::{Key, Window, WindowOptions};

struct Lcd {
    const width: usize,
    const height: usize,

    window: Window,
}

impl Lcd {
    fn new(width: usize, height: usize) -> Self {
        Lcd {
            width: width,
            height: heigth,

            window: Window::new(
                "GB-rs",
                width,
                height,
                WindowOptions::default(),
                )
                .unwrap_or_else(|e| {
                    panic!("{}", e);
                });

        }
    }

    fn update(&self, &frame: Vec<u32>) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
