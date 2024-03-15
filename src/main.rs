use macroquad::prelude::*;
use nalgebra::vector;

pub mod simulation_area;

fn window_config() -> Conf {
    Conf {
        window_title: "Water Simulation".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let simulation_area = simulation_area::SimulationArea::new(vector![128, 128]);

    loop {
        clear_background(BLACK);

        draw_texture(&simulation_area.texture, 0.0, 0.0, WHITE);

        next_frame().await;
    }
}
