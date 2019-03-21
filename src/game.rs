//! Game logic
extern crate rand;

use rand::{
    distributions::{ Distribution, Standard },
    Rng,
};

pub struct Game {
    pub board: [[Option<Tetrimino>; 20]; 10],
    pub next_tetriminos: Vec<Tetrimino>,
    pub current_tetrimino: Tetrimino,
    pub holding: Option<Tetrimino>,
    pub selected_column: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[None; 20]; 10],
            next_tetriminos: Game::gen_start_tetriminos(),
            current_tetrimino: rand::random::<Tetrimino>(),
            holding: None,
            selected_column: 5,
        }
    }

    fn gen_start_tetriminos() -> Vec<Tetrimino> {
        let mut nt: Vec<Tetrimino> = Vec::new();

        for _ in 0..5 {
            nt.push(rand::random::<Tetrimino>());
        }

        nt
    }

    pub fn soft_drop(&mut self) -> bool {
        true
    }

    pub fn hard_drop(&mut self) -> bool {
        true
    }

    pub fn move_left(&mut self) -> bool {
        true
    }

    pub fn move_right(&mut self) -> bool {
        true
    }
}

#[derive(Copy, Clone)]
pub enum Tetrimino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetrimino {
    pub fn color(&self) -> [f32; 4] {
        [0.6, 0.6, 0.6, 1.0]
    }
}

impl Distribution<Tetrimino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetrimino {
        match rng.gen_range(0, 7) {
            0 => Tetrimino::I,
            1 => Tetrimino::O,
            2 => Tetrimino::T,
            3 => Tetrimino::S,
            4 => Tetrimino::Z,
            5 => Tetrimino::J,
            _ => Tetrimino::L,
        }
    }
}
