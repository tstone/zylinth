use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::map::{DoorControl, Tile, TileRole};
use crate::switch::SwitchStateChanged;

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
    tiles: Query<(&Tile, Entity)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (tile, entity) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::Door(_id)) = tile.role {
        // "closed" mesh
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

        // add collider
        let collider = commands
            .spawn((
                RigidBody::Static,
                Collider::rectangle(tile.width as f32, tile.height as f32),
                Transform::from_xyz(0., 0., 0.1),
            ))
            .id();
        commands.entity(entity).add_child(collider);
    }
}

// pub fn switch_state_changed(
//     trigger: Trigger<SwitchStateChanged>,
//     door_controls: Query<&DoorControl>,
//     mut commands: Commands,
// ) {
//     trigger.event().switch_entity
// }
