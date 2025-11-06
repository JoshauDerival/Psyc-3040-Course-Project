use macroquad::prelude::*;

#[macroquad::main("Uneven Vision Simulation")]
async fn main() {
    let mut obj_x = 100.0;
    let mut direction = 1.0;

    loop {
        let screen_width = screen_width();
        let screen_height = screen_height();
        let mid_x = screen_width / 2.0;
        let mid_y = screen_height / 2.0;

        clear_background(BLACK);

        // Background halves
        draw_rectangle(0.0, 0.0, mid_x, screen_height, Color::from_rgba(200, 200, 200, 255));
        draw_rectangle(mid_x, 0.0, mid_x, screen_height, Color::from_rgba(100, 100, 100, 255)); 

        // --- Vertical fusion boundary ---
        draw_rectangle(mid_x - 2.0, 0.0, 4.0, screen_height, Color::from_rgba(255, 255, 255, 160));

        // --- Moving object ---
        // Distance from fusion line (affects blur)
        let distance_from_mid = (obj_x - mid_x).abs();
        let blur_factor = (1.0 - (distance_from_mid / (screen_width / 2.0))).clamp(0.0, 1.0);

        // Object color fades (simulating blur)
        let alpha = (255.0 * (1.0 - blur_factor * 0.9)) as u8;
        let color = Color::from_rgba(255, 0, 0, alpha);

        draw_circle(obj_x, mid_y, 100.0, color);

        // --- Labels ---
        draw_text("Left Eye (Nearsighted)", 80.0, 50.0, 30.0, BLACK);
        draw_text("Right Eye (Farsighted)", screen_width - 400.0, 50.0, 30.0, WHITE);
    
        // --- Animation logic ---
        obj_x += direction * 3.0;
        if obj_x > screen_width - 100.0 || obj_x < 100.0 {
            direction *= -1.0;
        }

        next_frame().await;
    }
}
