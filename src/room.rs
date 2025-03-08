use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

use crate::costmic_legacy_tiles::CosmicLegacyTiles;

#[derive(Debug, Default, Clone)]
pub struct Room {
    pub width: u8,
    pub height: u8,
    pub bottom_layer: Vec<Vec<Option<CosmicLegacyTiles>>>,
    pub top_layer: Vec<Vec<Option<CosmicLegacyTiles>>>,
}

impl Room {
    /// A room that's enclosed by walls on all sides
    pub fn generate_walled() -> Room {
        let top_margin = 2; // top cap + wall
        let bot_margin = 1;
        let left_margin = 1;
        let right_margin = 1;
        let width = rand::random_range(8..20);
        let height = rand::random_range(8..20);
        let total_width = width + (left_margin + right_margin);
        let total_height = height + (top_margin + bot_margin);
        let mut bottom_layer: Vec<Vec<Option<CosmicLegacyTiles>>> =
            vec![vec![None; total_height as usize]; total_width as usize];
        let mut top_layer: Vec<Vec<Option<CosmicLegacyTiles>>> =
            vec![vec![None; total_height as usize]; total_width as usize];

        let max_x = total_width - 1;
        let max_y = total_height - 1;
        let mut rng = rand::rng();

        for x in 0..total_width {
            for y in 0..total_height {
                bottom_layer[x as usize][y as usize] = match (x, y) {
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
                    (1, 2) => Some(CosmicLegacyTiles::FloorShadowTopCorner),
                    (_, 2) => Some(CosmicLegacyTiles::FloorShadowTop),
                    (1, _) => Some(CosmicLegacyTiles::FloorShadowLeft),
                    _ => Self::rnd_floor(&mut rng),
                }
            }
        }

        return Room {
            width: total_width,
            height: total_height,
            bottom_layer,
            top_layer,
        };
    }

    fn rnd_floor(rng: &mut ThreadRng) -> Option<CosmicLegacyTiles> {
        if rand::random_bool(0.9) {
            return Some(CosmicLegacyTiles::Floor);
        } else {
            return CosmicLegacyTiles::floor_tiles().iter().choose(rng).copied();
        }
    }

    fn rnd_wall(rng: &mut ThreadRng) -> Option<CosmicLegacyTiles> {
        if rand::random_bool(0.85) {
            return Some(CosmicLegacyTiles::Wall);
        } else {
            return CosmicLegacyTiles::wall_tiles().iter().choose(rng).copied();
        }
    }
}
