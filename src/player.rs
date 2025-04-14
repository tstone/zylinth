use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

use crate::sprite_animation::SpriteAnimConfig;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_keyboard_input);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 16, 1, None, None);
    let anim_config = SpriteAnimConfig::new(0, 15, 11);

    commands.spawn((
        Player,
        Sprite {
            image: asset_server.load("D2.png"),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layouts.add(layout),
                index: anim_config.first_sprite_index,
            }),
            ..default()
        },
        anim_config,
        PointLight2d {
            color: Color::hsl(57.0, 0.6, 0.79),
            intensity: 3.25,
            radius: 160.0,
            falloff: 3.5,
            ..default()
        },
        // starting position
        Transform::from_xyz(120., -120., 20.),
        RigidBody::Dynamic,
        Collider::circle(14.0),
        TranslationExtrapolation,
        LockedAxes::ROTATION_LOCKED,
        LinearDamping(2.75),
    ));
}

fn player_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();

    let up = input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let vertical = up as i8 - down as i8;
    let direction = Vec2::new(horizontal as f32, vertical as f32).clamp_length_max(1.0);

    for (mut vel, mut sprite) in query.iter_mut() {
        vel.0 += 700.0 * delta_secs * direction;

        // Change facing of player sprite
        if direction.x > 0.0 {
            sprite.flip_x = false;
        } else if direction.x < 0.0 {
            sprite.flip_x = true;
        }
    }
}
