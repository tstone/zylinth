use bevy::prelude::*;
use bevy_lit::prelude::PointLight2d;

use crate::collision::Collidable;
use crate::movement::Velocity;
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
    //
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 16, 1, None, None);
    let anim_config = SpriteAnimConfig::new(0, 15, 11);

    let rect = meshes.add(Rectangle::new(32.0, 32.0));

    commands.spawn((
        Player,
        Collidable {
            width: 32.0,
            height: 32.0,
        },
        Velocity::starting_speed(85.0),
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
            intensity: 3.0,
            radius: 150.0,
            falloff: 3.0,
            ..default()
        },
        Transform::from_xyz(100., -100., 20.),
        Mesh2d(rect),
        MeshMaterial2d(materials.add(Color::hsl(100.0, 1.0, 0.5))),
    ));
}

fn player_keyboard_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Sprite), With<Player>>,
) {
    for (mut vel, mut sprite) in query.iter_mut() {
        // X
        if input.just_pressed(KeyCode::ArrowLeft) || input.just_pressed(KeyCode::KeyA) {
            vel.x = -1.0;
        } else if input.just_released(KeyCode::ArrowLeft) || input.just_released(KeyCode::KeyA) {
            vel.x = 0.0;
        }
        if input.just_pressed(KeyCode::ArrowRight) || input.just_pressed(KeyCode::KeyD) {
            vel.x = 1.0;
        } else if input.just_released(KeyCode::ArrowRight) || input.just_released(KeyCode::KeyD) {
            vel.x = 0.0;
        }

        // Y
        if input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyW) {
            vel.y = 1.0;
        } else if input.just_released(KeyCode::ArrowUp) || input.just_released(KeyCode::KeyW) {
            vel.y = 0.0;
        }
        if input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::KeyS) {
            vel.y = -1.0;
        } else if input.just_released(KeyCode::ArrowDown) || input.just_released(KeyCode::KeyS) {
            vel.y = 0.0;
        }

        // Change facing of playler sprite
        if vel.x > 0.0 {
            sprite.flip_x = false;
        } else if vel.x < 0.0 {
            sprite.flip_x = true;
        }
    }
}
