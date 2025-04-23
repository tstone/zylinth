use bevy::prelude::*;

use super::tuesday::TuesdayTile;

#[derive(Asset, Reflect, PartialEq, Eq, Clone, Debug)]
pub struct Tileset {
    pub tile_width: u8,
    pub tile_height: u8,
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Debug)]
pub struct TilesetId {
    pub id: AssetId<Tileset>,
}

impl TilesetId {
    pub fn new(id: AssetId<Tileset>) -> Self {
        Self { id }
    }
}

pub fn init_tuesday_tileset(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut tilemaps: ResMut<Assets<Tileset>>,
    mut commands: Commands,
) {
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        11,
        6,
        None,
        None,
    ));
    let h = tilemaps.add(Tileset {
        tile_width: 32,
        tile_height: 32,
        image: asset_server.load("custom.png"),
        layout,
    });

    commands.spawn((Name::new(TuesdayTile::name()), TilesetId::new(h.id())));
}
