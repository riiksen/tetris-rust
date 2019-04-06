use ggez::event;
use ggez::{ Context, GameResult };

pub struct Game {
    pub matrix: [[Option<Tetrimino>; 20]; 10],
}

impl Game {
    pub fn new() -> Self {
        Game {}
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
