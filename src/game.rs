use ggez::event;
use ggez::{ Context, GameResult };

use super::tetrimino;

fn gen_start_tetriminos() -> [tetrimino::Type; 5] {
    let mut nt = [tetrimino::Type::I; 5];

    for i in 0..5 {
        nt[i] = rand::random::<tetrimino::Type>();
    }

    nt
}

pub struct Game {
    pub matrix: [[Option<tetrimino::Type>; 20]; 10],
    pub next_tetriminos: [tetrimino::Type; 5],
}

impl Game {
    pub fn new() -> Self {
        Game {
            matrix: [[None; 20]; 10],
            next_tetriminos: gen_start_tetriminos(),
        }
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
