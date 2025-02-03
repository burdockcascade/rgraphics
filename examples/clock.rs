use std::f32::consts::PI;
use chrono::{DateTime, Local, Timelike};
use glam::Vec2;
use winit::window::Window;
use rgraphics::graphics::draw::{Color, Renderer, Transform2D};
use rgraphics::Raymond;
use rgraphics::EventHandler;

pub struct MyWindow;

impl MyWindow {

    fn draw_clock(renderer: &mut Renderer, now: DateTime<Local>) {
        let (center_x, center_y) = (0.0, 0.0);
        let radius = 0.75;

        // Draw the clock face (circle - simplified here with lines)
        let num_segments = 60; // For smoother circle representation
        for i in 0..num_segments {
            let angle1 = 2.0 * PI * (i as f32) / (num_segments as f32);
            let angle2 = 2.0 * PI * ((i + 1) as f32) / (num_segments as f32);

            let x1 = center_x + radius * angle1.cos();
            let y1 = center_y + radius * angle1.sin();
            let x2 = center_x + radius * angle2.cos();
            let y2 = center_y + radius * angle2.sin();

            renderer.draw_line(Vec2::new(x1, y1), Vec2::new(x2, y2), 0.01, Color::BLUE);
        }
        
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();

        // Corrected and Simplified angle calculations (clockwise, 0 at 12 o'clock):
        let hour_angle = (hour as f32 % 12.0 + minute as f32 / 60.0) / 12.0 * 2.0 * PI; 
        let minute_angle = (minute as f32 + second as f32 / 60.0) / 60.0 * 2.0 * PI; 
        let second_angle = (second as f32) / 60.0 * 2.0 * PI;

        // Hand lengths (adjust as needed)
        let hour_length = radius * 0.5;
        let minute_length = radius * 0.7;
        let second_length = radius * 0.9;
        
        // Draw hands (same as before)
        MyWindow::draw_hand(renderer, center_x, center_y, hour_angle, hour_length, Color::RED); // Red - Hour
        MyWindow::draw_hand(renderer, center_x, center_y, minute_angle, minute_length, Color::GREEN); // Green - Minute
        MyWindow::draw_hand(renderer, center_x, center_y, second_angle, second_length, Color::BLUE); // Blue - Second
    }

    fn draw_hand(renderer: &mut Renderer, center_x: f32, center_y: f32, angle: f32, length: f32, color: Color) {
        let end_x = center_x + length * angle.cos();
        let end_y = center_y + length * angle.sin();
        renderer.draw_line(Vec2::new(center_x, center_y), Vec2::new(end_x, end_y), 0.01, color.into());
    }
    
}

impl EventHandler for MyWindow {
    fn on_draw(&mut self, renderer: &mut Renderer) {
        
        // get current time
        let now = chrono::Local::now();
         
        // draw clock
        MyWindow::draw_clock(renderer, now);
    }
}

fn main() {
    
    let my_game = MyWindow {};

    Raymond::create_window(400, 400, "Clock", Box::new(my_game))
        .set_target_fps(60)
        .run();

}