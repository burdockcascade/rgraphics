use cgmath::Vector2;
use rgraphics::graphics::draw::{Color, Renderer, Transform};
use rgraphics::Raymond;
use rgraphics::EventHandler;

pub struct MyWindow;

impl EventHandler for MyWindow {
    fn on_draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_circle(Transform::at(0.25, 0.65), 0.25, 32, Color::RED);
        renderer.draw_triangle(Transform::at(0.25, 0.25), Color::BLUE);
        renderer.draw_rectangle(Transform::at(0.10, 0.10), Vector2::new(0.5, 0.5), Color::GREEN);
        renderer.draw_line(Vector2::new(-0.2, -0.5), Vector2::new(0.5, 0.5), 0.01, Color::GREEN);
    }
}

fn main() {
    
    let my_game = MyWindow {};

    Raymond::create_window(800, 800, "Hello Triangle", Box::new(my_game))
        .set_target_fps(60)
        .run();

}