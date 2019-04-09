extern crate ggez;

use ggez::event;
use ggez::event::{ KeyCode, KeyMods };
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
        Self {
            current_game: None,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Some(cg) = &mut self.current_game {
            if cg.is_finished() { self.current_game = None; }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [1.0; 4].into());

        if let Some(cg) = &mut self.current_game {
            cg.render(ctx)?;
        }

        let fps = ggez::timer::fps(ctx);

        let text = graphics::Text::new(fps.to_string());

        graphics::draw(
            ctx,
            &text,
            (ggez::mint::Point2 { x: 400.0, y: 0.0 }, graphics::BLACK)
        ).unwrap();

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {
        if let Some(cg) = &mut self.current_game {
            cg.key_down_event(ctx, keycode, keymod, repeat);
        } else {
            if let KeyCode::Key1 = keycode {
               self.current_game = Some(Box::new(game::Marathon::new()));
            }
        }
    }
}

fn main() {
    let cb = ggez::ContextBuilder::new("tetriz", "555555");
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state).unwrap();
}
