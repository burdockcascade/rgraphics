use cgmath::Vector2;
use rgraphics::frame::Renderer;
use rgraphics::graphics::draw::Image;
use rgraphics::Raymond;
use rgraphics::EventHandler;
use std::sync::Arc;
use winit::window::Window;

pub struct MyWindow {
    tintin: Arc<Image>,
    position: Vector2<f32>
}

impl Default for MyWindow {
    fn default() -> Self {
        Self {
            tintin: Arc::new(Image::from_file("C:/Workspace/rgraphics/examples/assets/tintin.jpg")),
            position: Vector2::new(0.0, 0.0)
        }
    }
}

impl EventHandler for MyWindow {
    fn on_update(&mut self, delta: f32) {
        self.position.x += 0.1 * delta;
        if self.position.x > 1.0 {
            self.position.x = -1.0;
        }
    }

    fn on_draw(&mut self, window: Arc<Window>, renderer: &mut Renderer) {
        renderer.draw_image(self.position, self.tintin.clone());
    }

}

fn main() {
    
    let my_game = MyWindow::default();

    Raymond::create_window(600, 800, "Moving Image", Box::new(my_game))
        .set_target_fps(60)
        .run();

}