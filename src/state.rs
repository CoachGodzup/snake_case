use bracket_lib::terminal::{BTerm, VirtualKeyCode};
use bracket_lib::color::BLACK;


use crate::game::snake_game_state::{SnakeGameState, ENEMY_COLOR, PLAYER_COLOR, CPU_PLAYER_COLOR, RANDOM_PLAYER_COLOR};

pub const MENU_BACKGROUND_COLOR: (u8, u8, u8) = BLACK;

pub enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    pub mode: GameMode,
    game: Option<SnakeGameState>,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            game: None,
        }
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(4, "Welcome to snake_case");
        ctx.print_centered(6, "(P) Play Game");
        ctx.print_centered(7, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameMode::Playing;
        // init phase
        if self.game.is_none() {
            self.game = Some(SnakeGameState::new());
        }
        let game_state = self.game.as_ref().unwrap();
        if game_state.is_ended {
            self.mode = GameMode::End;
        } else {
            self.game.as_mut().unwrap().play(ctx);
        }
    }

    pub fn restart(&mut self) {
        self.game = Some(SnakeGameState::new());
        self.mode = GameMode::Playing;
    }

    pub fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(4, "Game over");
        ctx.print_color_centered(6, PLAYER_COLOR, MENU_BACKGROUND_COLOR, format!("Your score is: {}", self.game.as_ref().unwrap().final_score));
        ctx.print_color_centered(8, ENEMY_COLOR, MENU_BACKGROUND_COLOR, format!("Purple score is: {}", self.game.as_ref().unwrap().enemy_score));
        ctx.print_color_centered(10, CPU_PLAYER_COLOR, MENU_BACKGROUND_COLOR, format!("Yellow score is: {}", self.game.as_ref().unwrap().cpu_score));
        ctx.print_color_centered(12, RANDOM_PLAYER_COLOR, MENU_BACKGROUND_COLOR, format!("Randy score is: {}", self.game.as_ref().unwrap().random_score));

        ctx.print_centered(14, "(P) Play Again");
        ctx.print_centered(15, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
}