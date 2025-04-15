use super::ai::AiBehavior;

use crate::game::fruit::Fruit;
use crate::game::player::Direction;

use bracket_lib::terminal::Point;

pub struct ChasePlayerAi {}

impl AiBehavior for ChasePlayerAi {
    fn get_next_move(&self, _fruit: Fruit, position: Point, player_position: Point, actual_direction: Direction) -> Direction {
        let desired_move_y = self.move_vertically(position, player_position);
        let desired_move_x = self.move_horizontally(position, player_position);

        if desired_move_y.is_some() && !ChasePlayerAi::is_opposite(desired_move_y.unwrap(), actual_direction) {
            desired_move_y.unwrap()
        } else if desired_move_x.is_some() && !ChasePlayerAi::is_opposite(desired_move_x.unwrap(), actual_direction){
            desired_move_x.unwrap()
        } else {
            actual_direction
        }
    }
}

impl ChasePlayerAi {
    fn move_vertically(&self, position: Point, player_position: Point) -> Option<Direction> {
        if player_position.y < position.y {
            Some(Direction::Up)
        } else if player_position.y > position.y {
            Some(Direction::Down)
        } else {
            None
        }
    }
    
    fn move_horizontally(&self, position: Point, player_position: Point) -> Option<Direction> {
        if player_position.x < position.x {
            Some(Direction::Left)
        } else if player_position.x > position.x {
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