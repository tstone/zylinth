use bevy::color::palettes::tailwind::GRAY_300;
use bevy::prelude::*;
use bevy_lit::prelude::{AmbientLight2d, Lighting2dSettings};

use crate::player::Player;

pub struct CameraSetup;

impl Plugin for CameraSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera);
        app.add_systems(PostStartup, zoom);
        app.add_systems(Update, follow_player);
    }
}

fn camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        Lighting2dSettings { ..default() },
        AmbientLight2d {
            brightness: 0.175,
            color: Color::from(GRAY_300),
        },
    ));
}

fn zoom(mut query: Query<&mut OrthographicProjection, With<Camera2d>>) {
    let mut projection = query.single_mut();
    projection.scale = 0.3;
}

/// Force the camera to follow the player (at least in certain game modes)
fn follow_player(
    mut cam: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Changed<Transform>)>,
) {
    if !player.is_empty() && !cam.is_empty() {
        match (player.get_single(), cam.get_single_mut()) {
            (Ok(player), Ok(mut camera)) => {
                camera.translation.x = player.translation.x;
                camera.translation.y = player.translation.y;
            }
            _ => {}
        }
    }
}
