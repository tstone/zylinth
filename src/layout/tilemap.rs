use std::fmt::Debug;

use bevy::prelude::*;

use super::functional_tiles::UtilityTile;
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Debug)]
pub struct Tileset<T> {
    pub render: fn(T, &mut ChaCha8Rng) -> usize,
    pub tile_width: u8,
    pub tile_height: u8,
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Clone, Debug)]
pub struct TilemapConfig<T> {
    pub width: u32,
    pub height: u32,
    pub tileset: Tileset<T>,
}

#[derive(Component)]
pub struct Tile {
    x: u32,
    y: u32,
}

#[derive(Component)]
pub struct TileType {
    utility_type: UtilityTile,
}

pub fn render_utility_tilemap(
    tiles: Vec<Vec<Option<UtilityTile>>>,
    config: TilemapConfig<UtilityTile>,
    transform: Transform,
    mut commands: Commands,
    rng: &mut ChaCha8Rng,
) {
    let tilemap_entity = commands
        .spawn((config.clone(), transform, Visibility::Visible))
        .id();
    let mut tile_entities: Vec<Entity> = Vec::new();

    for x in 0..config.width {
        for y in 0..config.height {
            if let Some(t) = tiles[x as usize][y as usize] {
                let tile_index = (config.tileset.render)(t, rng);
                let offset_x = x as f32 * config.tileset.tile_width as f32;
                let offset_y = y as f32 * config.tileset.tile_height as f32;
                // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
                let flipped_y = config.width as f32 - offset_y - 1.0;
                let tile_entity = commands.spawn((
                    TileType { utility_type: t },
                    Sprite {
                        image: config.tileset.image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: config.tileset.layout.clone(),
                            index: tile_index,
                        }),
                        ..default()
                    },
                    Transform::from_xyz(offset_x, flipped_y, 0.0),
                ));
                tile_entities.push(tile_entity.id());
            }
        }
    }

    // set tiles as child of parent so that transforms cascade
    for tile in tile_entities {
        commands.entity(tilemap_entity).add_child(tile);
    }
}
