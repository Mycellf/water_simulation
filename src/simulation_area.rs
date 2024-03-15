use macroquad::prelude::*;
use nalgebra::Vector2;

pub struct SimulationArea {
    pub simulation: Vec<Particle>,
    pub image: Image,
    pub texture: Texture2D,
}

impl SimulationArea {
    pub fn new(size: Vector2<usize>) -> Self {
        let element_count = size.x * size.y;
        let simulation = (0..element_count).map(|_| Particle).collect();

        let image = Image::gen_image_color(size.x as u16, size.y as u16, WHITE);
        let texture = Texture2D::from_image(&image);

        Self {
            simulation,
            image,
            texture,
        }
    }
}

pub struct Particle;
