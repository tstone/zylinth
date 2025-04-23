use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::map::{Tile, TileRole, TuesdayTile};
use crate::player::Player;

#[derive(Component)]
pub struct Switch {
    pub id: u8,
    pub on: bool,
}

#[derive(Event)]
#[allow(unused)]
pub struct SwitchStateChanged {
    pub switch_id: u8,
    pub on: bool,
}

pub struct SwitchPlugin;

impl Plugin for SwitchPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwitchStateChanged>();
        app.add_observer(configure_switch);
        app.add_systems(
            PostUpdate,
            press_switch.after(TransformSystem::TransformPropagate),
        );
        app.add_systems(Update, update_switch_sprite);
    }
}

pub fn configure_switch(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(&Tile, Entity)>,
    mut commands: Commands,
    mut ev_switchstate: EventWriter<SwitchStateChanged>,
) {
    let (tile, entity) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::Switch(id, on)) = tile.role {
        debug!("Setting up switch {id}");
        commands.entity(entity).insert(Switch { id, on });
        ev_switchstate.send(SwitchStateChanged { switch_id: id, on });

        // add collider
        let collider = commands
            .spawn((
                RigidBody::Static,
                Collider::ellipse(10.0, 8.0),
                Transform::from_xyz(0.0, 5.0, 0.1),
            ))
            .id();
        commands.entity(entity).add_child(collider);
    }
}

pub fn press_switch(
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut tiles: Query<(&GlobalTransform, &mut Switch)>,
    mut ev_switchstate: EventWriter<SwitchStateChanged>,
) {
    if input.any_just_released([KeyCode::Enter, KeyCode::KeyF]) {
        if let Ok(player) = player.get_single() {
            let player_translation = player.translation();
            for (transform, mut switch) in tiles.iter_mut() {
                // TODO: does Parry/Avian have a more efficient way to do this?
                let translation = transform.translation();
                let a = (translation.x - player_translation.x).powf(2.);
                let b = (translation.y - player_translation.y).powf(2.);
                let distance = (a + b).sqrt();
                if distance < 27.50 {
                    switch.on = !switch.on;
                    debug!("switch {} changed to: {}", switch.id, switch.on);
                    ev_switchstate.send(SwitchStateChanged {
                        switch_id: switch.id,
                        on: switch.on,
                    });
                }
            }
        }
    }
}

pub fn update_switch_sprite(mut switch: Query<(&Switch, &mut Sprite), Changed<Switch>>) {
    for (switch, mut sprite) in switch.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if switch.on {
                atlas.index = TuesdayTile::SwitchRight(1).into();
            } else {
                atlas.index = TuesdayTile::SwitchLeft(1).into();
            }
        }
    }
}
