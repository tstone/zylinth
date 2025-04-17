use crate::layout::cosmic_legacy::{CosmicLegacyTile, decorate};
use crate::layout::lighting::spot_lights;
use crate::layout::shadowizer::shadowize;
use crate::layout::tilemap::{RenderedTileLayer, render_tilemap};
use crate::layout::tileset::init_cosmic_tileset;
use crate::layout::wall_wrap::wrap_walls;

use super::fixer::floor_fixer;
use super::tileset::Tileset;
use super::walking_squares::walking_squares;
use bevy::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub struct TileLayoutPlugin;

impl Plugin for TileLayoutPlugin {
    fn build(&self, app: &mut App) {
        // let seed = random_range(0..u64::MAX);
        let seed = 1;
        debug!("Using rng seed: {seed}");
        app.insert_resource(RngSeed(seed));

        app.add_event::<RenderedTileLayer>();

        app.init_asset::<Tileset>();

        app.add_systems(PreStartup, init_cosmic_tileset);

        app.add_observer(render_tilemap);
        app.add_observer(spot_lights);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TileSprite {
    pub index: usize,
    pub collider: bool,
}

#[derive(Component)]
pub struct TileLayer {
    pub grid: Vec<Vec<Option<TileSprite>>>,
    pub tileset_name: &'static str,
    pub z: f32,
}

pub struct SpawnBuilding {
    width: usize,
    height: usize,
}

impl SpawnBuilding {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl Command for SpawnBuilding {
    fn apply(self, world: &mut World) {
        let seed = world.get_resource::<RngSeed>().unwrap();
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);

        let floor = walking_squares(24, 100, 0.15, 0.15, &mut rng);
        let floor_fixed = floor_fixer(floor, &mut rng);
        let walled = wrap_walls(floor_fixed, &mut rng);
        let bg_decorations = decorate(&walled, &mut rng);
        let shadow_walls = shadowize(walled, &mut rng);
        let base_layer = CosmicLegacyTile::from_utility_to_tile_sprite(shadow_walls, &mut rng);
        let bg_layer = CosmicLegacyTile::to_tile_sprite(bg_decorations);

        // TODO: make another system that listens for game start, grabs all the tile layers, and figures out where to spawn the player (or something like that)

        // spawn layers
        world.spawn((
            TileLayer {
                grid: base_layer,
                tileset_name: CosmicLegacyTile::name(),
                z: 0.0,
            },
            Transform::default(),
        ));
        world.spawn((
            TileLayer {
                grid: bg_layer,
                tileset_name: CosmicLegacyTile::name(),
                z: 1.0,
            },
            Transform::default(),
        ));
    }
}

#[derive(Resource)]
pub struct RngSeed(u64);
