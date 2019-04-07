extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::{ Context, GameResult };

mod game;
pub mod tetrimino;

use game::Game;

struct MainState {
    current_game: Option<Box<Game>>,
}

impl MainState {
    fn new() -> Self {
        MainState {
            current_game: None,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0; 4].into());

        if let Some(cg) = &mut self.current_game {
            cg.render(ctx);
        }

        Ok(())
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("tetriz", "555555");
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state).unwrap();
}
