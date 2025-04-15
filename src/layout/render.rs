use avian2d::prelude::{Collider, RigidBody};
use bevy::{color::palettes::tailwind::GREEN_500, prelude::*};

use bevy_ecs_tilemap::prelude::*;
use bevy_lit::prelude::PointLight2d;
use rand::{prelude::*, random_range};
use rand_chacha::ChaCha8Rng;

use crate::layout::floor_plan::{from_maze, perlin_dog_bone, perlin_room};
use crate::layout::maze::Maze;
use crate::layout::{cosmic_legacy::decorate, fixer::floor_fixer};

use super::impassable::{IsImpassable, to_impassable};
use super::tilemap::Tileset;
use super::{cosmic_legacy::CosmicLegacyTile, shadowizer::shadowize, wall_wrap::wrap_walls};

pub fn generate_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: revert back to bevy_ecs_tilemap to see if that improves performance
    // TODO: randomly offset halls + allow "Z" paths -- need to have a "hallway" function
    // TODO: randomly relocate room center

    // TODO: create new "composite room" type that works by unioning together multiple random noise shapes
    // TODO: make room decoration types
    // - biology lab
    // - hydroponics lab
    // - library
    // - sewers
    // - office
    // TODO: use perlin noise or voronoi to slightly tint the floor to give large spaces variety

    // IDEAS:
    // Drone type 1 uses scanning and vaporizes targets
    // Drone type 2 has x-ray vision but leaves targets

    let seed = random_range(0..u64::MAX);
    debug!("Using seed: {seed}");
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    // let maze = Maze::generate(6, 4);
    // let floor = from_maze(&maze, 18, 12, &mut rng);
    // let floor = perlin_room(width as usize, height as usize, &mut rng);
    let floor = perlin_dog_bone(40, 16, &mut rng);
    let floor_fixed = floor_fixer(floor, &mut rng);
    let walled = wrap_walls(floor_fixed, &mut rng);
    let bg_decorations = decorate(&walled, &mut rng);
    let shadow_walls = shadowize(walled, &mut rng);

    render_tilemap(
        shadow_walls,
        &CosmicLegacyTile::to_utility_tileset(&asset_server),
        Transform::from_xyz(0.0, 0.0, -1.0),
        &mut commands,
        &mut rng,
        &asset_server,
    );
    render_tilemap(
        bg_decorations,
        &CosmicLegacyTile::to_cosmic_tileset(&asset_server),
        Transform::from_xyz(0.0, 0.0, 1.0),
        &mut commands,
        &mut rng,
        &asset_server,
    );
}

pub fn render_tilemap<T: Component + Copy + Clone + IsImpassable>(
    tiles: Vec<Vec<Option<T>>>,
    tileset: &Tileset<T>,
    transform: Transform,
    commands: &mut Commands,
    rng: &mut ChaCha8Rng,
    asset_server: &Res<AssetServer>,
) {
    let size = TilemapSize {
        x: tiles.len() as u32,
        y: tiles[0].len() as u32,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(size);
    let tiles = to_impassable(tiles);
    let mut tile_entities: Vec<Entity> = Vec::new();

    for x in 0..size.x {
        for y in 0..size.y {
            if let Some((t, impassable)) = tiles[x as usize][y as usize] {
                let tile_index = (tileset.render)(t, rng);
                // sprite maps are rendered with 0,0 in the bottom left so flip the Y coord
                let flipped_y = size.y - y - 1;
                let tile_pos = TilePos { x, y: flipped_y };
                debug!("tile index {tile_index}, pos {:?}", tile_pos);
                let mut tile_entity = commands.spawn((
                    // t,
                    TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(tile_index as u32),
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                ));

                // if impassable {
                //     tile_entity.insert((
                //         RigidBody::Static,
                //         Collider::rectangle(tileset.tile_width as f32, tileset.tile_height as f32),
                //         // CollisionMargin(0.1),
                //     ));
                // }

                tile_entities.push(tile_entity.id());
                tile_storage.set(&tile_pos, tile_entity.id());
            }
        }
    }

    let map_type = TilemapType::default();
    let texture: Handle<Image> = asset_server.load("CosmicLegacy_PetricakeGames.png");
    println!("is loaded? {}", asset_server.is_loaded(texture.id()));
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: tileset.tile_width as f32,
            y: tileset.tile_height as f32,
        },
        map_type,
        size,
        storage: tile_storage,
        // texture: TilemapTexture::Single(tileset.image.clone()),
        texture: TilemapTexture::Single(texture),
        tile_size: TilemapTileSize {
            x: tileset.tile_width as f32,
            y: tileset.tile_height as f32,
        },
        transform,
        ..Default::default()
    });
    for tile in tile_entities {
        commands.entity(tilemap_entity).add_child(tile);
    }
}

// TODO: move this somewhere better
pub fn spot_lights(tiles: Query<(&CosmicLegacyTile, Entity)>, mut commands: Commands) {
    for (tile, entity) in tiles.iter() {
        if *tile == CosmicLegacyTile::AlienTop {
            commands.entity(entity).insert((PointLight2d {
                color: Color::from(GREEN_500),
                radius: 40.0,
                intensity: 4.0,
                falloff: 8.0,
                ..default()
            },));
        }
    }
}
