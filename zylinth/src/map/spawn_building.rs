use super::decoration::decorate_empty;
use super::special::starter_room::starter_room;
use super::starter::mark_player_start_tile;
use super::tuesday::TuesdayTile;
use super::{NewMap, TileLayer, TileLayerRole};
use crate::defs::GameLayer;
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

        let mut puzzle = starter_room();
        let mut grid = wrap_walls(puzzle.grid, &mut rng);
        decorate_empty(&mut grid, &mut rng);
        mark_player_start_tile(&mut grid, 1, &mut rng);

        // TODO: change this to a custom command instead of spawning TileLayer
        world.spawn((
            TileLayer {
                grid: TuesdayTile::layer_to_tile_sprites(&grid, 0),
                tileset_name: TuesdayTile::name(),
                z: 0.0,
                ..Default::default()
            },
            Transform::default(),
        ));
        world.spawn((
            TileLayer {
                grid: TuesdayTile::layer_to_tile_sprites(&grid, 1),
                tileset_name: TuesdayTile::name(),
                z: 1.0,
                ..Default::default()
            },
            Transform::default(),
        ));
        world.spawn((
            TileLayer {
                grid: TuesdayTile::layer_to_tile_sprites(&grid, 2),
                tileset_name: TuesdayTile::name(),
                z: 5.0,
                layer: GameLayer::Interactables,
            },
            Transform::default(),
        ));

        for door_control in puzzle.door_controls {
            world.spawn(door_control);
        }

        world.send_event(NewMap);
    }
}
