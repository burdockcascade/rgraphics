use log::{debug, info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use rgraphics::{EventHandler, IgniteFX};

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

    let mut engine = IgniteFX::new(600, 800, "Window Example");



    engine.run(&mut MyGame);

}