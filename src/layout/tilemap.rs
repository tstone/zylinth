use std::fmt::Debug;

use bevy::prelude::*;

use rand_chacha::ChaCha8Rng;

use super::Impassable;
use super::impassable::{IsImpassable, to_impassable};

#[derive(Clone, Debug)]
pub struct Tileset<T> {
    pub render: fn(T, &mut ChaCha8Rng) -> usize,
    pub tile_width: u8,
    pub tile_height: u8,
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Debug)]
pub struct Tile {
    grid_x: u32,
    grid_y: u32,
    width: u8,
    height: u8,
    tile_index: usize,
}

impl Tile {
    pub fn into_rect(&self, transform: &GlobalTransform) -> Rect {
        let translation = transform.translation();
        Rect::new(
            translation.x,
            translation.y,
            translation.x + self.width as f32,
            translation.y + self.height as f32,
        )
    }
}

#[derive(Component, Debug)]
pub struct Tilemap {
    width: u32,
    height: u32,
}

pub fn render_tilemap<T: Component + Copy + Clone + IsImpassable>(
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
    let tiles = to_impassable(tiles);

    for x in 0..width {
        for y in 0..height {
            if let Some((t, impassable)) = tiles[x as usize][y as usize] {
                let tile_index = (tileset.render)(t, rng);
                let offset_x = x as f32 * tileset.tile_width as f32;
                let offset_y = y as f32 * tileset.tile_height as f32;
                // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
                let flipped_y = width as f32 - offset_y - 1.0;
                let mut tile_entity = commands.spawn((
                    t,
                    Tile {
                        grid_x: x,
                        grid_y: y,
                        width: tileset.tile_width,
                        height: tileset.tile_height,
                        tile_index,
                    },
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

                if impassable {
                    tile_entity.insert(Impassable);
                }

                tile_entities.push(tile_entity.id());
            }
        }
    }

    // set tiles as child of parent so that transforms cascade
    for tile in tile_entities {
        commands.entity(tilemap_entity).add_child(tile);
    }
}
