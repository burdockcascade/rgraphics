use std::sync::Arc;
use cgmath::Vector2;
use winit::window::Window;
use rgraphics::frame::Renderer;
use rgraphics::graphics::draw::Color;
use rgraphics::Raymond;
use rgraphics::EventHandler;

pub struct MyWindow {
}

impl EventHandler for MyWindow {
    fn on_draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_triangle(Vector2::new(0.25, 0.25), Color::BLUE);
        renderer.draw_triangle(Vector2::new(0.0, 0.0), Color::GREEN);
        renderer.draw_triangle(Vector2::new(-0.25, -0.25), Color::RED);
    }
}

fn main() {
    
    let my_game = MyWindow {};

    Raymond::create_window(600, 800, "Hello Triangle", Box::new(my_game))
        .set_target_fps(60)
        .run();

}