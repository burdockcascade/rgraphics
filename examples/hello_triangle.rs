use cgmath::Vector2;
use rgraphics::frame::Renderer;
use rgraphics::graphics::draw::Color;
use rgraphics::Raymond;
use rgraphics::EventHandler;

pub struct MyWindow {
}

impl EventHandler for MyWindow {
    fn on_draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_triangle(Vector2::new(0.0, 0.0), Color::RED);
    }
}

fn main() {
    
    let my_game = MyWindow {};

    Raymond::create_window(600, 800, "Window Example", Box::new(my_game))
        .set_target_fps(60)
        .run();

}