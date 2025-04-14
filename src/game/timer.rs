use bracket_lib::color::{NAVY_BLUE, YELLOW};
use bracket_lib::terminal::BTerm;
use std::time::Instant;

pub struct GameTimer {
    start_time: Instant,
    duration: u64,
}

impl GameTimer {
    pub fn new(duration: u64) -> Self {
        GameTimer {
            start_time: Instant::now(),
            duration,
        }
    }

    pub fn get_remaining_time(&self) -> u64 {
        let elapsed = self.start_time.elapsed().as_secs();
        if elapsed >= self.duration {
            0
        } else {
            self.duration - elapsed
        }
    }

    pub fn is_time_up(&self) -> bool {
        self.start_time.elapsed().as_secs() >= self.duration
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let remaining = self.get_remaining_time();
        let timer_text = format!("Time: {:02}s", remaining);
        let (width, height) = ctx.get_char_size();
        ctx.print_color(
            (width as i32) - (timer_text.len() as i32),
            (height as i32) - 1,
            YELLOW,
            NAVY_BLUE,
            &timer_text,
        );
    }
} 