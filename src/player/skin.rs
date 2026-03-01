/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};

use super::*;

#[derive(Component)]
pub struct LeftForarmSkin;

pub fn setup_body_skin(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // spawn the hands
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(10.0, ARM_LENGTH))),
        MeshMaterial2d(materials.add(Color::from(GRAY_300))),
        LeftForarmSkin,
    ));
}

pub fn draw_body(
    body: Single<(&mut Transform, &mut Body)>,
    // q_left_foot: Query<(&mut Transform), (With<LeftFoot>, Without<Body>)>,
    // q_right_foot: Query<(&mut Transform), (With<RightFoot>, Without<Body>, Without<LeftFoot>)>,
    // q_right_hand: Query<(&Transform), (With<RightHand>, Without<Body>, Without<LeftFoot>, Without<RightFoot>)>,
    // left_hand: Single<(&Transform), (With<LeftHand>, Without<Body>, Without<LeftFoot>, Without<RightFoot>,Without<RightHand>)>,
    left_hand: Single<&Transform, (With<LeftHand>, Without<Body>)>,
    mut left_arm_skin: Single<&mut Transform, (With<LeftForarmSkin>, Without<LeftHand>, Without<Body>)>,
    mut gizmos: Gizmos,
) {
    // Draw the left arm

    let direction = (left_hand.translation - body.0.translation).normalize();
    let distance = body.0.translation.distance(left_hand.translation);
    let middle_position = body.0.translation + direction * distance/2.0;

    let angle = direction.angle_between(body.0.translation.normalize());
    println!("angle is {}", angle.to_degrees());

    left_arm_skin.translation = middle_position;
    if left_hand.translation.x < body.0.translation.x {
        left_arm_skin.rotation = Quat::from_axis_angle(Vec3::Z, angle -std::f32::consts::PI);
    } else {
        left_arm_skin.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }

    gizmos.line_2d(body.0.translation.truncate(), (body.0.translation + direction * distance).truncate(), YELLOW_300);
}