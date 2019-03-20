//! Game view

use graphics::types::Color;
use graphics::{ Context, Graphics };

use graphics::{ Image, Line, Rectangle, Transformed };

use crate::game_controller::GameController;

pub struct GameViewSettings {
    pub position: [f64; 2],
    pub size: [f64; 2],
    pub background_color: Color,
    pub border_color: Color,
    /// Cell border color
    pub cb_color: Color,
    /// Cell border radious
    pub cb_radious: f64,
}

impl GameViewSettings {
    pub fn new() -> GameViewSettings {
        GameViewSettings {
            position: [85., 40.],
            size: [300., 500.],
            background_color: [0.4, 0.4, 0.4, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
            cb_color: [0.2, 0.2, 0.2, 1.0],
            cb_radious: 1.0,
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
        self.draw_board(controller, c, g);
        self.draw_state(controller, c, g);
        self.draw_next_and_holding(controller, c, g);
        self.draw_current_and_shadow(controller, c, g);
    }

    fn draw_board<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {
        // Draw board borders
        let board_border = [
            self.settings.position[0] - 10.0, self.settings.position[1] - 10.0,
            self.settings.size[0] + 20.0, self.settings.size[1] + 20.0,
        ];

        Rectangle::new(self.settings.border_color)
            .draw(board_border, &c.draw_state, c.transform, g);

        // Draw board background
        let board_rect = [
            self.settings.position[0], self.settings.position[1],
            self.settings.size[0], self.settings.size[1],
        ];

        Rectangle::new(self.settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // Draw cell borders
        let cell_border = Line::new(self.settings.cb_color, self.settings.cb_radious);

        // Draw vertical lines
        for i in 0..10 {

        }

        // Draw horizontal lines
        for j in 0..20 {

        }

    }

    fn draw_state<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {

    }

    fn draw_next_and_holding<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {

    }

    fn draw_current_and_shadow<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {

    }
}

// fn draw_rectangles<G: Graphics>(
//     cursor: [f64; 2],
//     window: &Window,
//     c: &Context,
//     g: &mut G,
// ) {
//     let size = window.size();
//     let draw_size = window.draw_size();
//     let zoom = 0.2;
//     let offset = 30.0;

//     let rect_border = graphics::Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0);

//     // Cursor.
//     let cursor_color = [0.0, 0.0, 0.0, 1.0];
//     let zoomed_cursor = [offset + cursor[0] * zoom, offset + cursor[1] * zoom];
//     graphics::ellipse(
//         cursor_color,
//         graphics::ellipse::circle(zoomed_cursor[0], zoomed_cursor[1], 4.0),
//         c.transform,
//         g
//     );

//     // User coordinates.
//     rect_border.draw([
//             offset,
//             offset,
//             size.width as f64 * zoom,
//             size.height as f64 * zoom
//         ],
//         &c.draw_state, c.transform, g);
//     let rect_border = graphics::Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 1.0);
//     rect_border.draw(
//         [
//             offset + size.width as f64 * zoom,
//             offset,
//             draw_size.width as f64 * zoom,
//             draw_size.height as f64 * zoom
//         ],
//         &c.draw_state, c.transform, g);
// }

// fn draw_axis_values<W: Window, G: Graphics>(
//     axis_values: &mut AxisValues,
//     window: &W,
//     c: &Context,
//     g: &mut G,
// ) {
//     let window_height = window.size().height as f64;
//     let max_axis_height = 200.0;
//     let offset = 10.0;
//     let top = window_height - (max_axis_height + offset);
//     let color = [1.0, 0.0, 0.0, 1.0];
//     let width = 10.0;
//     let mut draw = |i, v: f64| {
//         let i = i as f64;
//         let height = (v + 1.0) / 2.0 * max_axis_height;
//         let rect = [offset + i * (width + offset),
//             top + max_axis_height - height, width, height];
//         graphics::rectangle(color, rect, c.transform, g);
//     };
//     for (i, &v) in axis_values.values().enumerate() {
//         draw(i, v);
//     }
// }
