use rand::distributions::{ Distribution, Standard };
use rand::Rng;

use ggez::graphics::Color;

#[derive(Copy, Clone)]
pub enum Type {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

// TODO: Implement
pub fn random() -> Type {
    Type::I
}

impl Type {
    pub fn color(&self) -> Color {
        [0.6, 0.6, 0.6, 1.0].into()
    }
}

impl Distribution<Type> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type {
        match rng.gen_range(0, 7) {
            0 => Type::I,
            1 => Type::O,
            2 => Type::T,
            3 => Type::S,
            4 => Type::Z,
            5 => Type::J,
            _ => Type::L,
        }
    }
}
