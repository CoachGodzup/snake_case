use bracket_lib::prelude::Point;

use super::{fruit::Fruit, player::Direction};

pub trait AiBehavior {
    fn get_next_move(&self, fruit: Fruit, current_pos: Point, player_pos: Point, current_direction: Direction) -> Direction;
}

pub struct Ai {
    behavior: Box<dyn AiBehavior>,
}

impl Ai {
    pub fn new() -> Self {
        Ai {
            behavior: Box::new(ChaseFruitAi {})
        }
    }

    pub fn with_behavior(behavior: Box<dyn AiBehavior>) -> Self {
        Ai { behavior }
    }

    pub fn get_next_move(&self, fruit: Fruit, current_pos: Point, player_pos: Point, current_direction: Direction) -> Direction {
        self.behavior.get_next_move(fruit, current_pos, player_pos, current_direction)
    }
}

pub struct ChaseFruitAi {}

impl AiBehavior for ChaseFruitAi {
    fn get_next_move(&self, fruit: Fruit, current_pos: Point, _player_pos: Point, current_direction: Direction) -> Direction {
        let current_x = current_pos.x;
        let current_y = current_pos.y;
        let fruit_x = fruit.position.x;
        let fruit_y = fruit.position.y;

        // Calcola la direzione verso il frutto
        if current_x < fruit_x && current_direction != Direction::Left {
            Direction::Right
        } else if current_x > fruit_x && current_direction != Direction::Right {
            Direction::Left
        } else if current_y < fruit_y && current_direction != Direction::Up {
            Direction::Down
        } else if current_y > fruit_y && current_direction != Direction::Down {
            Direction::Up
        } else {
            current_direction
        }
    }
}

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