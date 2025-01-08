use cgmath::Vector2;
use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use rgraphics::{EventHandler, Raymond};
use rgraphics::graphics::draw::Color;

struct MyGame;

impl EventHandler for MyGame {
    fn on_init(&mut self) {
        info!("Init");
    }

    fn on_frame(&mut self, delta_time: f64) {
        info!("Frame");
    }

    fn on_close(&mut self) {
        info!("Close");
    }
}

fn main() {

    // enable trace logging
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("TODO: panic message");

    Raymond::create_window(600, 800, "Window Example")
        .draw_rectangle(Vector2::new(1.0, 1.0), Vector2::new(0.0, 0.0), 0.0, Color::RED)
        .draw_triangle(0.6, Color::GREEN)
        .run(&mut MyGame);

}