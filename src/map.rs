use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn point_in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.point_in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.point_in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }

    pub fn render(&self, context: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_index(x, y);
                match self.tiles[index] {
                    TileType::Floor => {
                        context.set(x, y, GRAY, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        context.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_new_map() {
        let map: Map = Map::new();

        assert_eq!(map.tiles.len(), NUM_TILES);
        assert_eq!(map.tiles[0], TileType::Floor)
    }

    #[test]
    fn get_map_index() {
        // Check that, given an x, y coordinate pair, the correct index in the tiles vector is
        // returned
        assert_eq!(map_index(0, 0), 0);
        assert_eq!(map_index(1, 0), 1);
        assert_eq!(map_index(0, 1), 80);
        assert_eq!(map_index(1, 1), 81);
        assert_eq!(map_index(79, 0), 79);
        assert_eq!(map_index(0, 49), 3920);
    }

    #[test]
    fn check_point_in_bounds() {
        let map: Map = Map::new();

        let mut point: Point = Point::new(1, 1);
        assert_eq!(map.point_in_bounds(point), true);

        point = Point::new(79, 49);
        assert_eq!(map.point_in_bounds(point), true);

        point = Point::new(-1, -1);
        assert_eq!(map.point_in_bounds(point), false);

        point = Point::new(100, 100);
        assert_eq!(map.point_in_bounds(point), false);
    }

    #[test]
    fn check_can_enter_tile() {
        let mut map: Map = Map::new();
        map.tiles[1] = TileType::Wall;

        let mut point: Point = Point::new(1,0);
        assert_eq!(map.can_enter_tile(point), false);

        point = Point::new(0, 1);
        assert_eq!(map.can_enter_tile(point), true);

        point = Point::new(100, 100);
        assert_eq!(map.can_enter_tile(point), false);
    }

    #[test]
    fn check_try_index() {
        let map: Map = Map::new();

        let mut point: Point = Point::new(-1, -1);
        assert_eq!(map.try_index(point).is_some(), false);

        point = Point::new(1, 1);
        assert_eq!(map.try_index(point).is_some(), true);
    }
}