use super::special::starter_room::starter_room;
use super::starter::mark_player_start_tile;
use super::tuesday::TuesdayTile;
use super::{NewMap, TileLayer, TileLayerRole};
use crate::map::wall_wrap::wrap_walls;
use crate::seed::RngSeed;
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

        // let mut grid = walking_squares(
        //     self.width,
        //     self.height,
        //     self.density,
        //     self.branch_factor,
        //     self.wander_factor,
        //     &mut rng,
        // );
        let mut grid = starter_room();
        // fix_floor(&mut grid, &mut rng);
        let mut grid = wrap_walls(grid, &mut rng);
        mark_player_start_tile(&mut grid);

        // TODO: it seems like having Grid with layers of different types is a problem
        // TODO: change this to a custom command instead of spawning TileLayer
        world.spawn((
            TileLayer {
                role: TileLayerRole::Base,
                grid: TuesdayTile::layer_to_tile_sprites(&grid, 0),
                tileset_name: TuesdayTile::name(),
                z: 0.0,
            },
            Transform::default(),
        ));
        world.spawn((
            TileLayer {
                role: TileLayerRole::Base,
                grid: TuesdayTile::layer_to_tile_sprites(&grid, 1),
                tileset_name: TuesdayTile::name(),
                z: 5.0,
            },
            Transform::default(),
        ));
        world.send_event(NewMap);
    }
}
