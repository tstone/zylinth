use bevy::color::palettes::tailwind::GRAY_300;
use bevy::prelude::*;
use bevy_lit::prelude::{AmbientLight2d, Lighting2dSettings};
use bevy_pancam::PanCam;

pub fn camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Msaa::Off,
        PanCam::default(),
        Lighting2dSettings { ..default() },
        AmbientLight2d {
            brightness: 0.2,
            color: Color::from(GRAY_300),
        },
    ));
}

pub fn zoom(mut query: Query<&mut OrthographicProjection, With<Camera2d>>) {
    let mut projection = query.single_mut();
    projection.scale = 0.3;
}
