use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::map::{DoorControl, Tile, TileRole};
use crate::switch::{Switch, SwitchStateChanged};

#[derive(Component)]
pub struct Door {
    id: u8,
    open: bool,
}

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(init_door);
        app.add_systems(Update, switch_state_changed);
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
        commands.entity(entity).insert(Door { id, open: false });
        close_door(entity, tile, &mut commands, &mut meshes, &mut materials);
    }
}

pub fn switch_state_changed(
    mut ev_switchstate: EventReader<SwitchStateChanged>,
    door_controls: Query<&DoorControl>,
    switches: Query<&Switch>,
    colliders: Query<(Entity, &Parent), With<Collider>>,
    mut mesh_query: Query<(&mut Transform, &Parent), With<Mesh2d>>,
    mut doors: Query<(&mut Door, &Tile, Entity)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for door_control in door_controls.iter() {
        for event in ev_switchstate.read() {
            if door_control.switches.contains(&event.switch_id) {
                let all_on = door_control
                    .switches
                    .iter()
                    .map(|id| switches.iter().find(|sw| sw.id == *id).unwrap().on)
                    .fold(true, |acc, on| acc && on);

                // TODO: hard coded to 0 index here -- should figure out how multi-door works
                let (mut door, tile, door_entity) = doors
                    .iter_mut()
                    .find(|(door, _, _)| door.id == door_control.doors[0])
                    .unwrap();

                if all_on && !door.open {
                    door.open = true;
                    debug!("Opening door {}", door.id);
                    open_door(door_entity, &colliders, &mut mesh_query, &mut commands);
                } else if !all_on && door.open {
                    debug!("Closing door {}", door.id);
                    door.open = false;
                    close_door(
                        door_entity,
                        tile,
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                    );
                }
            }
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
