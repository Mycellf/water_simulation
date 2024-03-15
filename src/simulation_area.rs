use macroquad::prelude::*;

pub struct SimulationArea {
    pub simulation: Vec<Particle>,
    pub image: Image,
    pub texture: Texture2D,
}

pub struct Particle;
