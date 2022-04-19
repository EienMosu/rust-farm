use crate::prelude::*;

const NUMBER_OF_TILES: usize = { DISPLAY_WIDTH * DISPLAY_HEIGHT } as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum ObjectType {
    Floor,
    Wall,
    Apple,
    RottenApple,
}
pub struct Map {
    pub objects: Vec<ObjectType>,
    pub tiles: Vec<Tile>,
    pub walls: Vec<Wall>,
    pub apples: Vec<Apple>,
    pub roten_apples: Vec<RottenApple>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            objects: vec![ObjectType::Floor; NUMBER_OF_TILES],
            tiles: Vec::new(),
            apples: Vec::new(),
            roten_apples: Vec::new(),
            walls: Vec::new(),
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        //info!("TOTAL TILE {}", self.objects.len());

        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                // let index = map_to_index(x, y);
                // match self.objects[index] {
                //     ObjectType::Floor => ctx.set(x, y, BLACK, RED, to_cp437('#')),
                //     ObjectType::Wall => ctx.set(x, y, WHITE, BLACK, to_cp437('!')),
                //     ObjectType::Apple => ctx.set(x, y, WHITE, RED, to_cp437('.')),
                //     ObjectType::RottenApple => ctx.set(x, y, WHITE, YELLOW, to_cp437(',')),
                // }
                let floor = Floor::new(Point { x, y });
                floor.render(ctx);
            }
        }

        for apple in &self.apples {
            apple.render(ctx);
        }

        for rotten_apple in &self.roten_apples {
            rotten_apple.render(ctx);
        }

        for wall in &self.walls {
            wall.render(ctx);
        }
    }
    pub fn is_in_bounds(&self, point: Point) -> bool {
        (point.x >= 0 && point.x < DISPLAY_WIDTH) && (point.y >= 0 && point.y < DISPLAY_HEIGHT)
    }
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.is_in_bounds(point)
            && self.objects[map_to_index(point.x, point.y)] == ObjectType::Floor
    }
    pub fn try_map_to_index(&self, point: Point) -> Option<usize> {
        if !self.is_in_bounds(point) {
            None
        } else {
            Some(map_to_index(point.x, point.y))
        }
    }
}

pub fn map_to_index(x: i32, y: i32) -> usize {
    ((y * DISPLAY_WIDTH) + x) as usize
}
