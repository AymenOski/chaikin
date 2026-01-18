use chaikin::chaikin_algo;
use chaikin::Point;
use macroquad::prelude::*;

#[macroquad::main("Chaikin's Corner Cutting Algorithm")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut chaikin_steps: Vec<Vec<Point>> = Vec::new();
    let mut current_step_index: usize = 0;
    let mut last_step_time = get_time();
    let step_duration = 0.5;
    let mut animation_started = false;
    let mut warning_time: Option<f64> = None;
    let warning_duration = 2.0;

    loop {
        clear_background(BLACK);

        // Exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Clear screen and reset for new points
        if is_key_pressed(KeyCode::C) {
            control_points.clear();
            chaikin_steps.clear();
            animation_started = false;
            current_step_index = 0;
            warning_time = None;
        }

        if !animation_started && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            control_points.push(Point {
                x: mx as f64,
                y: my as f64,
            });
        }

        if is_key_pressed(KeyCode::Enter) {
            if control_points.len() >= 2 {
                animation_started = true;
                chaikin_steps.clear();

                let mut points_for_iteration = control_points.clone();

                for _ in 0..7 {
                    // Save current step
                    chaikin_steps.push(points_for_iteration.clone());

                    // Generate next step using Chaikin
                    points_for_iteration = chaikin_algo(&points_for_iteration);
                }

                current_step_index = 0;
                last_step_time = get_time();
                warning_time = None;
            } else {
                warning_time = Some(get_time());
            }
        }

        if animation_started && !chaikin_steps.is_empty() {
            let current_time = get_time();
            if current_time - last_step_time > step_duration {
                current_step_index += 1;
                if current_step_index >= chaikin_steps.len() {
                    current_step_index = 0;
                }
                last_step_time = current_time;
            }

            // Draw current step
            let step = &chaikin_steps[current_step_index];
            for i in 0..step.len() - 1 {
                let p0 = step[i];
                let p1 = step[i + 1];
                draw_line(p0.x as f32, p0.y as f32, p1.x as f32, p1.y as f32, 2.0, RED);
            }
        }

        for &p in &control_points {
            draw_circle(p.x as f32, p.y as f32, 5.0, WHITE);
        }

        // Draw tutorial/instructions
        draw_text("Controls:", 10.0, 30.0, 20.0, WHITE);
        draw_text("Click to add points | Enter to animate | C to clear | ESC to exit", 10.0, 55.0, 18.0, WHITE);

        // Draw warning message if needed
        if let Some(start_time) = warning_time {
            if get_time() - start_time < warning_duration {
                draw_text(
                    &format!("Need at least 2 points! (have {})", control_points.len()),
                    10.0,
                    screen_height() - 30.0,
                    30.0,
                    YELLOW,
                );
            } else {
                warning_time = None;
            }
        }

        next_frame().await;
    }
}
