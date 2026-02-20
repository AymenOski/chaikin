mod poly;

use poly::{Point, Poly, Polygone};

pub async fn run() {
    use macroquad::prelude::*;
    
    let mut control_points: Vec<Point> = Vec::new();
    
    loop {
        clear_background(BLACK);
        
        // Exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        // Clear screen and reset for new points
        if is_key_pressed(KeyCode::C) {
            control_points.clear();
        }
        
        // Detect mouse click and store point
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            control_points.push(Point {
                x: mx as f64,
                y: my as f64,
                kind: Poly::P,
            });
        }
        
        // Display all control points as circles
        for point in &control_points {
            draw_circle(point.x as f32, point.y as f32, 5.0, WHITE);
        }
        
        // Draw tutorial/instructions
        draw_text("Controls:", 10.0, 30.0, 20.0, WHITE);
        draw_text("Click to add points | C to clear | ESC to exit", 10.0, 55.0, 18.0, WHITE);
        draw_text(&format!("Points: {}", control_points.len()), 10.0, 80.0, 20.0, GREEN);
        
        next_frame().await;
    }
}
