use bracket_lib::terminal::{to_cp437, BTerm, Point};

use super::snake_game_state::BACKGROUND_COLOR;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub struct Player {
    facing: Direction,
    head_position: Point,
    tail: Vec<Point>,
    length: usize,
    color: (u8, u8, u8),
}

impl Player {
    pub fn new(color: (u8, u8, u8)) -> Self {
        Player {
            facing: Direction::Up,
            head_position: Point { x: 40, y: 20 },
            tail: [].to_vec(),
            length: 4,
            color: color,
        }
    }

    pub fn change_direction(& mut self, dir: Direction) {
        self.facing = dir;
    }

    pub fn get_next_pos_player(&self) -> Point {
        match self.facing {
            Direction::Up => { return Point::new(self.head_position.x, self.head_position.y - 1)},
            Direction::Down => { return Point::new(self.head_position.x, self.head_position.y + 1)}
            Direction::Left => { return Point::new(self.head_position.x - 1, self.head_position.y)},
            Direction::Right => { return Point::new(self.head_position.x + 1, self.head_position.y)},
        }
    }

    pub fn move_player(&mut self, new_pos: Point, is_eating: bool) {
        self.tail.insert(0,self.head_position.clone());
        self.head_position = new_pos.clone();
        if is_eating {
            self.length = self.length + 1;
        } else if self.tail.len() >= self.length {
            self.tail.remove(self.tail.len() - 1);
        }
    }

    pub fn collide(&self, pos: Point) -> bool {
        self.tail.iter().find(|elm| elm.x == pos.x && elm.y == pos.y).is_some() || (self.head_position.x == pos.x && self.head_position.y == pos.y)
    }

    fn render_head(& mut self, ctx: & mut BTerm) {
        let glyph;
        match self.facing {
            Direction::Up => { glyph = '^'; }
            Direction::Down => { glyph = ','; }
            Direction::Left => { glyph = '<'; }
            Direction::Right => { glyph = '>'; }
        }
        
        ctx.set(self.head_position.x, self.head_position.y, self.color, BACKGROUND_COLOR, to_cp437(glyph));
    }

    fn render_tail(& mut self, ctx: & mut BTerm) {
        for x in 0..self.tail.len() {
            let tail_piece = self.tail[x];
            ctx.set(tail_piece.x, tail_piece.y, self.color, BACKGROUND_COLOR, to_cp437('#'));
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_direction(&self) -> Direction {
        self.facing
    }

    pub fn render(& mut self, ctx: & mut BTerm) {
        self.render_head(ctx);
        self.render_tail(ctx);

    } }

