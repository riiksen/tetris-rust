use ggez::graphics::{ self, Color, Image };
use ggez::{ Context };

use crate::tetrimino;

pub struct MarathonView {
    pub position: [f32; 2],
    pub size: [f32; 2],
    pub background_color: Color,
    pub border_color: Color,
    
    // Cell border color
    pub cb_color: Color,

    // Cell border radius
    pub cb_radius: f32,

    pub i_image: Image,
    pub j_image: Image,
    pub l_image: Image,
    pub o_image: Image,
    pub s_image: Image,
    pub t_image: Image,
    pub z_image: Image,
}

impl MarathonView {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            position: [85., 40.],
            size: [300., 500.],
            background_color: [0.4, 0.4, 0.4, 1.0].into(),
            border_color: [0.0, 0.0, 0.0, 1.0].into(),
            cb_color: [0.2, 0.2, 0.2, 1.0].into(),
            cb_radius: 1.0,

            i_image: graphics::Image::new(ctx, "/i.png").unwrap(),
            j_image: graphics::Image::new(ctx, "/j.png").unwrap(),
            l_image: graphics::Image::new(ctx, "/l.png").unwrap(),
            o_image: graphics::Image::new(ctx, "/o.png").unwrap(),
            s_image: graphics::Image::new(ctx, "/s.png").unwrap(),
            t_image: graphics::Image::new(ctx, "/t.png").unwrap(),
            z_image: graphics::Image::new(ctx, "/z.png").unwrap(),
        }
    }

    pub fn render_board(&mut self, ctx: &mut Context) {
        // Draw matrix borders
        let board_border = graphics::Rect::new(
            self.position[0] - 10.0, self.position[1] - 10.0,
            self.size[0] + 20.0, self.size[1] + 20.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), board_border, self.border_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        // Draw matrix background
        let board_rect = graphics::Rect::new(
            self.position[0], self.position[1],
            self.size[0], self.size[1],
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), board_rect, self.background_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        // Draw cell borders

        // Draw horizontal lines
        for i in 0..20 {
            let x = self.position[0];
            let y = self.position[1] + i as f32 / 20.0 * self.size[1];
            let x2 = self.position[0] + self.size[0];
            let y2 = self.position[1] + i as f32 / 20.0 * self.size[1];

            // let line = [x, y, x2, y2];

            let line = [
                ggez::mint::Point2 { x: x, y: y },
                ggez::mint::Point2 { x: x2, y: y2 },
            ];

            let mesh = graphics::Mesh::new_line(ctx, &line, self.cb_radius, self.cb_color).unwrap();

            graphics::draw(
                ctx,
                &mesh,
                graphics::DrawParam::default()
            ).unwrap()
        }

        // Draw vertical lines
        for i in 0..10 {
            let x = self.position[0] + i as f32 / 10.0 * self.size[0];
            let y = self.position[1];
            let x2 = self.position[0] + i as f32 / 10.0 * self.size[0];
            let y2 = self.position[1] + self.size[1];

            // let line = [x, y, x2, y2];
            let line = [
                ggez::mint::Point2 { x: x, y: y },
                ggez::mint::Point2 { x: x2, y: y2 },
            ];

            let mesh = graphics::Mesh::new_line(ctx, &line, self.cb_radius, self.cb_color).unwrap();

            graphics::draw(
                ctx,
                &mesh,
                graphics::DrawParam::default()
            ).unwrap();
        }
    }

    pub fn render_current_next_and_holding(&mut self, ctx: &mut Context) {
        // Draw holding
        let holding_border = graphics::Rect::new(
            self.position[0] - 80.0, self.position[1] - 10.0,
            40.0 + 20.0, 40.0 + 20.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), holding_border, self.border_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        let holding_background = graphics::Rect::new(
            self.position[0] - 70.0, self.position[1],
            40.0, 40.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), holding_background, self.background_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();


        // Draw current tetrimino border and background
        let current_border = graphics::Rect::new(
            self.position[0] + self.size[0] + 20.0, self.position[1] - 10.0,
            50.0 + 20.0, 50.0 + 20.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), current_border, self.border_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        let current_background = graphics::Rect::new(
            self.position[0] + self.size[0] + 30.0, self.position[1],
            50.0, 50.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), current_background, self.background_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        // Draw next tetrimino border and background
        let next_border = graphics::Rect::new(
            self.position[0] + self.size[0] + 20.0, self.position[1] - 10.0 + 80.0,
            50.0 + 20.0, 50.0 * 5.0 + 10.0 + 5.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), next_border, self.border_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        let next_background = graphics::Rect::new(
            self.position[0] + self.size[0] + 30.0, self.position[1] + 80.0,
            50.0, 50.0 * 5.0 - 10.0 + 5.0,
        );

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), next_background, self.background_color).unwrap();

        graphics::draw(
            ctx,
            &mesh,
            graphics::DrawParam::default()
        ).unwrap();

        // Draw next tetrimino separators
        let x = self.position[0] + self.size[0] + 30.0;
        let x2 = x + 50.0;

        for i in 0..5 {
            let y = self.position[1] + 80.0 + i as f32 * 50.0;
            let y2 = self.position[1] + 80.0 + i as f32 * 50.0;

            // let line = [x, y, x2, y2];

            let line = [
                ggez::mint::Point2 { x: x, y: y },
                ggez::mint::Point2 { x: x2, y: y2 },
            ];

            let mesh = graphics::Mesh::new_line(ctx, &line, self.cb_radius, self.cb_color).unwrap();

            graphics::draw(
                ctx,
                &mesh,
                graphics::DrawParam::default()
            ).unwrap()
        }
    }

    pub fn render_state(
        &mut self,
        ctx: &mut Context,
        // (matrix, current_tetrimino, next_tetrimino, holding)
        game: ([[Option<tetrimino::Type>; 20]; 10], tetrimino::Type, &Vec<tetrimino::Type>, Option<tetrimino::Type>))
    {
        // Draw grid state
        for (i, row) in game.0.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if let None = col { continue; }

                println!("{}, {}", i, j);

                let x = self.position[0] + i as f32 / 10.0 * self.size[0];
                let y = self.position[1] + j as f32 / 20.0 * self.size[1];

                let x2 = self.position[0] + (i + 1) as f32 / 10.0 * self.size[0];
                let y2 = self.position[1] + (j + 1) as f32 / 20.0 * self.size[1];

                println!("{}, {}, {}, {}", x, y, x2, y2);

                // let x = self.position[0] + i as f32 / 10.0 * self.size[0];
                // let y = self.position[1];
                // let x2 = self.position[0] + i as f32 / 10.0 * self.size[0];
                // let y2 = self.position[1] + self.size[1];


                // let x = self.position[0];
                // let y = self.position[1] + i as f32 / 20.0 * self.size[1];
                // let x2 = self.position[0] + self.size[0];
                // let y2 = self.position[1] + i as f32 / 20.0 * self.size[1];

                let rect = graphics::Rect::new(x, y, x2, y2);
                let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, col.unwrap().color()).unwrap();

                graphics::draw(
                    ctx,
                    &mesh,
                    graphics::DrawParam::default()
                ).unwrap();
            }
        }

        // Draw current and next tetriminos state
        let x = self.position[0] + self.size[0] + 30.0;
        let y = self.position[1];
        let size = 50.0;

        self.render_tetrimino(ctx, (x, y, size), game.1);

        let x = self.position[0] + self.size[0] + 30.0;
        let size = 50.0;
        for i in 0..5 {
            let y = self.position[1] + 80.0 + i as f32 * 50.0;

            self.render_tetrimino(ctx, (x, y, size), game.2[i]);
        }

        // holding state
        if let Some(holding) = game.3 {
            let x = self.position[0] - 70.0;
            let y = self.position[1];
            let size = 40.0;

            self.render_tetrimino(ctx, (x, y, size), holding);
        }
    }

    // TODO: Implement
    pub fn render_current_and_shadow(&mut self, _ctx: &mut Context, _game: (tetrimino::Type, u8)) {

    }

    fn render_tetrimino(&mut self, ctx: &mut Context, pos: (f32, f32, f32), t: tetrimino::Type) {
        use tetrimino::Type;

        let image = match t {
            Type::I => &self.i_image,
            Type::L => &self.l_image,
            Type::J => &self.j_image,
            Type::S => &self.s_image,
            Type::Z => &self.z_image,
            Type::O => &self.o_image,
            Type::T => &self.t_image,
        };

        let position = ggez::mint::Point2 { x: pos.0, y: pos.1 };

        graphics::draw(
            ctx,
            image,
            graphics::DrawParam::default().dest(position).scale(ggez::mint::Vector2 { x: 0.20, y: 0.20 })
        ).unwrap();
    }
}
