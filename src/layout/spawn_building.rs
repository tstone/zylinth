use crate::layout::cosmic_legacy::CosmicLegacyTile;
use crate::layout::shadowizer::shadowize;
use crate::layout::wall_wrap::wrap_walls;
use crate::seed::RngSeed;

use super::decoration::decorate_layer;
use super::fixer::fix_floor;
use super::modifications::TileGrid;
use super::starter::mark_player_start_tile;
use super::walking_squares::walking_squares;
use super::{NewMap, TileLayer, TileLayerRole};
use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Custom command to spawn a new map based around a "building"
pub struct SpawnBuildingMap {
    pub width: usize,
    pub height: usize,
    /// what percent of the total map space should be filled
    pub density: f32,
    /// how much should this randomly go in a different direction
    pub wander_factor: f32,
    /// how much often this should split off in a new direction
    pub branch_factor: f32,
}

impl Command for SpawnBuildingMap {
    fn apply(self, world: &mut World) {
        let seed = world.get_resource::<RngSeed>().unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);

        let mut grid = walking_squares(
            self.width,
            self.height,
            self.density,
            self.branch_factor,
            self.wander_factor,
            &mut rng,
        );
        fix_floor(&mut grid, &mut rng);
        let mut grid = wrap_walls(grid, &mut rng);

        TileGrid::add_layer(&mut grid);
        decorate_layer(&mut grid, 1, &mut rng);
        shadowize(&mut grid, &mut rng);
        mark_player_start_tile(&mut grid);

        let base_layer = CosmicLegacyTile::to_tile_sprite(&grid, 0, &mut rng);
        let bg_layer = CosmicLegacyTile::to_tile_sprite(&grid, 1, &mut rng);

        // TODO: change this to a custom command instead of spawning TileLayer
        world.spawn((
            TileLayer {
                role: TileLayerRole::Base,
                grid: base_layer,
                tileset_name: CosmicLegacyTile::name(),
                z: 0.0,
            },
            Transform::default(),
        ));
        world.spawn((
            TileLayer {
                role: TileLayerRole::BackgroundDecorations,
                grid: bg_layer,
                tileset_name: CosmicLegacyTile::name(),
                z: 1.0,
            },
            Transform::default(),
        ));

        world.send_event(NewMap);
    }
}
