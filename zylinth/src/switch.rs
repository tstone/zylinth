use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

use crate::defs::GameLayer;
use crate::map::{Tile, TileRole, TuesdayTile};
use crate::selection::Selectable;

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
        app.add_systems(Update, update_switch_selection);
    }
}

fn configure_switch(
    trigger: Trigger<OnAdd, Tile>,
    tiles: Query<(&Tile, Entity)>,
    mut commands: Commands,
    mut ev_switchstate: EventWriter<SwitchStateChanged>,
) {
    let (tile, entity) = tiles.get(trigger.entity()).unwrap();
    if let Some(TileRole::Switch(id, on)) = tile.role {
        debug!("Setting up switch {id}");
        commands.entity(entity).insert((
            Switch { id, on },
            Selectable::default(),
            RigidBody::Static,
            Collider::ellipse(10.0, 8.0),
            CollisionLayers::new(GameLayer::Interactables, [GameLayer::Player]),
        ));
        ev_switchstate.send(SwitchStateChanged { switch_id: id, on });
    }
}

fn press_switch(
    input: Res<ButtonInput<KeyCode>>,
    mut switches: Query<(&mut Switch, &Selectable)>,
    mut ev_switchstate: EventWriter<SwitchStateChanged>,
) {
    if input.any_just_released([KeyCode::Enter, KeyCode::KeyF]) {
        for (mut switch, selectable) in switches.iter_mut() {
            if selectable.selected {
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

fn update_switch_sprite(mut switch: Query<(&Switch, &Selectable, &mut Sprite), Changed<Switch>>) {
    for (switch, selectable, mut sprite) in switch.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = sprite_index(switch, selectable);
        }
    }
}

fn update_switch_selection(
    mut switch: Query<(&Switch, &Selectable, &mut Sprite), Changed<Selectable>>,
) {
    for (switch, selectable, mut sprite) in switch.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = sprite_index(switch, selectable);
        }
    }
}

fn sprite_index(switch: &Switch, selectable: &Selectable) -> usize {
    if switch.on && selectable.selected {
        TuesdayTile::SwitchSelectedRight.into()
    } else if switch.on && !selectable.selected {
        TuesdayTile::SwitchRight(1).into()
    } else if !switch.on && !selectable.selected {
        TuesdayTile::SwitchLeft(1).into()
    } else {
        TuesdayTile::SwitchSelectedLeft.into()
    }
}
