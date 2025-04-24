use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::connections::SourceStateChanged;
use crate::defs::{ControlLink, ControlTarget};
use crate::map::{Tile, TileRole};

#[derive(Component)]
pub struct Door;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(init_door);
        app.add_systems(Update, door_target_state_changed);
    }
}

/// A system to render the door as "closed"
pub fn init_door(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(&Tile, Entity)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (tile, entity) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::Door(id)) = tile.role {
        commands
            .entity(entity)
            .insert((Door, ControlTarget::off(id)));
        close_door(entity, tile, &mut commands, &mut meshes, &mut materials);
    }
}

pub fn door_target_state_changed(
    changed_doors: Query<(&mut ControlTarget, &Tile, Entity), (With<Door>, Changed<ControlTarget>)>,
    colliders: Query<(Entity, &Parent), With<Collider>>,
    mut mesh_query: Query<(&mut Transform, &Parent), With<Mesh2d>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (target, tile, entity) in changed_doors.iter() {
        if target.activated {
            debug!("Opening door {}", target.id);
            open_door(entity, &colliders, &mut mesh_query, &mut commands);
        } else {
            close_door(entity, tile, &mut commands, &mut meshes, &mut materials);
        }
    }
}

fn open_door(
    door_entity: Entity,
    colliders: &Query<(Entity, &Parent), With<Collider>>,
    meshes: &mut Query<(&mut Transform, &Parent), With<Mesh2d>>,
    commands: &mut Commands,
) {
    // TODO: nice exit animation where it slides down or something
    for (mut transform, parent) in meshes.iter_mut() {
        if parent.get() == door_entity {
            transform.scale = Vec3::new(1.0, 0.0, 1.0);
        }
    }

    // remove collider
    for (entity, parent) in colliders.iter() {
        if parent.get() == door_entity {
            commands.entity(entity).despawn();
        }
    }
}

fn close_door(
    door_entity: Entity,
    door_tile: &Tile,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // "closed" mesh
    let square = meshes.add(Rectangle::new(32., 32.));
    let color = Color::srgb(1.0, 0.0, 0.0);
    let mesh = commands
        .spawn((Mesh2d(square), MeshMaterial2d(materials.add(color))))
        .id();
    commands.entity(door_entity).add_child(mesh);

    // add collider
    let collider = commands
        .spawn((
            RigidBody::Static,
            Collider::rectangle(door_tile.width as f32, door_tile.height as f32),
            Transform::from_xyz(0., 0., 0.1),
        ))
        .id();
    commands.entity(door_entity).add_child(collider);
}
