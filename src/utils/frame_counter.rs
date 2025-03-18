use std::time::Instant;

pub struct FrameCounter {
    last_printed_instant: Instant,
    frame_count: u32,
}

impl FrameCounter {
    pub fn new() -> Self {
        Self {
            last_printed_instant: Instant::now(),
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let elapsed_time = self.last_printed_instant.elapsed().as_secs_f32();
        if elapsed_time > 1.0 {
            let frame_time = elapsed_time * 1000.0 / self.frame_count as f32;
            let fps = self.frame_count as f32 / elapsed_time;
            println!("Frame time {:.2}ms ({:.1} FPS)", frame_time, fps);
            self.last_printed_instant = Instant::now();
            self.frame_count = 0;
        }
    }
}