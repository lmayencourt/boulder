/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

#[derive(Resource)]
pub struct MousePosition {
    pub world_position: Vec2,
}

/// Used to help identify the camera that transforms screen coordinates to world coordinates.
/// Only one camera should have this component.
#[derive(Component)]
pub struct MainCamera;

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