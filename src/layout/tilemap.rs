use std::fmt::Debug;

use crate::layout::impassable;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::text::cosmic_text::ttf_parser::gpos::Anchor;
use rand_chacha::ChaCha8Rng;
use std::cmp;

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

// TODO: should I still render a parent Tilemap?
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
    let collisions = spawn_collisions(width, height, tileset, &tiles, commands);

    for x in 0..width {
        for y in 0..height {
            if let Some(t) = tiles[x as usize][y as usize] {
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
                        anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                    Transform::from_xyz(offset_x, flipped_y, 0.0),
                ));

                // attach collider to top-left tile
                if let Some(collider) = &collisions[x as usize][y as usize] {
                    tile_entity.add_child(*collider);
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

/// It becomes way too imperformant to create a collider per tile
/// Instead this method takes a grid of tiles and finds all neighboring impassible tiles
/// Then resolves those into a set of colliders mapped to the tile it should be inserted on
fn spawn_collisions<T: Component + Copy + Clone + IsImpassable>(
    width: u32,
    height: u32,
    tileset: &Tileset<T>,
    grid: &Vec<Vec<Option<T>>>,
    commands: &mut Commands,
) -> Vec<Vec<Option<Entity>>> {
    // resolve tile ID to which are/aren't passable
    let mut grid = to_impassable(grid);

    let mut count = 0;

    // group all horizontal neighbors
    let mut contiguous_impassables: Vec<(u32, u32, u32, u32)> = Vec::new();
    for y in 0..height {
        let mut start_x: Option<u32> = None;
        for x in 0..width {
            let tile = grid[x as usize][y as usize];

            if tile.map_or(false, |(_, impassable)| impassable) {
                count += 1;
            }

            if start_x.is_none() && tile.map_or(false, |(_, impassable)| impassable) {
                // if there is no start x and the current tile is impassible
                start_x = Some(x);

                // if there is a start x AND this tile is not impassable or not defined or it's the last tile
            } else if start_x.is_some()
                && (tile.map_or(true, |(_, impassable)| !impassable) || x == (width - 1))
            {
                if let Some(start) = start_x {
                    // only save regions more than 1
                    if x - start > 2 {
                        // remove these tiles from the grid so as not to duplicate them with the vertical pass
                        for nx in start..=x {
                            grid[nx as usize][y as usize] = None;
                        }
                        contiguous_impassables.push((start, y, x, y));
                    }
                    start_x = None;
                }
            }
        }
    }

    // group all vertical neighbors excluding those who have been grouped by the horizontal grouping
    for x in 0..width {
        let mut start_y: Option<u32> = None;
        for y in 0..height {
            let tile = grid[x as usize][y as usize];

            if start_y.is_none() && tile.map_or(false, |(_, impassable)| impassable) {
                // if there is no start x and the current tile is impassible
                start_y = Some(y);

                // if there is a start x AND this tile is not impassable or not defined or it's the last tile
            } else if start_y.is_some()
                && (tile.map_or(true, |(_, impassable)| !impassable) || y == (height - 1))
            {
                if let Some(start) = start_y {
                    contiguous_impassables.push((x, start, x, y));
                    start_y = None;
                }
            }
        }
    }

    let mut coll_count = 0;

    // turn the list of collision neighbors into collider regions
    let mut colliders = vec![vec![None; height as usize]; width as usize];
    for (x0, y0, x1, y1) in contiguous_impassables {
        coll_count += 1;

        // x/y here are grid coordinates so they need to be scaled into
        // since they will be added to the parent the transform can be relative to the parent tilemap's position
        let width = cmp::max(x1 - x0, 1) * tileset.tile_width as u32;
        let height = cmp::max(y1 - y0, 1) * tileset.tile_height as u32;

        let collider = commands.spawn((
            RigidBody::Static,
            Collider::rectangle(width as f32, height as f32),
            Transform::from_xyz(width as f32 / 2.0, height as f32 / -2.0, 0.1),
        ));
        colliders[x0 as usize][y0 as usize] = Some(collider.id());
    }

    debug!("collisable tiles: {count}, colliders: {coll_count}");

    colliders
}
