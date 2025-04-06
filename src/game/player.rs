use bracket_lib::terminal::{to_cp437, BTerm, Point};
use bracket_lib::color::RGB;

use super::snake_game_state::BACKGROUND_COLOR;

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait SnakeCollision {
    fn get_position(&self) -> Point;
    fn get_length(&self) -> usize;
    fn set_length(&mut self, length: usize);
    fn add_length(&mut self, length: usize);
}

pub struct Player {
    pub body: Vec<Point>,
    pub direction: Direction,
    pub color: RGB,
}

impl Player {
    pub fn new(color: RGB, position: Point) -> Self {
        let mut body = Vec::new();
        body.push(position);
        body.push(Point::new(position.x - 1, position.y));
        body.push(Point::new(position.x - 2, position.y));
        body.push(Point::new(position.x - 3, position.y));

        Player {
            body,
            direction: Direction::Right,
            color,
        }
    }

    pub fn get_next_pos_player(&self) -> Point {
        let head = self.body[0];
        match self.direction {
            Direction::Up => Point::new(head.x, head.y - 1),
            Direction::Down => Point::new(head.x, head.y + 1),
            Direction::Left => Point::new(head.x - 1, head.y),
            Direction::Right => Point::new(head.x + 1, head.y),
        }
    }

    pub fn move_player(&mut self, new_pos: Point, is_eating: bool) {
        self.body.insert(0, new_pos);
        if !is_eating {
            self.body.pop();
        }
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if !self.is_opposite_direction(new_direction) {
            self.direction = new_direction;
        }
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    fn is_opposite_direction(&self, new_direction: Direction) -> bool {
        (self.direction == Direction::Up && new_direction == Direction::Down)
            || (self.direction == Direction::Down && new_direction == Direction::Up)
            || (self.direction == Direction::Left && new_direction == Direction::Right)
            || (self.direction == Direction::Right && new_direction == Direction::Left)
    }

    pub fn collide(&self, position: Point) -> bool {
        self.body.contains(&position)
    }

    fn render_head(& mut self, ctx: & mut BTerm) {
        let glyph;
        match self.direction {
            Direction::Up => { glyph = '^'; }
            Direction::Down => { glyph = ','; }
            Direction::Left => { glyph = '<'; }
            Direction::Right => { glyph = '>'; }
        }
        
        ctx.set(self.body[0].x, self.body[0].y, self.color, BACKGROUND_COLOR, to_cp437(glyph));
    }

    fn render_tail(& mut self, ctx: & mut BTerm) {
        for x in 1..self.body.len() {
            let tail_piece = self.body[x];
            ctx.set(tail_piece.x, tail_piece.y, self.color, BACKGROUND_COLOR, to_cp437('#'));
        }
    }

    pub fn render(& mut self, ctx: & mut BTerm) {
        self.render_head(ctx);
        self.render_tail(ctx);
    }
}

impl SnakeCollision for Player {
    fn get_position(&self) -> Point {
        self.body[0]
    }

    fn get_length(&self) -> usize {
        self.body.len()
    }

    fn set_length(&mut self, length: usize) {
        while self.body.len() > length {
            self.body.pop();
        }
    }

    fn add_length(&mut self, length: usize) {
        let current_length = self.body.len();
        for _ in 0..length {
            self.body.push(self.body[current_length - 1]);
        }
    }
}

