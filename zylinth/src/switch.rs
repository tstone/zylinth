use bevy::log::tracing_subscriber::field::debug;
use bevy::prelude::*;

use crate::map::{Tile, TuesdayTile, TuesdayTile::*};
use crate::player::Player;

#[derive(Component)]
pub struct Switch {
    on: bool,
    // TODO: this needs
    // target: Entity
}

#[derive(Event)]
pub struct SwitchStateChanged {
    entity: Entity,
    on: bool,
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
    tiles: Query<(&Tile, &Sprite, Entity), Changed<Sprite>>,
    mut commands: Commands,
    mut ev_switchstate: EventWriter<SwitchStateChanged>,
) {
    let (tile, sprite, entity) = tiles.get(trigger.entity()).unwrap();
    if tile.tileset_name == TuesdayTile::name() {
        let index = sprite
            .texture_atlas
            .as_ref()
            .map(|atlas| atlas.index)
            .unwrap_or(usize::MAX);
        if index == SwitchLeft as usize {
            commands.entity(entity).insert(Switch { on: false });
            ev_switchstate.send(SwitchStateChanged { entity, on: false });
        } else if index == SwitchRight as usize {
            commands.entity(entity).insert(Switch { on: true });
            ev_switchstate.send(SwitchStateChanged { entity, on: true });
        }
    }
}

pub fn press_switch(
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut tiles: Query<(&GlobalTransform, &mut Switch, Entity)>,
    mut ev_switchstate: EventWriter<SwitchStateChanged>,
) {
    if input.any_just_released([KeyCode::Enter, KeyCode::KeyF]) {
        debug!("pressed activate");
        if let Ok(player) = player.get_single() {
            let player_translation = player.translation();
            debug!("got player");
            for (transform, mut switch, entity) in tiles.iter_mut() {
                let translation = transform.translation();
                let a = (translation.x - player_translation.x).powf(2.);
                let b = (translation.y - player_translation.y).powf(2.);
                let distance = (a + b).sqrt();
                debug!("distance: {distance}");
                if distance < 30.0 {
                    switch.on = !switch.on;
                    debug!("switch state changed to: {}", switch.on);
                    ev_switchstate.send(SwitchStateChanged {
                        entity,
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
                atlas.index = TuesdayTile::SwitchRight as usize;
            } else {
                atlas.index = TuesdayTile::SwitchLeft as usize;
            }
        }
    }
}
