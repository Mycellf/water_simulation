use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Water Simulation".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
