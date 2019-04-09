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
        match self {
            Type::I => [0.2, 0.2, 0.1, 1.0].into(),
            Type::O => [0.0, 0.1, 0.1, 1.0].into(),
            Type::T => [0.1, 0.0, 0.1, 1.0].into(),
            Type::S => [0.1, 0.0, 0.0, 1.0].into(),
            Type::Z => [0.0, 0.1, 0.0, 1.0].into(),
            Type::J => [0.0, 0.0, 0.1, 1.0].into(),
            Type::L => [0.1, 0.4, 0.0, 1.0].into(),
        }
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
