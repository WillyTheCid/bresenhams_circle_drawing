use minifb::{Window, WindowOptions};

pub struct AppWindow {
    width: usize,
    height: usize
}

impl AppWindow {
    pub fn new(width: usize, height: usize) -> Self {
        AppWindow {
            height,
            width
        }
    }
    pub fn display(&self) {
        let buffer = vec![0u32; self.width * self.height];

        let mut window = match Window::new("Test", self.width, self.height, WindowOptions::default()) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };

        window.set_target_fps(60);

        while window.is_open() {
            window.update_with_buffer(&buffer, self.width, self.height).unwrap();
        }

    }
}

