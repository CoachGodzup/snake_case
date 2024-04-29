use bracket_lib::terminal::{BTerm, VirtualKeyCode};

use crate::game::snake_game_state::SnakeGameState;

pub enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    pub mode: GameMode,
    score: usize,
    game: Option<SnakeGameState>,
}

impl State {
    pub fn new() -> Self {
        State {
            mode: GameMode::Menu,
            score: 0,
            game: None,
        }
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to snake_case");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

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
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("Your score is: {}", self.game.as_ref().unwrap().final_score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
}