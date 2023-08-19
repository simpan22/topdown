use glium::glutin::dpi::PhysicalPosition;
use nalgebra_glm::Vec2;


pub struct Mouse {
    screen_pos: Vec2,
}

impl Mouse {
    pub fn new(screen_w: i32, screen_h: i32) -> Self {
        Mouse {
            screen_pos: Vec2::new((screen_w / 2) as f32, (screen_h / 2) as f32),
        }
    }

    pub fn update(&mut self, pos: &PhysicalPosition<f64>) {
        self.screen_pos.x = pos.x as f32;
        self.screen_pos.y = pos.y as f32;
    }


}
