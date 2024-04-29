use bracket_lib::color::{NAVY_BLUE, WHITE};
use bracket_lib::random::RandomNumberGenerator;
use bracket_lib::terminal::{Point, VirtualKeyCode};
use bracket_lib::{color::ROYALBLUE, terminal::BTerm};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::fruit::Fruit;
use super::player::Player;
use super::map::Map;

pub const BACKGROUND_COLOR: (u8, u8, u8) = NAVY_BLUE;

pub struct SnakeGameState {
    player: Player,
    map: Map,
    fruit: Fruit,
    pub is_ended: bool,
    pub final_score: usize,

}

impl SnakeGameState {
    pub fn new() -> Self {
        SnakeGameState {
            player: Player::new(),
            map: Map::new(),
            fruit: Fruit::new(65, 10),
            is_ended: false,
            final_score: 0,
        }
    }

    pub fn restart(& mut self) {
        self.player = Player::new();
        self.map = Map::new();
        self.fruit = self.fruit_builder();
        self.is_ended = false;
    }

    pub fn fruit_builder(&mut self) -> Fruit {

        let mut rng = RandomNumberGenerator::new();
        // get all map places
        // remove places occupied by player and tail
        // pick random

        Fruit::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT))
    }

    fn render(& mut self, ctx: &mut BTerm) {
        self.map.render(ctx);
        self.fruit.render(ctx);
        self.player.render(ctx);
        self.gui_render(ctx);
    }

    fn gui_render(& mut self, ctx: &mut BTerm) {
        ctx.print_color_centered(0, WHITE, BACKGROUND_COLOR, &format!("Your score is: {}", self.player.get_length()));
    }

    pub fn play(&mut self,  ctx: &mut BTerm) {
        self.player_inputs_handler(ctx);
        self.move_player();
        self.render(ctx);
    
    }

    fn end_game(&mut self) {
        self.is_ended = true;
        self.final_score = self.player.get_length();
    }
    
    fn pacman_effect(pos: Point) -> Point {
        let mut new_pos = pos.clone();

        if new_pos.x < 0 {
            new_pos.x = SCREEN_WIDTH as i32;
        } else if new_pos.x  as i32 > SCREEN_WIDTH as i32 {
            new_pos.x = 0;
        }

        if new_pos.y < 0 {
            new_pos.y = SCREEN_HEIGHT  as i32;
        } else if new_pos.y as i32 > SCREEN_HEIGHT as i32{
            new_pos.y = 0;
        }

        new_pos
    }

    fn move_player(& mut self) {
        let mut new_pos = self.player.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = SnakeGameState::pacman_effect(new_pos);
        }

        if self.player.collide(new_pos) {
            self.end_game();
        } else if self.map.can_enter(new_pos) {
            let is_eating = new_pos.x == self.fruit.position.x && new_pos.y == self.fruit.position.y;
            self.player.move_player(new_pos, is_eating);
            if is_eating {
                self.fruit = self.fruit_builder();
            }
        }
    }

    pub fn player_inputs_handler(& mut self, ctx: & mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.player.change_direction(super::player::Direction::Up),
                VirtualKeyCode::Down => self.player.change_direction(super::player::Direction::Down),
                VirtualKeyCode::Left => self.player.change_direction(super::player::Direction::Left),
                VirtualKeyCode::Right => self.player.change_direction(super::player::Direction::Right),
                VirtualKeyCode::Q => self.end_game(),
                _ => {},
            }
        }
    }

}