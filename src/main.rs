extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::{ Context, GameResult };

mod game;
pub mod tetrimino;

use game::Game;

struct MainState {

}

impl MainState {
    fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("tetriz", "555555");
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state).unwrap();
}
