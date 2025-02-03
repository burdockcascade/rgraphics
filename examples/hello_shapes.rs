use glam::Vec2;
use rgraphics::graphics::draw::{Color, Renderer, Transform2D};
use rgraphics::Raymond;
use rgraphics::EventHandler;

pub struct MyWindow;

impl EventHandler for MyWindow {
    fn on_draw(&mut self, renderer: &mut Renderer) {
        renderer.draw_rectangle(Transform2D::at(0.10, 0.10), Vec2::new(0.5, 0.5), Color::GREEN);
        renderer.draw_circle(Transform2D::at(0.25, 0.65), 0.25, 32, Color::RED);
        renderer.draw_triangle(Transform2D::at(0.25, 0.25), Color::BLUE);
        
    }
}

fn main() {
    
    let my_game = MyWindow {};

    Raymond::create_window(800, 800, "Hello Triangle", Box::new(my_game))
        .set_target_fps(60)
        .run();

}