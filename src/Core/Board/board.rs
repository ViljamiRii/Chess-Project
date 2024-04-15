use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::EventHandler;

pub struct ChessGame {
    // Your state here...
}

impl ChessGame {
    pub fn new(_ctx: &mut Context) -> ChessGame {
        // Load/create resources such as images here.
        ChessGame {
            // ...
        }
    }
}

impl EventHandler for ChessGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, None);

        let screen_size = ctx.gfx.window().inner_size();
        let (screen_width, screen_height) = (screen_size.width as f32, screen_size.height as f32);
        let board_size = 8;
        let square_size = screen_width.min(screen_height) / board_size as f32; // Calculate square size based on the minimum of screen width and height

        let offset_x = (screen_width - square_size * board_size as f32) / 2.0; // Calculate offset to center the board on the screen
        let offset_y = (screen_height - square_size * board_size as f32) / 2.0;

        for i in 0..board_size {
            for j in 0..board_size {
                let color = if (i + j) % 2 == 0 {
                    Color::from_rgb(255, 206, 158) // Light color
                } else {
                    Color::from_rgb(209, 139, 71) // Dark color
                };

                let rect = graphics::Rect::new(
                    i as f32 * square_size + offset_x,
                    j as f32 * square_size + offset_y,
                    square_size,
                    square_size,
                );

                let square = graphics::Mesh::new_rectangle(
                    ctx, 
                    graphics::DrawMode::fill(), 
                    rect, 
                    color)?;

                canvas.draw(&square, graphics::DrawParam::default());
            }
        }
        
        // Draw code here...
        canvas.finish(ctx)
    }
}