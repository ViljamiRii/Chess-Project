#![allow(non_snake_case)]

mod Core;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use crate::Core::Board::board::ChessGame; // Import the ChessGame struct from the board module

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("chess_engine", "Viljami Riihimäki")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess Engine Project"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1800.0, 1000.0).resizable(true))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let chess = ChessGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, chess);
}