use macroquad::prelude::*;
use nalgebra::Vector2;

#[derive(Clone, Debug)]
pub struct SimulationArea {
    pub size: Vector2<usize>,
    pub simulation: Vec<Particle>,
    pub image: Image,
    pub texture: Texture2D,
}

impl SimulationArea {
    pub fn new(size: Vector2<usize>) -> Self {
        let element_count = size.x * size.y;
        let simulation = (0..element_count)
            .map(|_| Particle::new(ParticleType::Air))
            .collect();

        let image = Image::gen_image_color(size.x as u16, size.y as u16, WHITE);
        let texture = Texture2D::from_image(&image);

        Self {
            size,
            simulation,
            image,
            texture,
        }
    }

    pub fn get(&self, position: Vector2<usize>) -> Option<&Particle> {
        let index = self.get_index(position)?;

        Some(&self.simulation[index])
    }

    pub fn get_mut(&mut self, position: Vector2<usize>) -> Option<&mut Particle> {
        let index = self.get_index(position)?;

        Some(&mut self.simulation[index])
    }

    pub fn set(&mut self, position: Vector2<usize>, particle: Particle) -> Option<()> {
        let index = self.get_index(position)?;

        self.image.get_image_data_mut()[index] = particle.particle_type.get_color().into();
        self.simulation[index] = particle;

        Some(())
    }

    pub fn get_index(&self, position: Vector2<usize>) -> Option<usize> {
        if position.x < self.size.x && position.y < self.size.y {
            Some(position.x + position.y * self.size.x)
        } else {
            None
        }
    }
}

impl std::ops::Index<Vector2<usize>> for SimulationArea {
    type Output = Particle;

    fn index(&self, index: Vector2<usize>) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::ops::IndexMut<Vector2<usize>> for SimulationArea {
    fn index_mut(&mut self, index: Vector2<usize>) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub pressure: u16,
}

impl Particle {
    pub fn new(particle_type: ParticleType) -> Self {
        Self {
            particle_type,
            pressure: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParticleType {
    Air,
    Wall,
}

impl ParticleType {
    pub fn get_color(self) -> Color {
        match self {
            Self::Air => BLACK,
            Self::Wall => BLUE,
        }
    }
}
