//! Game controller

use piston::input::GenericEvent;

use crate::game::Game;

pub struct GameController {
    /// Stores the game state
    pub game: Game,

    pub current_column: u8,
}

impl GameController {
    pub fn new(game: Game) -> GameController {
        GameController {
            game: game,
            current_column: 5,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{ Button, Key };

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A | Key::Left => {
                    if self.game.move_left() {
                        self.current_column -= 1;
                    }
                },
                Key::S | Key::Down => {

                },
                Key::D | Key::Right => {
                    if self.game.move_right() {
                        self.current_column += 1;
                    }
                },
                Key::Space => {
                    self.game.hard_drop();
                },
                _ => {},
           }
        }
    }
}
