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

        draw_rectangle(0.0, 0.0, mid_x, screen_height, Color::from_rgba(200, 200, 200, 255));
        draw_rectangle(mid_x, 0.0, mid_x, screen_height, Color::from_rgba(100, 100, 100, 255)); 

        draw_rectangle(mid_x - 2.0, 0.0, 4.0, screen_height, Color::from_rgba(255, 255, 255, 160));

        let radius = 100.0;
        let circle_pos = vec2(obj_x, mid_y);
        let n_segments = 50;

        let mut vertices: Vec<Vertex> = Vec::with_capacity((n_segments + 2) as usize);
        let mut indices: Vec<u16> = Vec::with_capacity((n_segments * 3) as usize);

        let get_color_with_alpha = |x_pos: f32| {
            let fade_width = 150.0;
            let dist_from_mid = (x_pos - mid_x).abs();
            let alpha = (dist_from_mid / fade_width).clamp(0.0, 1.0);
            Color::from_rgba(255, 0, 0, (255.0 * alpha) as u8)
        };

        vertices.push(Vertex::new(
            circle_pos.x,
            circle_pos.y,
            0.,
            0.,
            0.,
            get_color_with_alpha(circle_pos.x),
        ));

        for i in 0..=n_segments {
            let angle = i as f32 * (360. / n_segments as f32);
            let vertex_pos = vec2(
                circle_pos.x + angle.to_radians().cos() * radius,
                circle_pos.y + angle.to_radians().sin() * radius,
            );
            vertices.push(Vertex::new(
                vertex_pos.x,
                vertex_pos.y,
                0.,
                0.,
                0.,
                get_color_with_alpha(vertex_pos.x),
            ));
        }

        for i in 1..=n_segments {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }

        draw_mesh(&Mesh {
            vertices,
            indices,
            texture: None,
        });

        draw_text("Bright area", 80.0, 50.0, 30.0, BLACK);
        draw_text("Dark area", screen_width - 400.0, 50.0, 30.0, WHITE);
    
        obj_x += direction * 3.0;
        if obj_x > screen_width - 100.0 || obj_x < 100.0 {
            direction *= -1.0;
        }

        next_frame().await;
    }
}
