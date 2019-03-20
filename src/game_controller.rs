//! Game controller

use piston::input::GenericEvent;

use crate::game::Game;

pub struct GameController {
    /// Stores the game state
    pub game: Game,
}

impl GameController {
    pub fn new(game: Game) -> GameController {
        GameController {
            game: game,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{ Button, Key };

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A | Key::Left => {
                    self.game.move_left();
                },
                Key::S | Key::Down => {
                    self.game.soft_drop();
                },
                Key::D | Key::Right => {
                    self.game.move_right();
                },
                Key::Space => {
                    self.game.hard_drop();
                },
                _ => {},
           }
        }
    }
}
