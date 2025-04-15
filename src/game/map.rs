use crate::prelude::*;

use super::snake_game_state::BACKGROUND_COLOR;
const NUM_TILES: usize = 4096_usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    // Wall,
    Floor,
}
// why?? does not make more sense a sparse array?
pub struct Map {
    pub tiles: Vec<TileType>,
}

// rows first
pub fn map_index(x: usize, y: usize) -> usize {
    (y * SCREEN_WIDTH) + x
}

pub fn get_random_position() -> Point {
    Point::new(RandomNumberGenerator::new().range(0, SCREEN_WIDTH), RandomNumberGenerator::new().range(0, SCREEN_HEIGHT))
}

pub fn pacman_effect(pos: Point) -> Point {
    let mut new_pos: Point = pos;
    
    if pos.x < 0 {
        new_pos.x = SCREEN_WIDTH as i32;
    } else if pos.x  > SCREEN_WIDTH as i32 {
        new_pos.x = 0;
    }

    if pos.y < 0 {
        new_pos.y = SCREEN_HEIGHT as i32;
    } else if pos.y > SCREEN_HEIGHT as i32{
        new_pos.y = 0;
    }

    new_pos
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    fn render_tile (&self, ctx: &mut BTerm, tile: TileType, x: usize, y: usize) {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, BACKGROUND_COLOR, BACKGROUND_COLOR, to_cp437('.'))
            }
            /*TileType::Wall => {
                ctx.set(x, y, BLACK, BACKGROUND_COLOR, to_cp437('#'))
            }*/
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        // TODO: iterate through tiles instead?
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_index(x, y);
                self.render_tile(ctx, self.tiles[idx], x, y);
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x <= SCREEN_WIDTH as i32 &&
        point.y >= 0 && point.y <= SCREEN_HEIGHT as i32
    }

    pub fn can_enter(&self, dest: Point) -> bool {
        self.in_bounds(dest) &&
        self.tiles[map_index(dest.x as usize, dest.y as usize)]== TileType::Floor
    }
}

