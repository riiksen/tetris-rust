//! Game view

use graphics::types::Color;
use graphics::{ Context, Graphics };

use crate::game_controller::GameController;

pub struct GameViewSettings {

}

impl GameViewSettings {
    pub fn new() -> GameViewSettings {
        GameViewSettings {

        }
    }
}

pub struct GameView {
    settings: GameViewSettings,
}

impl GameView {
    pub fn new(settings: GameViewSettings) -> GameView {
        GameView {
            settings: settings,
        }
    }

    pub fn draw<G: Graphics>(
        &self,
        controller: &GameController,
        c: &Context,
        g: &mut G
    ) {

    }
}
