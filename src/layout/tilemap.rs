use std::fmt::Debug;

use bevy::prelude::*;

use rand_chacha::ChaCha8Rng;

#[derive(Clone, Debug)]
pub struct Tileset<T> {
    pub render: fn(T, &mut ChaCha8Rng) -> usize,
    pub tile_width: u8,
    pub tile_height: u8,
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct Tile {
    x: u32,
    y: u32,
}

#[derive(Component)]
pub struct Tilemap {
    width: u32,
    height: u32,
}

pub fn render_tilemap<T: Component + Copy + Clone>(
    tiles: Vec<Vec<Option<T>>>,
    tileset: &Tileset<T>,
    transform: Transform,
    commands: &mut Commands,
    rng: &mut ChaCha8Rng,
) {
    let width = tiles.len() as u32;
    let height = tiles[0].len() as u32;
    let tilemap_entity = commands
        .spawn((Tilemap { width, height }, transform, Visibility::Visible))
        .id();
    let mut tile_entities: Vec<Entity> = Vec::new();

    for x in 0..width {
        for y in 0..height {
            if let Some(t) = tiles[x as usize][y as usize] {
                let tile_index = (tileset.render)(t, rng);
                let offset_x = x as f32 * tileset.tile_width as f32;
                let offset_y = y as f32 * tileset.tile_height as f32;
                // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
                let flipped_y = width as f32 - offset_y - 1.0;
                let tile_entity = commands.spawn((
                    t,
                    Tile { x, y },
                    Sprite {
                        image: tileset.image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: tileset.layout.clone(),
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
