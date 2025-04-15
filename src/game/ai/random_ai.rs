use super::ai::AiBehavior;

use crate::game::fruit::Fruit;
use crate::game::player::Direction;

use bracket_lib::terminal::Point;
use bracket_lib::random::RandomNumberGenerator;
pub struct RandomAi {}

// function that picks a random direction
fn pick_random_direction() -> Direction {
    let mut rng = RandomNumberGenerator::new();
    let direction = rng.range(0, 3);
    match direction {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => panic!("Invalid direction"),
    }
}

fn is_opposite(dir1: Direction, dir2: Direction) -> bool {
    dir1 == Direction::Up && dir2 == Direction::Down ||
    dir1 == Direction::Down && dir2 == Direction::Up ||
    dir1 == Direction::Left && dir2 == Direction::Right ||
    dir1 == Direction::Right && dir2 == Direction::Left
}

impl AiBehavior for RandomAi {
    fn get_next_move(&self, _fruit: Fruit, _position: Point, _player_position: Point, actual_direction: Direction) -> Direction {
        loop {
            let new_direction: Direction = pick_random_direction();
            if !is_opposite(new_direction, actual_direction) {
                return new_direction;
            }
        }
    }
}
