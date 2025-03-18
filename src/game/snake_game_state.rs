use bracket_lib::color::{GREEN_YELLOW, NAVY_BLUE, REBECCA_PURPLE};
use bracket_lib::terminal::VirtualKeyCode;
use bracket_lib::terminal::BTerm;

use super::ai::Ai;
use super::fruit::{fruit_builder, Fruit};
use super::player::Player;
use super::map::{get_random_position, Map};
use super::map::pacman_effect;

pub const BACKGROUND_COLOR: (u8, u8, u8) = NAVY_BLUE;
pub const PLAYER_COLOR: (u8, u8, u8) = GREEN_YELLOW;
pub const ENEMY_COLOR: (u8, u8, u8) = REBECCA_PURPLE;

pub struct SnakeGameState {
    player: Player,
    enemy: Player,
    ai: Ai,
    map: Map,
    fruit: Fruit,
    pub is_ended: bool,
    pub final_score: usize,
}

impl SnakeGameState {
    pub fn new() -> Self {
        let enemy = Player::new(ENEMY_COLOR, get_random_position());

        SnakeGameState {
            player: Player::new(PLAYER_COLOR, get_random_position()),
            enemy,
            map: Map::new(),
            ai: Ai::new(),
            fruit: Fruit::new(65, 10),
            is_ended: false,
            final_score: 0,
        }
    }

    fn render(& mut self, ctx: &mut BTerm) {
        self.map.render(ctx);
        self.fruit.render(ctx);
        self.player.render(ctx);
        self.enemy.render(ctx);
        self.gui_render(ctx);
    }

    fn gui_render(& mut self, ctx: &mut BTerm) {
        ctx.print_color_centered(0, PLAYER_COLOR, BACKGROUND_COLOR, &format!("Your score is: {}", self.player.get_length()));
        ctx.print_color_centered(1, ENEMY_COLOR, BACKGROUND_COLOR, &format!("Purple score is: {}", self.enemy.get_length()));
    }

    pub fn play(&mut self,  ctx: &mut BTerm) {
        self.player_inputs_handler(ctx);
        self.move_player();
        self.move_enemy();
        self.render(ctx);
    }

    fn end_game(&mut self) {
        self.is_ended = true;
        self.final_score = self.player.get_length();
    }

    fn move_snake(& mut self) {
        let mut new_pos = self.player.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = pacman_effect(new_pos);
        }

        if self.player.collide(new_pos) {
            self.end_game(); // TODO param
        } else if self.map.can_enter(new_pos) {
            let is_eating = new_pos.x == self.fruit.position.x && new_pos.y == self.fruit.position.y;
            let is_eating_enemy = self.enemy.collide(new_pos);
            self.player.move_player(new_pos, is_eating);
            if is_eating {
                self.player.add_length(1);
                self.fruit = fruit_builder();
            }
            if is_eating_enemy {
                self.player.add_length(self.enemy.get_length());
                self.enemy.set_length(4);
                self.respawn_enemy();
            }
        }
    }

    fn move_player(& mut self) {
        self.move_snake();
    }

    fn move_enemy(& mut self) { // TODO Params
        self.enemy.change_direction(self.ai.get_next_move(self.fruit, self.enemy.get_next_pos_player(), self.enemy.get_direction())); // TODO: passa solo la direzione e la posizione attuale?

        let mut new_pos = self.enemy.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = pacman_effect(new_pos);
        }

        if self.enemy.collide(new_pos) {
            self.respawn_enemy(); // TODO param
        } else if self.player.collide(new_pos){
            self.enemy.add_length(self.player.get_length());
            self.respawn_player();

        } else if self.map.can_enter(new_pos) {
            let is_eating = new_pos.x == self.fruit.position.x && new_pos.y == self.fruit.position.y;
            self.enemy.move_player(new_pos, is_eating);
            if is_eating {
                self.enemy.add_length(1);
                self.fruit = fruit_builder();
            }
        }
    }


    fn respawn_player(& mut self) {
        self.player = Player::new(PLAYER_COLOR, get_random_position());
    }

    fn respawn_enemy(& mut self) {
        self.enemy = Player::new(ENEMY_COLOR, get_random_position());
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