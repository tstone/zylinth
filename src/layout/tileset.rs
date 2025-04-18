use bevy::prelude::*;

use super::cosmic_legacy::CosmicLegacyTile;

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

pub fn init_cosmic_tileset(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut tilemaps: ResMut<Assets<Tileset>>,
    mut commands: Commands,
) {
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(16),
        17,
        8,
        None,
        None,
    ));
    let h = tilemaps.add(Tileset {
        tile_width: 16,
        tile_height: 16,
        image: asset_server.load("CosmicLegacy_PetricakeGames.png"),
        layout,
    });

    commands.spawn((Name::new(CosmicLegacyTile::name()), TilesetId::new(h.id())));
}
