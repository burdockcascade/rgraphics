use std::collections::HashMap;
use std::iter::Map;
use std::sync::{Arc, Mutex};
use cgmath::Vector2;
use log::LevelFilter;
use rgraphics::frame::Renderer;
use rgraphics::Raymond;
use rgraphics::EventHandler;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use winit::keyboard::KeyCode;
use rgraphics::graphics::draw::{Color, Image};
use rgraphics::frame::Frame;

pub struct MyGame {
    images: HashMap<String, Image>,
}

impl Default for MyGame {
    fn default() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
}

impl EventHandler for MyGame {
    fn on_init(&mut self) {
        println!("Game initialized");
        
        self.images.insert("tintin".to_string(), Image::from_file("C:/Workspace/rgraphics/examples/tintin.jpg"));
        
    }
    
    fn on_keyboard_input(&mut self, key: KeyCode) {
        println!("Key pressed: {:?}", key);
    }
    
    fn on_cursor_moved(&mut self, position: Vector2<f32>) {
        println!("Cursor moved: {:?}", position);
    }

    fn on_frame(&mut self, frame: &mut Frame) {
        
        
        let renderer = frame.renderer();
        
        renderer.draw_image(Vector2::new(0.2, 0.2), self.images.get("tintin").unwrap().clone());

        renderer.draw_triangle(Vector2::new(0.3, -0.4), Color::RED);
        renderer.draw_triangle(Vector2::new(-0.2, 0.4), Color::BLUE);

    }
    
    fn on_close(&mut self) -> bool {
        println!("Game closed");
        true
    }
}

fn main() {

    // enable trace logging
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("TODO: panic message");
    
    let my_game = MyGame::default();

    Raymond::create_window(600, 800, "Window Example")
        .with_handler(Box::new(my_game))
        .run();



}