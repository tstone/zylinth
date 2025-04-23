use std::fmt::Debug;

use avian2d::prelude::*;
use bevy::prelude::*;
use std::cmp;

use super::TileRole;
use super::plugin::{TileLayer, TileSprite};
use super::tileset::{Tileset, TilesetId};

#[allow(unused)]
#[derive(Component, Debug, Default)]
pub struct Tile {
    pub grid_x: u32,
    pub grid_y: u32,
    pub width: u8,
    pub height: u8,
    pub tileset_name: &'static str,
    pub role: Option<TileRole>,
}

#[allow(unused)]
#[derive(Component, Debug)]
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
}

#[derive(Event)]
pub struct RenderedTileLayer;

/// Whenever a new TileLayer is spawned this system will detect it and
/// render it into sprites
pub fn render_tilemap(
    trigger: Trigger<OnAdd, TileLayer>,
    query: Query<(&TileLayer, &Transform)>,
    tileset_ids: Query<(&Name, &TilesetId)>,
    tileset_assets: Res<Assets<Tileset>>,
    mut commands: Commands,
    mut ev_rendered: EventWriter<RenderedTileLayer>,
) {
    let (layer, transform) = query.get(trigger.entity()).unwrap();
    let tileset_id = tileset_ids
        .iter()
        .find(|(name, _)| **name == Name::new(layer.tileset_name))
        .map(|(_, id)| id)
        .unwrap();
    let tileset = tileset_assets.get(tileset_id.id).unwrap();

    let width = layer.grid.len() as u32;
    let height = layer.grid[0].len() as u32;
    let tilemap_entity = commands
        .spawn((Tilemap { width, height }, *transform, Visibility::Visible))
        .add_child(trigger.entity())
        .id();

    let mut tile_entities: Vec<Entity> = Vec::new();
    let collisions = spawn_collisions(width, height, tileset, &layer.grid, &mut commands);

    for x in 0..width {
        for y in 0..height {
            if let Some(t) = &layer.grid[x as usize][y as usize] {
                let offset_x = x as f32 * tileset.tile_width as f32;
                let offset_y = y as f32 * tileset.tile_height as f32;
                // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
                let flipped_y = width as f32 - offset_y - 1.0;

                if let Some(role) = &t.role {
                    debug!("Inserting a {:?} at {x},{y}", role);
                }

                let mut tile_entity = commands.spawn((
                    Tile {
                        grid_x: x,
                        grid_y: y,
                        width: tileset.tile_width,
                        height: tileset.tile_height,
                        tileset_name: layer.tileset_name,
                        role: t.role.clone(),
                    },
                    Sprite {
                        image: tileset.image.clone(),
                        texture_atlas: Some(TextureAtlas {
                            layout: tileset.layout.clone(),
                            index: t.index,
                        }),
                        ..default()
                    },
                    Transform::from_xyz(offset_x, flipped_y, layer.z),
                    // TODO: make this a nice debug plugin or something
                    // TextFont {
                    //     font_size: 8.0,
                    //     ..Default::default()
                    // },
                    // Text2d::new(format!("({},{})", x, y)),
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

    ev_rendered.send(RenderedTileLayer);
}

/// It becomes way too imperformant to create a collider per tile
/// Instead this method takes a grid of tiles and finds all neighboring impassible tiles
/// Then resolves those into a set of colliders mapped to the tile it should be inserted on
fn spawn_collisions(
    width: u32,
    height: u32,
    tileset: &Tileset,
    grid: &Vec<Vec<Option<TileSprite>>>,
    commands: &mut Commands,
) -> Vec<Vec<Option<Entity>>> {
    let mut count = 0;
    let mut grid = grid.clone();

    // group all horizontal neighbors
    let mut contiguous_impassables: Vec<(u32, u32, u32, u32)> = Vec::new();
    for y in 0..height {
        let mut start_x: Option<u32> = None;
        for x in 0..width {
            let tile = &grid[x as usize][y as usize];
            let has_collider = tile.clone().map_or(false, |t| t.collider);

            if has_collider {
                count += 1;
            }

            if start_x.is_none() && has_collider {
                // if there is no start x and the current tile is impassible
                start_x = Some(x);

                // if there is a start x AND this tile is not impassable or not defined or it's the last tile
            } else if start_x.is_some() && (!has_collider || x == (width - 1)) {
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
            let tile = &grid[x as usize][y as usize];
            let has_collider = tile.clone().map_or(false, |t| t.collider);

            if start_y.is_none() && has_collider {
                // if there is no start x and the current tile is impassible
                start_y = Some(y);

                // if there is a start x AND this tile is not impassable or not defined or it's the last tile
            } else if start_y.is_some() && (!has_collider || y == (height - 1)) {
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
            Transform::from_xyz(
                (width as f32 / 2.0) - (tileset.tile_width as f32 / 2.0),
                (height as f32 / -2.0) + (tileset.tile_height as f32 / 2.0),
                0.1,
            ),
        ));
        colliders[x0 as usize][y0 as usize] = Some(collider.id());
    }

    debug!("collisable tiles: {count}, colliders: {coll_count}");

    colliders
}
