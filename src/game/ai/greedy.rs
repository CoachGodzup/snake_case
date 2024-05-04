use bracket_lib::terminal::Point;

use crate::game::{snake_game_state::SnakeGameState, player::Direction};

use super::Ai;

pub struct Greedy {

}

fn move_vertically(target: Point, position: Point) -> Option<Direction> {
    if target.y < position.y {
        Some(Direction::Up)
    } else if target.y > position.y {
        Some(Direction::Down)
    } else {
        None
    }
}

fn move_horizontally(target: Point, position: Point) -> Option<Direction> {
    if target.x < position.x {
        Some(Direction::Left)
    } else if target.x > position.x {
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


/* TODO IMPORTA IL TRATTO AI IN QUALCHE MODO
GENERALIZZA
CREA AI AGGRESSIVA
CREA NUOVO GIOCATORE CON IA AGGRESSIVA

SISTEMA DEL KARMA */
impl Ai for Greedy {
    fn new () -> Self {
        Greedy {}
    }

    fn get_next_move (&self, state: &SnakeGameState ) -> Direction {

        let fruit = state.get_fruit();
        let position = state.get_enemy().get_next_pos_player();
        let actual_direction = state.get_enemy().get_direction();

        let desired_move_y = move_vertically(fruit.position, position);
        let desired_move_x = move_horizontally(fruit.position, position);

        if desired_move_y.is_some() && !is_opposite(desired_move_y.unwrap(), actual_direction) {
            desired_move_y.unwrap()
        } else if desired_move_x.is_some() && !is_opposite(desired_move_x.unwrap(), actual_direction){
            desired_move_x.unwrap()
        } else {
            actual_direction
        }

        // TODO: select two direction, and moving to the desired one, preferred y
    }

}