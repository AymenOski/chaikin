mod poly;

use poly::{Point, Poly, Polygone};

pub async fn run() {
    use macroquad::prelude::*;
    
    loop {
        clear_background(BLACK);
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        next_frame().await; // 
    }
}
