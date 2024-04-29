use bracket_lib::terminal::{main_loop, BError, BTermBuilder, BTerm};
use crate::state::{GameMode, State};

mod game;
mod state;
mod menu;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: usize = 80;
    pub const SCREEN_HEIGHT: usize = 50;
}

use prelude::*;


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}
fn main() -> BError {

    let context = BTermBuilder::simple80x50()
        .with_title("snake_case")
        .with_fps_cap(15.0)
        .with_tile_dimensions(15, 15)
        .build()?;

    main_loop(context, State::new())
}
