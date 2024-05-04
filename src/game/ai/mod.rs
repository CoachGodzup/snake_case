use super::{player::Direction, snake_game_state::SnakeGameState};

pub mod greedy;
pub mod berserk;

pub trait Ai {
    fn new() -> Self;
    fn get_next_move (&self, state: &SnakeGameState ) -> Direction;
}
