use bracket_lib::prelude::Point;
use super::player::{Player, SnakeCollision};
use super::map::{Map, pacman_effect, get_random_position};
use super::fruit::{Fruit, fruit_builder};

pub fn respawn_snake(color: (u8, u8, u8)) -> Player {
    Player::new(color.into(), get_random_position())
} 