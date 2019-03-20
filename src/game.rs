//! Game logic
extern crate rand;

use rand::{
    distributions::{ Distribution, Standard },
    Rng,
};

pub struct Game {
    board: [[u8; 20]; 10],
    next_tetriminos: Vec<Tetrimino>,
    current_tetrimino: Tetrimino,
    holding: Option<Tetrimino>,
    selected_column: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[0u8; 20]; 10],
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
