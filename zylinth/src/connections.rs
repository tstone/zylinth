use bevy::color::palettes::css::REBECCA_PURPLE;
use bevy::color::palettes::tailwind::INDIGO_600;
use bevy::prelude::*;

use crate::defs::{ControlLink, ControlSource, ControlTarget};
use crate::player::Player;
use crate::selection::Selectable;

#[derive(Resource, Default, Debug)]
pub struct ConnectionState {
    source_entity: Option<Entity>,
    source_id: Option<u8>,
    target_entity: Option<Entity>,
    target_id: Option<u8>,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionMode {
    #[default]
    Default,
    MakingConnection,
}

#[derive(Event)]
#[allow(unused)]
pub struct SourceStateChanged {
    pub source_id: u8,
    pub on: bool,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct ConnectionLine;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct TempConnectionLine;

pub struct ConnectionsPlugin;

impl Plugin for ConnectionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(link_added);
        app.add_observer(link_removed);
        app.add_event::<SourceStateChanged>();

        app.insert_state(ConnectionMode::Default);
        app.insert_resource(ConnectionState::default());

        app.init_gizmo_group::<ConnectionLine>();
        app.init_gizmo_group::<TempConnectionLine>();

        app.add_systems(Startup, line_setup);
        app.add_systems(Update, propagate_source_to_target);
        app.add_systems(
            Update,
            start_connection.run_if(in_state(ConnectionMode::Default)),
        );
        app.add_systems(
            Update,
            (render_temp_connection, end_connection)
                .run_if(in_state(ConnectionMode::MakingConnection)),
        );
    }
}

fn start_connection(
    selectables: Query<(
        &Selectable,
        Entity,
        Option<&ControlSource>,
        Option<&ControlTarget>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
    mut connection_state: ResMut<ConnectionState>,
    mut next_mode: ResMut<NextState<ConnectionMode>>,
    player: Query<&GlobalTransform, With<Player>>,
) {
    if input.any_just_pressed([KeyCode::KeyV, KeyCode::ControlRight]) {
        for (selectable, entity, source, target) in selectables.iter() {
            if selectable.selected {
                if let Some(source) = source {
                    connection_state.source_entity = Some(entity);
                    connection_state.source_id = Some(source.id);
                }
                if let Some(target) = target {
                    connection_state.target_entity = Some(entity);
                    connection_state.target_id = Some(target.id);
                }

                if connection_state.source_entity.is_some()
                    || connection_state.target_entity.is_some()
                {
                    // change state to connection active
                    next_mode.set(ConnectionMode::MakingConnection);
                } else {
                    debug!("Connection not started as no source or target found");
                }
            }
        }
    }
}

fn line_setup(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<TempConnectionLine>();
    config.line_style = GizmoLineStyle::Dotted;
    config.line_width = 3.0;

    let (config, _) = config_store.config_mut::<ConnectionLine>();
    config.line_width = 3.0;
}

fn render_temp_connection(
    player: Query<&GlobalTransform, With<Player>>,
    mut gizmos: Gizmos<TempConnectionLine>,
    connection_state: Res<ConnectionState>,
    entities: Query<&GlobalTransform, Without<Player>>,
) {
    let from_entity = connection_state
        .source_entity
        .or(connection_state.target_entity);
    if let Some(from_entity) = from_entity {
        let from_transform = entities.get(from_entity).unwrap();
        let player_transform = player.get_single().unwrap();
        let from = Vec2::new(
            from_transform.translation().x,
            from_transform.translation().y,
        );
        let to = Vec2::new(
            player_transform.translation().x,
            player_transform.translation().y + 10.0,
        );

        // TODO: maybe the player animation should have some cool buzzing "holding" animation (or gizmos circling around it)
        // TODO: animate this to look cooler, or apply a shader or something
        gizmos.line_gradient_2d(from, to, INDIGO_600, REBECCA_PURPLE);
    }
}

fn end_connection(
    selectables: Query<(
        &Selectable,
        Entity,
        Option<&ControlSource>,
        Option<&ControlTarget>,
    )>,
    input: Res<ButtonInput<KeyCode>>,
    mut connection_state: ResMut<ConnectionState>,
    mut next_mode: ResMut<NextState<ConnectionMode>>,
    mut commands: Commands,
) {
    if input.any_just_pressed([KeyCode::KeyV, KeyCode::ControlRight]) {
        for (selectable, entity, source, target) in selectables.iter() {
            if selectable.selected {
                match (
                    connection_state.source_id,
                    connection_state.target_id,
                    source,
                    target,
                ) {
                    // there is at least a from source and only a to target
                    (Some(source_id), _, None, Some(target))
                        if connection_state.source_entity != Some(entity) =>
                    {
                        debug!("Created connection from {source_id} to {}", target.id);
                        commands.spawn(ControlLink::new(source_id, target.id));
                    }
                    // there is at least a from target, and only a to source
                    (_, Some(target_id), Some(source), None)
                        if connection_state.target_entity != Some(entity) =>
                    {
                        debug!("Created connection from {} to {target_id}", source.id);
                        commands.spawn(ControlLink::new(source.id, target_id));
                    }
                    // there is only a from source, and at least a to target
                    (Some(source_id), None, _, Some(target))
                        if connection_state.source_entity != Some(entity) =>
                    {
                        debug!("Created connection from {source_id} to {}", target.id);
                        commands.spawn(ControlLink::new(source_id, target.id));
                    }
                    // there is only a from target, and at least a to source
                    (None, Some(target_id), Some(source), _)
                        if connection_state.target_entity != Some(entity) =>
                    {
                        debug!("Created connection from {} to {target_id}", source.id);
                        commands.spawn(ControlLink::new(source.id, target_id));
                    }
                    _ => {
                        warn!(
                            "Found ambiguous or disallowed case for connection: {:?} {:?} {:?} {:?}",
                            connection_state.source_id, connection_state.target_id, source, target,
                        );
                    }
                }
            }
        }

        // TODO: turn off temp

        connection_state.source_entity = None;
        connection_state.source_id = None;
        connection_state.target_entity = None;
        connection_state.target_id = None;
        next_mode.set(ConnectionMode::Default);
    }
}

pub fn propagate_source_to_target(
    mut ev_sourcestate: EventReader<SourceStateChanged>,
    links: Query<&ControlLink>,
    mut targets: Query<&mut ControlTarget>,
) {
    for event in ev_sourcestate.read() {
        for link in links.iter() {
            if link.source == event.source_id {
                let mut target = targets
                    .iter_mut()
                    .find(|target| target.id == link.target)
                    .unwrap();

                if event.on && !target.activated {
                    target.activated = true;
                } else if !event.on && target.activated {
                    target.activated = false;
                }
            }
        }
    }
}

fn link_added(
    trigger: Trigger<OnAdd, ControlLink>,
    links: Query<(&ControlLink, Entity)>,
    mut sources: Query<&mut ControlSource>,
    mut targets: Query<&mut ControlTarget>,
    mut commands: Commands,
) {
    let (link, new_link_entity) = links.get(trigger.entity()).unwrap();
    let mut new_target = targets.iter_mut().find(|t| t.id == link.target).unwrap();

    // despawn other link if present
    for (other_link, other_link_entity) in links.iter() {
        if other_link.target == new_target.id && other_link_entity != new_link_entity {
            // despawning the link will trigger `link_removed` and should take care of
            // updating the old source/targets
            debug!(
                "Despawning old link from {} to {}",
                other_link.source, other_link.target
            );
            commands.entity(other_link_entity).despawn();
            break;
        }
    }

    // update new source state
    let mut source = sources.iter_mut().find(|s| s.id == link.source).unwrap();
    source.connected = true;

    // update new target state
    new_target.connected = true;
    new_target.activated = source.on; // propagate activation    
}

fn link_removed(
    trigger: Trigger<OnAdd, ControlLink>,
    links: Query<&ControlLink>,
    mut sources: Query<&mut ControlSource>,
    mut targets: Query<&mut ControlTarget>,
) {
    let old_link = links.get(trigger.entity()).unwrap();

    // it could be the case that a prior connection was severed but one still exists
    // make sure there are no more remaining links for this target before marking it as disconnected
    if let Some(mut old_source) = sources.iter_mut().find(|s| s.id == old_link.source) {
        let remaining_links = links
            .iter()
            .filter(|l| l.source == old_source.id)
            .collect::<Vec<_>>();
        if remaining_links.len() == 0 {
            old_source.connected = false;
        }
    }

    if let Some(mut old_target) = targets.iter_mut().find(|t| t.id == old_link.target) {
        let remaining_links = links
            .iter()
            .filter(|l| l.target == old_target.id)
            .collect::<Vec<_>>();
        if remaining_links.len() == 0 {
            old_target.connected = false;
        }
    }
}
