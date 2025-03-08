use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

use crate::costmic_legacy_tiles::CosmicLegacyTiles;

#[derive(Debug, Default, Clone)]
pub struct Room {
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Vec<Option<CosmicLegacyTiles>>>,
}

impl Room {
    /// A room that's enclosed by walls on all sides
    pub fn generate_walled(min: u8, max: u8) -> Room {
        let top_margin = 3; // top cap + 2x wall
        let bot_margin = 1;
        let left_margin = 1;
        let right_margin = 1;
        let width = rand::random_range(min..max);
        let height = rand::random_range(min..max);
        let total_width = width + (left_margin + right_margin);
        let total_height = height + (top_margin + bot_margin);
        let mut tiles: Vec<Vec<Option<CosmicLegacyTiles>>> =
            vec![vec![None; total_height as usize]; total_width as usize];

        let max_x = total_width - 1;
        let max_y = total_height - 1;
        let mut rng = rand::rng();

        for x in 0..total_width {
            for y in 0..total_height {
                tiles[x as usize][y as usize] = match (x, y) {
                    (0, 0) => Some(CosmicLegacyTiles::TopLeftOuterCorner),
                    (x, 0) if x == max_x => Some(CosmicLegacyTiles::TopRightOuterCorner),
                    (0, y) if y == max_y => Some(CosmicLegacyTiles::BottomLeftOuterCorner),
                    (x, y) if x == max_x && y == max_y => {
                        Some(CosmicLegacyTiles::BottomRightOuterCorner)
                    }
                    (x, _) if x == max_x => Some(CosmicLegacyTiles::TopCapRight),
                    (_, y) if y == max_y => Some(CosmicLegacyTiles::TopCapBottomSimple),
                    (0, _) => Some(CosmicLegacyTiles::TopCapLeft),
                    (_, 0) => Some(CosmicLegacyTiles::TopCapTopSimple),
                    (_, 1) => Self::rnd_wall(&mut rng),
                    (_, 2) => Some(CosmicLegacyTiles::Wall),
                    (1, 3) => Some(CosmicLegacyTiles::FloorShadowTopCorner),
                    (_, 3) => Some(CosmicLegacyTiles::FloorShadowTop),
                    (1, _) => Some(CosmicLegacyTiles::FloorShadowLeft),
                    _ => Self::rnd_floor(&mut rng),
                }
            }
        }

        return Room {
            width: total_width,
            height: total_height,
            tiles,
        };
    }

    /// Given a walled room, drop the bottom left chunk
    pub fn remove_bottom_left_chunk_walled(room: &mut Room) {
        let cut_x = rand::random_range(0..room.width - 5);
        let cut_x = std::cmp::max(cut_x, 4);

        let cut_y = rand::random_range(0..room.height - 7);
        let cut_y = std::cmp::max(cut_y, 5);

        for x in 0..room.width {
            for y in 0..room.height {
                room.tiles[x as usize][y as usize] = match (x, y) {
                    (x, y) if y == cut_y && x == cut_x - 1 => {
                        Some(CosmicLegacyTiles::TopRightInnerCorner)
                    }
                    (x, y) if y == cut_y && x == 0 => {
                        Some(CosmicLegacyTiles::BottomLeftOuterCorner)
                    }
                    (x, y) if x == cut_x - 1 && y == room.height - 1 => {
                        Some(CosmicLegacyTiles::BottomLeftOuterCorner)
                    }
                    (x, y) if x == cut_x - 1 && y > cut_y => Some(CosmicLegacyTiles::TopCapLeft),
                    (x, y) if y == cut_y && x > 0 && x < cut_x => {
                        Some(CosmicLegacyTiles::TopCapBottomSimple)
                    }
                    (x, y) if x < cut_x && y > cut_y => None,
                    _ => room.tiles[x as usize][y as usize],
                }
            }
        }
    }

    fn rnd_floor(rng: &mut ThreadRng) -> Option<CosmicLegacyTiles> {
        if rand::random_bool(0.975) {
            return Some(CosmicLegacyTiles::Floor);
        } else {
            return CosmicLegacyTiles::floor_tiles().iter().choose(rng).copied();
        }
    }

    fn rnd_wall(rng: &mut ThreadRng) -> Option<CosmicLegacyTiles> {
        if rand::random_bool(0.9) {
            return Some(CosmicLegacyTiles::Wall);
        } else {
            return CosmicLegacyTiles::wall_tiles().iter().choose(rng).copied();
        }
    }
}
