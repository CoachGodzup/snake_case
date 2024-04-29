use std::num;
use bracket_lib::terminal::Point;

use super::{fruit::Fruit, player::{Direction}};

pub struct Ai {
}

impl Ai {
    pub fn new () -> Self {
        Ai {}
    }

    pub fn get_next_move (&self, fruit: Fruit, position: Point, actual_direction: Direction) -> Direction {
        let desired_move_y = self.move_vertically(fruit, position);
        let desired_move_x = self.move_horizontally(fruit, position);

        if desired_move_y.is_some() && !Ai::is_opposite(desired_move_y.unwrap(), actual_direction) {
            desired_move_y.unwrap()
        } else if desired_move_x.is_some() && !Ai::is_opposite(desired_move_x.unwrap(), actual_direction){
            desired_move_x.unwrap()
        } else {
            actual_direction
        }

        // TODO: select two direction, and moving to the desired one, preferred y
    }

    fn move_vertically(&self, fruit: Fruit, position: Point) -> Option<Direction> {
        if fruit.position.y < position.y {
            Some(Direction::Up)
        } else if fruit.position.y > position.y {
            Some(Direction::Down)
        } else {
            None
        }
    }
    
    fn move_horizontally(&self, fruit: Fruit, position: Point) -> Option<Direction> {
        if fruit.position.x < position.x {
            Some(Direction::Left)
        } else if fruit.position.x > position.x {
            Some(Direction::Right)
        } else {
            None
        }
    }

    fn is_opposite(dir1: Direction, dir2: Direction) -> bool {
        dir1 == Direction::Up && dir2 == Direction::Down ||
        dir1 == Direction::Down && dir2 == Direction::Up ||
        dir1 == Direction::Left && dir2 == Direction::Right ||
        dir1 == Direction::Right && dir2 == Direction::Left
    }
}