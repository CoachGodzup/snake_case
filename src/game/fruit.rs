use bracket_lib::{color::{ALICEBLUE, BLANCHED_ALMOND}, terminal::{to_cp437, BTerm, Point}};

use super::snake_game_state::BACKGROUND_COLOR;

pub struct Fruit {
    pub position: Point,
}

impl Fruit {
    pub fn new(x: usize, y: usize) -> Self {
        Fruit {
            position: Point::new(x, y)
        }
    }

    pub fn render(&self, ctx: & mut BTerm) {
        ctx.set(self.position.x, self.position.y, BLANCHED_ALMOND, BACKGROUND_COLOR, to_cp437('@'))
    }
}