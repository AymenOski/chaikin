mod poly;

use macroquad::prelude::*;
use poly::{Point, Poly, Polygone};

pub async fn run() {
    let mut control_points = Polygone::empty();
    let mut animation_started = false;
    let mut animation_step: u8 = 0;
    let mut last_step_time: f64 = 0.0;
    let step_duration: f64 = 1.0;

    loop {
        clear_background(BLACK);

        // Exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Clear screen and reset for new points
        if is_key_pressed(KeyCode::C) {
            control_points.delete();
            animation_started = false;
            animation_step = 0;
        }

        if is_key_pressed(KeyCode::Enter) && !animation_started {
            if control_points.polygone.len() >= 1 {
                control_points.cut_corners();
                animation_started = true;
                animation_step = 0;
                last_step_time = get_time();
            }
        }

        // Detect mouse click and store point
        if is_mouse_button_pressed(MouseButton::Left) && !animation_started {
            let (mx, my) = mouse_position();
            let control_point = Point::new(mx as f64, my as f64, Poly::P, 0);

            if control_points.polygone.is_empty() {
                control_points = Polygone::new(control_point);
            } else {
                control_points.append_point(control_point);
            }
        }

        // Advance animation step on timer
        if animation_started {
            let now = get_time();
            println!("Now: {}, Last Step Time: {}, Step Duration: {}", now, last_step_time, step_duration);
            if now - last_step_time >= step_duration {
                animation_step += 1;
                if animation_step > 7 {
                    animation_step = 0;
                }
                last_step_time = now;
            }
        } else {
            // Draw circles at each point
            for point in &control_points.polygone {
                draw_circle(point.x as f32, point.y as f32, 5.0, WHITE);
            }
        }

        // Draw: at step 0 show all control points; at step N always include first+last and all step=N points
        let step = if animation_started { animation_step } else { 0 };
        let mut filtered: Vec<&Point> = Vec::new();
        if !control_points.polygone.is_empty() {
            if step == 0 {
                // show all original control points
                for p in &control_points.polygone {
                    if p.step == 0 {
                        filtered.push(p);
                    }
                }
            } else {
                filtered.push(&control_points.polygone[0]);
                for p in &control_points.polygone {
                    if p.step == step {
                        filtered.push(p);
                    }
                }
                filtered.push(&control_points.polygone[control_points.polygone.len() - 1]);
            }
        }

        if animation_started {
            for i in 0..filtered.len().saturating_sub(1) {
                draw_line(
                    filtered[i].x as f32,
                    filtered[i].y as f32,
                    filtered[i + 1].x as f32,
                    filtered[i + 1].y as f32,
                    2.0,
                    RED,
                );
            }
        }

        // Draw tutorial/instructions
        draw_text("Controls:", 10.0, 30.0, 20.0, WHITE);
        draw_text(
            "Click to add points | Enter to animate | C to clear | ESC to exit",
            10.0,
            55.0,
            18.0,
            WHITE,
        );

        if animation_started {
            draw_text(
                &format!("Step: {}/7", animation_step),
                10.0,
                105.0,
                20.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}
