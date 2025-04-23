use bevy::prelude::*;

use crate::map::{Tile, TuesdayTile, TuesdayTile::*};

#[derive(Component)]
pub struct DoorClosed;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(close_door);
    }
}

/// A system to render the door as "closed"
pub fn close_door(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(&Tile, &Sprite, Entity), Changed<Sprite>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (tile, sprite, entity) = tiles.get(trigger.entity()).unwrap();
    if tile.tileset_name == TuesdayTile::name() {
        let index = sprite
            .texture_atlas
            .as_ref()
            .map(|atlas| atlas.index)
            .unwrap_or(usize::MAX);
        if index == DoorFrame as usize {
            let square = meshes.add(Rectangle::new(32., 32.));
            let color = Color::srgb(1.0, 0.0, 0.0);
            let mesh = commands
                .spawn((
                    DoorClosed,
                    Mesh2d(square),
                    MeshMaterial2d(materials.add(color)),
                ))
                .id();
            commands.entity(entity).add_child(mesh);
        }
    }
}
