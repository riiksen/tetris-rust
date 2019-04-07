use rand::distributions::{ Distribution, Standard };
use rand::Rng;

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

impl Type {
    pub fn color(&self) -> [f32; 4] {
        [0.6, 0.6, 0.6, 1.0]
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
