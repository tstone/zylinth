use super::tilemap::{RenderedTileLayer, Tile};
use bevy::{color::palettes::tailwind::GREEN_500, prelude::*};
use bevy_lit::prelude::PointLight2d;

pub fn spot_lights(
    _trigger: Trigger<RenderedTileLayer>,
    tiles: Query<(&Tile, Entity)>,
    mut commands: Commands,
) {
    for (tile, entity) in tiles.iter() {
        // TODO: what to illuminate?

        // if tile.tileset_name == CosmicLegacyTile::name()
        //     && tile.tile_index == CosmicLegacyTile::AlienTop as usize
        // {
        //     commands.entity(entity).insert((PointLight2d {
        //         color: Color::from(GREEN_500),
        //         radius: 40.0,
        //         intensity: 4.0,
        //         falloff: 8.0,
        //         ..default()
        //     },));
        // }
    }
}
