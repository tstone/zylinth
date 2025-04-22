use crate::layout::lighting::spot_lights;
use crate::layout::tilemap::{RenderedTileLayer, render_tilemap};
use crate::layout::tileset::*;

use super::functional_tiles::UtilityTile;
use super::tileset::Tileset;
use bevy::prelude::*;

pub struct TileLayoutPlugin;

impl Plugin for TileLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RenderedTileLayer>();
        app.add_event::<NewMap>();

        app.init_asset::<Tileset>();

        app.add_systems(PreStartup, init_tuesday_tileset);

        app.add_observer(render_tilemap);
        app.add_observer(spot_lights);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileSprite {
    pub index: usize,
    pub collider: bool,
    pub role: Option<UtilityTile>,
}

pub trait IsImpassable {
    fn is_impassable(&self) -> bool;
}

#[derive(Component)]
pub struct PlayerStartTile;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq)]
pub enum TileLayerRole {
    Base,
    BackgroundDecorations,
    ForegroundDecorations,
}

#[allow(unused)]
#[derive(Component)]
pub struct TileLayer {
    pub role: TileLayerRole,
    pub grid: Vec<Vec<Option<TileSprite>>>,
    pub tileset_name: &'static str,
    pub z: f32,
}

#[derive(Event)]
pub struct NewMap;
