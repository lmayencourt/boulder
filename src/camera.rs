/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};

use crate::MainCamera;
use crate::player::Body;

pub fn follow_player(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    q_body: Query<&Transform, (With<Body>, Without<MainCamera>)>,
) {
    let body_transform = q_body.single().unwrap();
    let mut camera_transform = q_camera.single_mut().unwrap();

    // Smoothly interpolate the camera's position towards the body
    let target_position = Vec3::new(body_transform.translation.x, body_transform.translation.y, camera_transform.translation.z);
    camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1);
}