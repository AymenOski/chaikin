mod poly;

use macroquad::prelude::*;
use poly::{Point, Poly, Polygone};

pub async fn run() {
    let mut control_points = Polygone::empty();

    loop {
        clear_background(BLACK);

        // Exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Clear screen and reset for new points
        if is_key_pressed(KeyCode::C) {
            control_points.delete();
        }

        if is_key_pressed(KeyCode::Enter) {
            control_points.cut_corners();
        }

        // Detect mouse click and store point
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let control_point = Point::new(mx as f64, my as f64, Poly::P, 0);

            if control_points.polygone.is_empty() {
                control_points = Polygone::new(control_point);
            } else {
                control_points.append_point(control_point);
            }
        }

        // Display all control points as circles
        for point in &control_points.polygone {
            draw_circle(point.x as f32, point.y as f32, 5.0, WHITE);
        }

        // Draw tutorial/instructions
        draw_text("Controls:", 10.0, 30.0, 20.0, WHITE);
        draw_text(
            "Click to add points | C to clear | ESC to exit",
            10.0,
            55.0,
            18.0,
            WHITE,
        );
        draw_text(
            &format!("Points: {}", control_points.polygone.len()),
            10.0,
            80.0,
            20.0,
            GREEN,
        );

        next_frame().await;
    }
}
