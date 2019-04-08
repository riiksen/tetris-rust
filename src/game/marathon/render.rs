use ggez::graphics::{ self, Color };
use ggez::{ Context };

use crate::tetrimino;

pub struct ViewSettings {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub background_color: Color,
    pub border_color: Color,
    
    // Cell border color
    pub cb_color: Color,

    // Cell border radius
    pub cb_radius: f32,
}

impl ViewSettings {
    pub fn new() -> Self {
        Self {
            position: [85., 40.],
            size: [300., 500.],
            background_color: [0.4, 0.4, 0.4, 1.0].into(),
            border_color: [0.0, 0.0, 0.0, 1.0].into(),
            cb_color: [0.2, 0.2, 0.2, 1.0].into(),
            cb_radius: 1.0,
        }
    }
}

pub fn render_board(ctx: &mut Context, view_settings: &ViewSettings) {
    // Draw matrix borders
    let board_border = graphics::Rect::new(
        view_settings.position[0] - 10.0, view_settings.position[1] - 10.0,
        view_settings.size[0] + 20.0, view_settings.size[1] + 20.0,
    );

    let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), board_border, view_settings.border_color).unwrap();

    graphics::draw(
        ctx,
        &mesh,
        graphics::DrawParam::default()
    );

    // Draw matrix background
    let board_rect = graphics::Rect::new(
        view_settings.position[0], view_settings.position[1],
        view_settings.size[0], view_settings.size[1],
    );

    let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), board_rect, view_settings.background_color).unwrap();

    graphics::draw(
        ctx,
        &mesh,
        graphics::DrawParam::default()
    );

    // Draw cell borders

    // Draw horizontal lines
    for i in 0..20 {
        let x = view_settings.position[0];
        let y = view_settings.position[1] + i as f32 / 20.0 * view_settings.size[1];
        let x2 = view_settings.position[0] + view_settings.size[0];
        let y2 = view_settings.position[1] + i as f32 / 20.0 * view_settings.size[1];

        // let line = [x, y, x2, y2];

        let line = [
            ggez::mint::Point2 { x: x, y: y },
            ggez::mint::Point2 { x: x2, y: y2 },
        ];

        let mesh = graphics::Mesh::new_line(ctx, &line, view_settings.cb_radius, view_settings.cb_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        );
       }

    // Draw vertical lines
    for i in 0..10 {
        let x = view_settings.position[0] + i as f32 / 10.0 * view_settings.size[0];
        let y = view_settings.position[1];
        let x2 = view_settings.position[0] + i as f32 / 10.0 * view_settings.size[0];
        let y2 = view_settings.position[1] + view_settings.size[1];

        // let line = [x, y, x2, y2];
        let line = [
            ggez::mint::Point2 { x: x, y: y },
            ggez::mint::Point2 { x: x2, y: y2 },
        ];

        let mesh = graphics::Mesh::new_line(ctx, &line, view_settings.cb_radius, view_settings.cb_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        );
    }
}

// TODO: Implement
pub fn render_current_next_and_holding(_ctx: &mut Context, view_settings: &ViewSettings) {

}

// TODO: Implement
pub fn render_state(ctx: &mut Context, view_settings: &ViewSettings, game: &super::Marathon) {
    // Draw grid state
    for (i, row) in game.matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if let None = col { continue; }

            let x = view_settings.position[0] + i as f32 / 10.0 * view_settings.size[0];
            let y = view_settings.position[1] + j as f32 / 20.0 * view_settings.size[1];
            let x2 = x + view_settings.size[0];
            let y2 = y + view_settings.size[1];

            let rect = graphics::Rect::new(x, y, x2, y2);
            let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, col.unwrap().color()).unwrap();

            graphics::draw(
                ctx,
                &mesh,
                graphics::DrawParam::default()
            );
        }
    }

    // Draw current and next tetriminos state
    let x = view_settings.position[0] + view_settings.size[0] + 30.0;
    let y = view_settings.position[1];
    let size = 50.0;

    render_tetrimino(ctx, view_settings, (x, y, size), game.current_tetrimino);

    let x = view_settings.position[0] + view_settings.size[0] + 30.0;
    let size = 50.0;
    for i in 0..5 {
        let y = view_settings.position[1] + 80.0 + i as f32 * 50.0;

        render_tetrimino(ctx, view_settings, (x, y, size), game.next_tetriminos[i]);
    }

    // holding state
    if let Some(holding) = game.holding {
        let x = view_settings.position[0] - 70.0;
        let y = view_settings.position[1];
        let size = 40.0;

        render_tetrimino(ctx, view_settings, (x, y, size), holding);
    }
}

// TODO: Implement
pub fn render_current_and_shadow(_ctx: &mut Context, view_settings: &ViewSettings) {

}

// TODO: Implement
fn render_tetrimino(_ctx: &mut Context, view_settings: &ViewSettings, pos: (f32, f32, f32), t: tetrimino::Type) {

}
