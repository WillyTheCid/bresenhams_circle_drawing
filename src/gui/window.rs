use minifb::{Window, WindowOptions};

use crate::utils::{config, frame_counter::FrameCounter};

use super::scene::Scene;

pub struct AppWindow {
    width: usize,
    height: usize,
    scene: Scene,
    frame_counter: FrameCounter,
    buffer: Vec<u32>
}

impl AppWindow {
    pub fn new() -> Self {
        AppWindow {
            width: config::WINDOW_WIDTH,
            height: config::WINDOW_HEIGHT,
            scene: Scene::new(),
            frame_counter: FrameCounter::new(),
            buffer: vec![0u32; config::WINDOW_WIDTH * config::WINDOW_HEIGHT]
        }
    }

    pub fn display(&mut self) {
        let mut window = match Window::new("Test", self.width, self.height, WindowOptions::default()) {
            Ok(win) => win,
            Err(err) => {
                println!("Unable to create window {}", err);
                return;
            }
        };

        window.set_target_fps(config::FPS);

        while window.is_open() {
            self.update();
            self.draw();
            window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
        }

    }

    fn draw(&mut self) {
        self.scene.draw(&mut self.buffer);
    }

    fn update(&mut self) {
        self.scene.update();
        self.frame_counter.update();
    }
}

