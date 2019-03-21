//! Game view

use graphics::types::Color;
use graphics::{ Context, Graphics };

use graphics::{ Image, Line, Rectangle, Transformed };

use crate::game_controller::GameController;
use crate::game::Tetrimino;

pub struct GameViewSettings {
    pub position: [f64; 2],
    pub size: [f64; 2],
    pub background_color: Color,
    pub border_color: Color,
    /// Cell border color
    pub cb_color: Color,
    /// Cell border radius
    pub cb_radius: f64,
}

impl GameViewSettings {
    pub fn new() -> GameViewSettings {
        GameViewSettings {
            position: [85., 40.],
            size: [300., 500.],
            background_color: [0.4, 0.4, 0.4, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
            cb_color: [0.2, 0.2, 0.2, 1.0],
            cb_radius: 1.0,
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
        self.draw_current_next_and_holding(controller, c, g);
        self.draw_shadow(controller, c, g);
    }

    fn draw_board<G: Graphics>(&self, _controller: &GameController, c: &Context, g: &mut G) {
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
        let cell_border = Line::new(self.settings.cb_color, self.settings.cb_radius);

        // Draw horizontal lines
        for i in 0..20 {
            let x = self.settings.position[0];
            let y = self.settings.position[1] + i as f64 / 20.0 * self.settings.size[1];
            let x2 = self.settings.position[0] + self.settings.size[0];
            let y2 = self.settings.position[1] + i as f64 / 20.0 * self.settings.size[1];

            let line = [x, y, x2, y2];
            cell_border.draw(line, &c.draw_state, c.transform, g);
        }

        // Draw vertical lines
        for i in 0..10 {
            let x = self.settings.position[0] + i as f64 / 10.0 * self.settings.size[0];
            let y = self.settings.position[1];
            let x2 = self.settings.position[0] + i as f64 / 10.0 * self.settings.size[0];
            let y2 = self.settings.position[1] + self.settings.size[1];

            let line = [x, y, x2, y2];
            cell_border.draw(line, &c.draw_state, c.transform, g);
        }

    }

    fn draw_state<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {
        let ref board = controller.game.board;

        for (i, row) in board.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if let None = col { continue; }

                let x = self.settings.position[0] + i as f64 / 10.0 * self.settings.size[0];
                let y = self.settings.position[1] + j as f64 / 20.0 * self.settings.size[1];
                let x2 = x + self.settings.size[0];
                let y2 = y + self.settings.size[1];

                let rect = [x, y, x2, y2];

                Rectangle::new(col.unwrap().color())
                    .draw(rect, &c.draw_state, c.transform, g);
            }
        }
    }

    fn draw_current_next_and_holding<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {
        // Draw holding
        let ref holding = controller.game.holding;

        let holding_border = [
            self.settings.position[0] - 80.0, self.settings.position[1] - 10.0,
            40.0 + 20.0, 40.0 + 20.0,
        ];

        Rectangle::new(self.settings.border_color)
            .draw(holding_border, &c.draw_state, c.transform, g);

        let holding_background = [
            self.settings.position[0] - 70.0, self.settings.position[1],
            40.0, 40.0,
        ];

        Rectangle::new(self.settings.background_color)
            .draw(holding_background, &c.draw_state, c.transform, g);


        // Draw current tetrimino border and background
        let current_border = [
            self.settings.position[0] + self.settings.size[0] + 20.0, self.settings.position[1] - 10.0,
            50.0 + 20.0, 50.0 + 20.0,
        ];

        Rectangle::new(self.settings.border_color)
            .draw(current_border, &c.draw_state, c.transform, g);

        let current_background = [
            self.settings.position[0] + self.settings.size[0] + 30.0, self.settings.position[1],
            50.0, 50.0,
        ];

        Rectangle::new(self.settings.background_color)
            .draw(current_background, &c.draw_state, c.transform, g);

        // Draw next tetrimino border and background
        let next_border = [
            self.settings.position[0] + self.settings.size[0] + 20.0, self.settings.position[1] - 10.0 + 80.0,
            50.0 + 20.0, 50.0 * 5.0 + 10.0 + 5.0,
        ];

        Rectangle::new(self.settings.border_color)
            .draw(next_border, &c.draw_state, c.transform, g);

        let next_background = [
            self.settings.position[0] + self.settings.size[0] + 30.0, self.settings.position[1] + 80.0,
            50.0, 50.0 * 5.0 - 10.0 + 5.0,
        ];

        Rectangle::new(self.settings.background_color)
            .draw(next_background, &c.draw_state, c.transform, g);

        // Draw next tetrimino separators
        let x = self.settings.position[0] + self.settings.size[0] + 30.0;
        let x2 = x + 50.0;

        let cell_border = Line::new(self.settings.cb_color, self.settings.cb_radius);

        for i in 0..5 {
            let y = self.settings.position[1] + 80.0 + i as f64 * 50.0;
            let y2 = self.settings.position[1] + 80.0 + i as f64 * 50.0;

            let line = [x, y, x2, y2];
            cell_border.draw(line, &c.draw_state, c.transform, g);
        }
    }

    fn draw_shadow<G: Graphics>(&self, _controller: &GameController, c: &Context, g: &mut G) {

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
