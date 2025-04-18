use crate::layout::cosmic_legacy::{CosmicLegacyTile, decorate};
use crate::layout::shadowizer::shadowize;
use crate::layout::wall_wrap::wrap_walls;

use super::fixer::floor_fixer;
use super::starter::mark_player_start_tile;
use super::walking_squares::walking_squares;
use super::{NewMap, RngSeed, TileLayer, TileLayerRole};
use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

/// Custom command to spawn a new map based around a "building"
pub struct SpawnBuildingMap {
    width: usize,
    height: usize,
}

impl SpawnBuildingMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Command for SpawnBuildingMap {
    fn apply(self, world: &mut World) {
        let seed = world.get_resource::<RngSeed>().unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);

        let floor = walking_squares(24, 100, 0.15, 0.15, &mut rng);
        let floor_fixed = floor_fixer(floor, &mut rng);
        let walled = wrap_walls(floor_fixed, &mut rng);
        let bg_decorations = decorate(&walled, &mut rng);
        let mut shadow_walls = shadowize(walled, &mut rng);
        mark_player_start_tile(&mut shadow_walls);

        let base_layer = CosmicLegacyTile::from_utility_to_tile_sprite(shadow_walls, &mut rng);
        let bg_layer = CosmicLegacyTile::to_tile_sprite(bg_decorations);

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
