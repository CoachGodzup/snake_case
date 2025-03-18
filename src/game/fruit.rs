use bracket_lib::{color::BLANCHED_ALMOND, terminal::{to_cp437, BTerm, Point}};

use super::{map::get_random_position, snake_game_state::BACKGROUND_COLOR};
#[derive(Clone, Copy, PartialEq)]

pub struct Fruit {
    pub position: Point,
}

pub fn fruit_builder() -> Fruit {
    let random_position = get_random_position();
    Fruit::new(random_position.x as usize, random_position.y as usize)
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