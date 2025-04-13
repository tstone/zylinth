use bevy::prelude::*;

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            speed: 50.0,
        }
    }
}

impl Velocity {
    pub fn starting_speed(speed: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            speed,
        }
    }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_movement);
    }
}

fn character_movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += vel.x * TIME_STEP * vel.speed;
        translation.y += vel.y * TIME_STEP * vel.speed;
    }
}
