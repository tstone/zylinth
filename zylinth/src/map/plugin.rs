use crate::map::lighting::spot_lights;
use crate::map::tilemap::{RenderedTileLayer, render_tilemap};
use crate::map::tileset::*;

use super::tileset::Tileset;
use bevy::prelude::*;

pub struct TileLayoutPlugin;

impl Plugin for TileLayoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RenderedTileLayer>();
        app.add_event::<NewMap>();

        app.init_asset::<Tileset>();

        app.add_systems(PreStartup, init_tuesday_tileset);
        app.add_systems(
            PostUpdate,
            spot_lights.after(TransformSystem::TransformPropagate),
        );

        app.add_observer(render_tilemap);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileSprite {
    pub index: usize,
    pub collider: bool,
    pub role: Option<TileRole>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileRole {
    Switch(u8, bool),
    Door(u8),
    PlayerStart(u8),
}

pub trait IsImpassable {
    fn is_impassable(&self) -> bool;
}

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
