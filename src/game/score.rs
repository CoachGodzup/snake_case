use bracket_lib::color::{GREEN_YELLOW, NAVY_BLUE, REBECCA_PURPLE, YELLOW};
use bracket_lib::terminal::BTerm;

pub const BACKGROUND_COLOR: (u8, u8, u8) = NAVY_BLUE;
pub const PLAYER_COLOR: (u8, u8, u8) = GREEN_YELLOW;
pub const ENEMY_COLOR: (u8, u8, u8) = REBECCA_PURPLE;
pub const CPU_PLAYER_COLOR: (u8, u8, u8) = YELLOW;

#[derive(Clone, Copy)]
pub struct Score {
    pub final_score: usize,
    pub enemy_score: usize,
    pub cpu_score: usize,
    max_player_length: usize,
    max_enemy_length: usize,
    max_cpu_length: usize,
}

impl Score {
    pub fn new() -> Self {
        Score {
            final_score: 0,
            enemy_score: 0,
            cpu_score: 0,
            max_player_length: 4,
            max_enemy_length: 4,
            max_cpu_length: 4,
        }
    }

    pub fn update_max_lengths(&mut self, player_length: usize, enemy_length: usize, cpu_length: usize) {
        self.max_player_length = self.max_player_length.max(player_length);
        self.max_enemy_length = self.max_enemy_length.max(enemy_length);
        self.max_cpu_length = self.max_cpu_length.max(cpu_length);
    }

    pub fn update_final_scores(&mut self, player_length: usize, enemy_length: usize, cpu_length: usize) {
        self.final_score = player_length;
        self.enemy_score = enemy_length;
        self.cpu_score = cpu_length;
    }

    pub fn get_current_scores(&self) -> (usize, usize, usize) {
        (self.final_score, self.enemy_score, self.cpu_score)
    }

    pub fn render_game_scores(&self, ctx: &mut BTerm) {
        let (player_score, enemy_score, cpu_score) = self.get_current_scores();
        ctx.print_color_centered(0, PLAYER_COLOR, BACKGROUND_COLOR, &format!("Tu (verde): {}", player_score));
        ctx.print_color_centered(1, ENEMY_COLOR, BACKGROUND_COLOR, &format!("CPU (viola): {}", enemy_score));
        ctx.print_color_centered(2, CPU_PLAYER_COLOR, BACKGROUND_COLOR, &format!("CPU (giallo): {}", cpu_score));
    }

    pub fn render_final_scores(&self, ctx: &mut BTerm) {
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
            &format!("Tu (verde) - Punteggio: {}, Lunghezza massima: {}", self.final_score, self.max_player_length)
        );
        ctx.print_color_centered(
            23,
            ENEMY_COLOR,
            BACKGROUND_COLOR,
            &format!("CPU (viola) - Punteggio: {}, Lunghezza massima: {}", self.enemy_score, self.max_enemy_length)
        );
        ctx.print_color_centered(
            24,
            CPU_PLAYER_COLOR,
            BACKGROUND_COLOR,
            &format!("CPU (giallo) - Punteggio: {}, Lunghezza massima: {}", self.cpu_score, self.max_cpu_length)
        );
    }
} 