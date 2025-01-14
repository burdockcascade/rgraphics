use cgmath::Vector2;
use log::LevelFilter;
use rgraphics::frame::{Renderer};
use rgraphics::graphics::draw::{Color, Image};
use rgraphics::{EventHandler, InputEvent};
use rgraphics::Raymond;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::collections::HashMap;

pub struct MyWindow {
    images: HashMap<String, Image>,
}

impl Default for MyWindow {
    fn default() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
}

impl EventHandler for MyWindow {
    fn on_init(&mut self) {
        println!("Window initialized");
        
        self.images.insert("tintin".to_string(), Image::from_file("C:/Workspace/rgraphics/examples/tintin.jpg"));
        
    }

    fn on_input_event(&mut self, event: InputEvent) {
        // println!("Game input: {:?}", event);
    }

    fn on_update(&mut self, delta: f32) {
        // println!("Game update: {:?}", delta);
    }
    
    fn on_draw(&mut self, renderer: &mut Renderer) {
        
        renderer.draw_image(Vector2::new(0.2, 0.2), self.images.get("tintin").unwrap().clone());

        renderer.draw_triangle(Vector2::new(0.3, -0.4), Color::RED);
        renderer.draw_triangle(Vector2::new(-0.2, 0.4), Color::BLUE);
        renderer.draw_rectangle(Vector2::new(0.2, 0.2), Vector2::new(0.5, 0.5), 0.0, Color::GREEN);

    }
    
    fn on_close(&mut self) -> bool {
        println!("Window closed");
        true
    }
}

fn main() {

    // enable trace logging
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("TODO: panic message");
    
    let my_game = MyWindow::default();

    Raymond::create_window(600, 800, "Window Example", Box::new(my_game)).run();

}