use cgmath::Vector2;
use log::LevelFilter;
use rgraphics::Raymond;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use rgraphics::graphics::draw::{Color, Image};

fn main() {

    // enable trace logging
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("TODO: panic message");

    let tintin = Image::from_file("C:/Workspace/rgraphics/examples/tintin.jpg");
    let helloworld = Image::from_file("C:/Workspace/rgraphics/examples/helloworld.jpg");
    
    Raymond::create_window(600, 800, "Window Example")
        //.draw_rectangle(Vector2::new(1.0, 1.0), Vector2::new(0.0, 0.0), 0.0, Color::RED)
        //.draw_triangle(0.0, Color::GREEN)
        .draw_image(tintin)
        .run();

}