use bracket_lib::color::{GREEN_YELLOW, NAVY_BLUE, REBECCA_PURPLE, YELLOW, ALICE_BLUE};
use bracket_lib::terminal::VirtualKeyCode;
use bracket_lib::terminal::BTerm;

use super::ai::ai::Ai;
use super::fruit::{fruit_builder, Fruit};
use super::player::{Player, SnakeCollision};
use super::map::{get_random_position, Map};
use super::map::pacman_effect;

pub const BACKGROUND_COLOR: (u8, u8, u8) = NAVY_BLUE;
pub const PLAYER_COLOR: (u8, u8, u8) = GREEN_YELLOW;
pub const ENEMY_COLOR: (u8, u8, u8) = REBECCA_PURPLE;
pub const CPU_PLAYER_COLOR: (u8, u8, u8) = YELLOW;
pub const RANDOM_PLAYER_COLOR: (u8, u8, u8) = ALICE_BLUE;

pub struct SnakeGameState {
    player: Player,
    enemy: Player,
    cpu_player: Player,
    random_player: Player,
    ai: Ai,
    cpu_ai: Ai,
    random_ai: Ai,
    map: Map,
    fruit: Fruit,
    pub is_paused: bool,
    pub is_ended: bool,
    pub final_score: usize,
    pub enemy_score: usize,
    pub cpu_score: usize,
    pub random_score: usize,
}

impl SnakeGameState {
    pub fn new() -> Self {
        let enemy = Player::new(ENEMY_COLOR.into(), get_random_position());
        let cpu_player = Player::new(CPU_PLAYER_COLOR.into(), get_random_position());
        let random_player = Player::new(RANDOM_PLAYER_COLOR.into(), get_random_position());

        SnakeGameState {
            player: Player::new(PLAYER_COLOR.into(), get_random_position()),
            enemy,
            cpu_player,
            random_player,
            map: Map::new(),
            ai: Ai::with_behavior(Box::new(super::ai::chase_player_ai::ChasePlayerAi {})),
            cpu_ai: Ai::with_behavior(Box::new(super::ai::chase_fruit_ai::ChaseFruitAi {})),
            random_ai: Ai::with_behavior(Box::new(super::ai::random_ai::RandomAi {})),
            fruit: Fruit::new(65, 10),
            is_ended: false,
            is_paused: false,
            final_score: 0,
            enemy_score: 0,
            cpu_score: 0,
            random_score: 0,
        }
    }

    fn render_pause(& mut self, ctx: &mut BTerm) {
        ctx.print_color_centered(
            15,
            PLAYER_COLOR,
            BACKGROUND_COLOR,
            "Game Paused"
        );
    }

    fn render(& mut self, ctx: &mut BTerm) {
        self.map.render(ctx);
        self.fruit.render(ctx);
        self.player.render(ctx);
        self.enemy.render(ctx);
        self.cpu_player.render(ctx);
        self.random_player.render(ctx);
        self.gui_render(ctx);

        // todo deprecated
        if self.is_ended {
            ctx.print_color_centered(
                20,
                PLAYER_COLOR,
                BACKGROUND_COLOR,
                "Game Over! Punteggi finali:"
            );
            ctx.print_color_centered(
                22,
                PLAYER_COLOR,
                BACKGROUND_COLOR,
                &format!("Tu (verde): {}", self.final_score)
            );
            ctx.print_color_centered(
                23,
                ENEMY_COLOR,
                BACKGROUND_COLOR,
                &format!("CPU (viola): {}", self.enemy_score)
            );
            ctx.print_color_centered(
                24,
                CPU_PLAYER_COLOR,
                BACKGROUND_COLOR,
                &format!("CPU (giallo): {}", self.cpu_score)
            );
        }
    }

    fn gui_render(& mut self, ctx: &mut BTerm) {
        ctx.print_color_centered(0, PLAYER_COLOR, BACKGROUND_COLOR, &format!("Your score is: {}", self.player.get_length()));
        ctx.print_color_centered(1, ENEMY_COLOR, BACKGROUND_COLOR, &format!("Purple CPU score is: {}", self.enemy.get_length()));
        ctx.print_color_centered(2, CPU_PLAYER_COLOR, BACKGROUND_COLOR, &format!("Yellow CPU score is: {}", self.cpu_player.get_length()));
        ctx.print_color_centered(3, RANDOM_PLAYER_COLOR, BACKGROUND_COLOR, &format!("Randy CPU score is: {}", self.random_player.get_length()));

    }

    pub fn play(&mut self,  ctx: &mut BTerm) {
        self.player_inputs_handler(ctx);
        if !self.is_paused {
            self.move_player();
            self.move_enemy();
            self.move_cpu_player();
            self.move_random_player();
            self.render(ctx);
        } else {
            self.render_pause(ctx);
        }
    }

    fn end_game(&mut self) {
        self.is_ended = true;
        self.final_score = self.player.get_length();
        self.enemy_score = self.enemy.get_length();
        self.cpu_score = self.cpu_player.get_length();
        self.random_score = self.random_player.get_length();
    }

    fn handle_snake_collision<T: SnakeCollision>(eater: &mut T, eaten: &mut T) {
        eater.add_length(eaten.get_length());
        eaten.set_length(4);
    }

    fn move_snake(& mut self) {
        let mut new_pos = self.player.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = pacman_effect(new_pos);
        }

        if self.player.collide(new_pos) {
            self.end_game();
        } else if self.map.can_enter(new_pos) {
            let is_eating = new_pos.x == self.fruit.position.x && new_pos.y == self.fruit.position.y;
            let is_eating_enemy = self.enemy.collide(new_pos);
            let is_eating_cpu = self.cpu_player.collide(new_pos);
            let is_eating_random = self.random_player.collide(new_pos);
            
            self.player.move_player(new_pos, is_eating);
            
            if is_eating {
                self.player.add_length(1);
                self.fruit = fruit_builder();
            }
            if is_eating_enemy {
                Self::handle_snake_collision(&mut self.player, &mut self.enemy);
                self.respawn_enemy();
            }
            if is_eating_cpu {
                Self::handle_snake_collision(&mut self.player, &mut self.cpu_player);
                self.respawn_cpu_player();
            }
            if is_eating_random {
                Self::handle_snake_collision(&mut self.player, &mut self.random_player);
                self.respawn_random_player(); 
            }
        }
    }

    fn move_player(& mut self) {
        self.move_snake();
    }

    fn move_enemy(& mut self) {
        self.enemy.change_direction(self.ai.get_next_move(
            self.fruit,
            self.enemy.get_next_pos_player(),
            self.player.get_next_pos_player(),
            self.enemy.get_direction()
        ));

        let mut new_pos = self.enemy.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = pacman_effect(new_pos);
        }

        if self.enemy.collide(new_pos) {
            self.respawn_enemy();
        } else if self.random_player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.enemy, &mut self.random_player);
            self.respawn_random_player();
        } else if self.player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.enemy, &mut self.player);
            self.respawn_player();
        } else if self.cpu_player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.enemy, &mut self.cpu_player);
            self.respawn_cpu_player();
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
        self.player = Player::new(PLAYER_COLOR.into(), get_random_position());
    }

    fn respawn_enemy(& mut self) {
        self.enemy = Player::new(ENEMY_COLOR.into(), get_random_position());
    }

    fn move_cpu_player(& mut self) {
        self.cpu_player.change_direction(self.cpu_ai.get_next_move(
            self.fruit,
            self.cpu_player.get_next_pos_player(),
            self.player.get_next_pos_player(),
            self.cpu_player.get_direction()
        ));

        let mut new_pos = self.cpu_player.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = pacman_effect(new_pos);
        }

        if self.random_player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.cpu_player, &mut self.random_player);
            self.respawn_random_player();
        } else if self.cpu_player.collide(new_pos) {
            self.respawn_cpu_player();
        } else if self.player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.cpu_player, &mut self.player);
            self.respawn_player();
        } else if self.enemy.collide(new_pos) {
            Self::handle_snake_collision(&mut self.cpu_player, &mut self.enemy);
            self.respawn_enemy();
        } else if self.map.can_enter(new_pos) {
            let is_eating = new_pos.x == self.fruit.position.x && new_pos.y == self.fruit.position.y;
            self.cpu_player.move_player(new_pos, is_eating);
            if is_eating {
                self.cpu_player.add_length(1);
                self.fruit = fruit_builder();
            }
        }
    }

    fn respawn_cpu_player(& mut self) {
        self.cpu_player = Player::new(CPU_PLAYER_COLOR.into(), get_random_position());
    }

    fn move_random_player(& mut self) {
        self.random_player.change_direction(self.random_ai.get_next_move(
            self.fruit,
            self.random_player.get_next_pos_player(),
            self.player.get_next_pos_player(),
            self.random_player.get_direction()
        ));

        let mut new_pos = self.random_player.get_next_pos_player();

        if !self.map.in_bounds(new_pos) {
            new_pos = pacman_effect(new_pos);
        }

        if self.random_player.collide(new_pos) {
            self.respawn_random_player();
        } else if self.cpu_player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.random_player, &mut self.cpu_player);
            self.respawn_cpu_player();
        } else if self.player.collide(new_pos) {
            Self::handle_snake_collision(&mut self.random_player, &mut self.player);
            self.respawn_player();
        } else if self.enemy.collide(new_pos) {
            Self::handle_snake_collision(&mut self.random_player, &mut self.enemy);
            self.respawn_enemy();
        } else if self.map.can_enter(new_pos) {
            let is_eating = new_pos.x == self.fruit.position.x && new_pos.y == self.fruit.position.y;
            self.random_player.move_player(new_pos, is_eating);
            if is_eating {
                self.random_player.add_length(1);
                self.fruit = fruit_builder();
            }
        }
    }

    fn respawn_random_player(& mut self) {
        self.random_player = Player::new(RANDOM_PLAYER_COLOR.into(), get_random_position());
    }
    
    fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    pub fn player_inputs_handler(& mut self, ctx: & mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.player.change_direction(super::player::Direction::Up),
                VirtualKeyCode::Down => self.player.change_direction(super::player::Direction::Down),
                VirtualKeyCode::Left => self.player.change_direction(super::player::Direction::Left),
                VirtualKeyCode::Right => self.player.change_direction(super::player::Direction::Right),
                VirtualKeyCode::P => self.toggle_pause(),
                VirtualKeyCode::Q => self.end_game(),
                _ => {},
            }
        }
    }
}