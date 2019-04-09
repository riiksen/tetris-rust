extern crate ggez;

use ggez::event;
use ggez::event::{ KeyCode, KeyMods };
use ggez::graphics;
use ggez::{ Context, GameResult };

use std::env;
use std::path;

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
               self.current_game = Some(Box::new(game::Marathon::new(ctx)));
            }
        }
    }
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    println!("{:?}", resource_dir);

    let cb = ggez::ContextBuilder::new("tetriz", "555555").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state).unwrap();
}
