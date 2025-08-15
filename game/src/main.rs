use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Blaster of Disaster".into(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(Color::from_rgba(24, 24, 24, 255));

        draw_text("Hello from Macroquad!", 20.0, 40.0, 32.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 80.0, 24.0, GREEN);

        next_frame().await;
    }
}

