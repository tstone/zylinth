use crate::layout::lighting::spot_lights;
use crate::layout::tilemap::{RenderedTileLayer, render_tilemap};
use crate::layout::tileset::init_cosmic_tileset;

use super::functional_tiles::UtilityTile;
use super::tileset::Tileset;
use bevy::prelude::*;

pub struct TileLayoutPlugin;

impl Plugin for TileLayoutPlugin {
    fn build(&self, app: &mut App) {
        // let seed = random_range(0..u64::MAX);
        let seed = 1;
        debug!("Using rng seed: {seed}");
        app.insert_resource(RngSeed(seed));

        app.add_event::<RenderedTileLayer>();
        app.add_event::<NewMap>();

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
    pub role: Option<UtilityTile>,
}

#[derive(Component)]
pub struct PlayerStartTile;

#[derive(Debug, PartialEq, Eq)]
pub enum TileLayerRole {
    Base,
    BackgroundDecorations,
    ForegroundDecorations,
}

#[derive(Component)]
pub struct TileLayer {
    pub role: TileLayerRole,
    pub grid: Vec<Vec<Option<TileSprite>>>,
    pub tileset_name: &'static str,
    pub z: f32,
}

// TODO: extract this to a separate plugin
#[derive(Resource)]
pub struct RngSeed(pub u64);

#[derive(Event)]
pub struct NewMap;
