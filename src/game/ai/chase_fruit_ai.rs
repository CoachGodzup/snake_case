use super::ai::AiBehavior;

use crate::game::fruit::Fruit;
use crate::game::player::Direction;

use bracket_lib::terminal::Point;

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