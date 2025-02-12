use log::{info, LevelFilter};
use rgraphics::graphics::draw::{Color, Image, Renderer, Transform2D};
use rgraphics::Raymond;
use rgraphics::{EventHandler, InputEvent};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::collections::HashMap;
use std::sync::Arc;
use glam::Vec2;
use winit::window::Window;

pub struct MyWindow {
    images: HashMap<String, Arc<Image>>,
    position: Vec2,
    frame_count: u32,
    fps_timer: f32
}

impl Default for MyWindow {
    fn default() -> Self {
        Self {
            images: HashMap::with_capacity(4),
            position: Vec2::new(0.0, 0.0),
            frame_count: 0,
            fps_timer: 0.0
        }
    }
}

impl EventHandler for MyWindow {
    fn on_init(&mut self) {
        info!("Window initialized");
        self.images.insert("tintin".to_string(), Arc::new(Image::from_file("C:/Workspace/rgraphics/examples/assets/tintin.jpg")));
        self.images.insert("tintindog".to_string(), Arc::new(Image::from_file("C:/Workspace/rgraphics/examples/assets/tintindog.jpg")));
    }

    fn on_input_event(&mut self, event: InputEvent) {
        //println!("Game input: {:?}", event);
    }

    fn on_update(&mut self, delta: f32) {
        
        self.frame_count += 1;
        self.fps_timer += delta;
        
        // calculate fps and print
        if self.fps_timer >= 1.0 {
            info!("FPS: {}", self.frame_count);
            self.frame_count = 0;
            self.fps_timer = 0.0;
        }
        
        // every frame move the position of the image
        self.position.x += 0.1 * delta;
        if self.position.x > 1.0 {
            self.position.x = -1.0;
        }
        
    }

    fn on_draw(&mut self, renderer: &mut Renderer) {

        //renderer.draw_image(Vector2::new(0.4, 0.4), self.images.get("tintindog").unwrap().clone());
        //renderer.draw_image(Vector2::new(-0.2, -0.2), self.images.get("tintin").unwrap().clone());
        renderer.draw_image(self.position, self.images.get("tintin").unwrap().clone());

        renderer.draw_triangle(Transform2D::at(0.3, -0.4), Color::RED);
        renderer.draw_triangle(Transform2D::at(-0.2, 0.4), Color::BLUE);
        // renderer.draw_rectangle(Transform::at(0.2, 0.2), Vector2::new(0.5, 0.5), Color::GREEN);
        //renderer.draw_circle(Vector2::new(-0.5, -0.5), 0.25, 32, Color::RED);
        //renderer.draw_line(Vector2::new(-0.5, -0.5), Vector2::new(0.5, 0.5), 0.5, Color::GREEN);

    }

    fn on_close(&mut self) -> bool {
        info!("Window closed");
        true
    }
}

fn main() {

    // enable trace logging
    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("TODO: panic message");
    
    let my_game = MyWindow::default();

    Raymond::new(Box::new(my_game))
        .set_target_fps(60)
        .set_window_attributes(Window::default_attributes()
            .with_title("Hello Window")
            .with_resizable(false)
            .with_transparent(true))
        .run();

}