use crate::{model::circle::Circle, utils::config};

pub struct Scene {
    circle: Circle
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            circle: Circle::new(config::CIRCLE_RADIUS, vec![config::CIRCLE_X_POS, config::CIRCLE_Y_POS])
        }
    }

    pub fn update(&self) {
        
    }

    pub fn draw(&self, buffer: &mut Vec<u32>) {
    
        self.circle.draw(buffer);
    }
}