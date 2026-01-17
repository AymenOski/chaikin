use macroquad::prelude::*;

#[macroquad::main("Chaikin's Corner Cutting Algorithm")]
async fn main() {

    loop {
        clear_background(BLACK);

        // Exit
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
