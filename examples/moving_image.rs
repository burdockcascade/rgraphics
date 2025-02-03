use rgraphics::graphics::draw::{Image, Renderer, Transform2D};
use rgraphics::Raymond;
use rgraphics::EventHandler;
use std::sync::Arc;
use glam::Vec2;

pub struct MyWindow {
    tintin: Arc<Image>,
    transform: Transform2D
}

impl Default for MyWindow {
    fn default() -> Self {
        Self {
            tintin: Arc::new(Image::from_file("C:/Workspace/rgraphics/examples/assets/tintin.jpg")),
            transform: Transform2D::at(-1.0, 0.0)
        }
    }
}

impl EventHandler for MyWindow {
    fn on_update(&mut self, delta: f32) {
        self.transform.position.x += 0.1 * delta;
        if self.transform.position.x > 1.0 {
            self.transform.position.x = -1.0;
        }
    }

    fn on_draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_image(self.transform.clone(), self.tintin.clone());
    }

}

fn main() {
    
    let my_game = MyWindow::default();

    Raymond::create_window(600, 800, "Moving Image", Box::new(my_game))
        .set_target_fps(60)
        .run();

}