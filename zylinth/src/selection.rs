use core::f32;

use avian2d::prelude::*;
use bevy::{prelude::*, transform};

use crate::defs::GameLayer;
use crate::player::Player;

#[derive(Component, Default)]
pub struct Selectable {
    pub selected: bool,
}

pub struct SelectionPlugion;

impl Plugin for SelectionPlugion {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select);
    }
}

fn select(
    spacial_query: SpatialQuery,
    player: Query<&GlobalTransform, With<Player>>,
    transforms: Query<&GlobalTransform, With<Selectable>>,
    mut selectables: Query<(&mut Selectable, Entity)>,
) {
    let player_transform = player.get_single().unwrap();
    let player_transl = player_transform.translation();

    let mask = LayerMask(GameLayer::Interactables.to_bits());
    let intersections = spacial_query.shape_intersections(
        &Collider::circle(20.0),
        Vec2::new(player_transl.x, player_transl.y),
        0.0,
        &SpatialQueryFilter::from_mask(mask),
    );

    if intersections.len() > 0 {
        let player_pos = player.get_single().unwrap().translation();
        let nearest = intersections
            .iter()
            .map(|e| (*e, transforms.get(*e)))
            .filter(|(e, transform)| transform.is_ok())
            .map(|(e, transform)| (e, (player_pos - transform.unwrap().translation()).length()))
            .fold(None, |acc, e| match acc {
                None => Some(e),
                Some(acc) => {
                    if acc.1 < e.1 {
                        Some(e)
                    } else {
                        Some(acc)
                    }
                }
            });

        if let Some((target, _)) = nearest {
            for (mut selectable, entity) in selectables.iter_mut() {
                if entity != target && selectable.selected {
                    // unselect everything else
                    selectable.selected = false;
                } else if entity == target && !selectable.selected {
                    selectable.selected = true;
                }
            }
            return;
        }
    }

    for (mut selectable, _) in selectables.iter_mut() {
        selectable.selected = false;
    }
}
