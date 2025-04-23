use super::tuesday::{TuesdayTile, TuesdayTile::*};
use bevy::prelude::*;
use rand::Rng;
use tilegen::{Replacement, ReplacementRule, TileGrid};

pub fn mark_player_start_tile(
    grid: &mut TileGrid<TuesdayTile>,
    start_position_count: u8,
    rng: &mut impl Rng,
) {
    let z = grid.depth() - 1;
    grid.apply_layer_replacements(
        z,
        vec![ReplacementRule {
            condition: |src, _| {
                debug!("{},{} - {:?}", src.x, src.y, src.below().tile);
                src.below() == Floor
                    && src.right().below() == Floor
                    && src.down().below() == Floor
                    && src.bottom_right().below() == Floor
            },
            replacements: vec![Replacement::this(TuesdayTile::PlayerStart(1))],
            apply_count: Some(start_position_count as u16),
            ..Default::default()
        }],
        rng,
    );
}
