use bracket_lib::prelude::Point;

use crate::game::fruit::Fruit;
use crate::game::player::Direction;

pub trait AiBehavior {
    fn get_next_move(&self, fruit: Fruit, current_pos: Point, player_pos: Point, current_direction: Direction) -> Direction;
}

pub struct Ai {
    behavior: Box<dyn AiBehavior>,
}

impl Ai {
    pub fn with_behavior(behavior: Box<dyn AiBehavior>) -> Self {
        Ai { behavior }
    }

    pub fn get_next_move(&self, fruit: Fruit, current_pos: Point, player_pos: Point, current_direction: Direction) -> Direction {
        self.behavior.get_next_move(fruit, current_pos, player_pos, current_direction)
    }
}
