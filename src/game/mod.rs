use ggez::{ Context, GameResult };
use ggez::event::{ KeyCode, KeyMods };

mod marathon;

pub use marathon::Marathon;

pub trait Game {
    fn render(&mut self, ctx: &mut Context) -> GameResult;
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn start(&mut self) -> GameResult;
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool);
    fn is_finished(&mut self) -> bool;
}

