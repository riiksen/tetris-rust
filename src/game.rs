use ggez::{ Context, GameResult };

use super::tetrimino;

fn gen_start_tetriminos() -> [tetrimino::Type; 5] {
    let mut nt = [tetrimino::Type::I; 5];

    for i in 0..5 {
        nt[i] = rand::random::<tetrimino::Type>();
    }

    nt
}

pub trait Game {
    fn render(&mut self, ctx: &mut Context) -> GameResult;
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn start(&mut self) -> GameResult;
}

pub struct Marathon {
    pub matrix: [[Option<tetrimino::Type>; 20]; 10],
    pub next_tetriminos: [tetrimino::Type; 5],
}

impl Marathon {
    fn new() -> Self {
        Marathon {
            matrix: [[None; 20]; 10],
            next_tetriminos: gen_start_tetriminos(),
        }
    }
}

impl Game for Marathon {
    fn start(&mut self) -> GameResult {
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn render(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
