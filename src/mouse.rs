/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::camera::MainCamera;

#[derive(Resource)]
pub struct MousePosition {
    pub world_position: Vec2,
}

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MousePosition{world_position: Vec2::ZERO});
        app.add_systems(
            Update,
            read_mouse_position,
        );
    }
}

pub fn read_mouse_position(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_position: ResMut<MousePosition>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = q_camera.single().unwrap();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| {
        let err = camera.viewport_to_world_2d(camera_transform, cursor);
            match err {
                Ok(pos) => Some(pos),
                Err(_) => None,
            }
        })
    {
        mouse_position.world_position = world_position;
        gizmos.circle_2d(world_position, 5.0, Color::srgb(0., 0.5, 1.));
        // println!("Mouse position: {:?}", world_position);
    }
}