use std::usize;

use super::tilemap::Tile;
use super::tuesday::{TuesdayTile, TuesdayTile::*};
use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

pub fn spot_lights(
    tiles: Query<(&Tile, &Sprite, Entity), Changed<Sprite>>,
    mut commands: Commands,
) {
    for (tile, sprite, entity) in tiles.iter() {
        if tile.tileset_name == TuesdayTile::name() {
            let index = sprite
                .texture_atlas
                .as_ref()
                .map(|atlas| atlas.index)
                .unwrap_or(usize::MAX);
            if index == PanelDisabled as usize {
                commands.entity(entity).insert((PointLight2d {
                    color: Color::srgb(1., 0., 0.),
                    radius: 30.0,
                    intensity: 4.0,
                    falloff: 8.0,
                    ..default()
                },));
            }
        }
    }
}
