/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/
use std::cmp::Ordering;

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;

use crate::mouse::MousePosition;
use crate::wall::holds::{LeftHandOnHold, RightHandOnHold, LeftFootClosestHold, RightFootClosestHold};

pub mod body;
pub mod hand;
mod skin;

use crate::GameState;
pub use body::*;
use hand::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, skin::setup_body_skin);
        app.add_systems(Update, body::body_parts_speed_limiter);
        app.add_systems(Update, hands_control.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, feet_control.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, body_control.run_if(in_state(GameState::Playing)));
        // app.add_systems(Update, skin::draw_body);
    }
}

fn feet_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_body: Query<(&mut Transform, &mut Body)>,
    mut q_left_foot: Query<(&mut Transform, &mut Velocity, Entity, &mut LeftFoot), (Without<Body>)>,
    mut q_right_foot: Query<(&mut Transform, &mut Velocity, Entity, &mut RightFoot), (Without<Body>, Without<LeftFoot>)>,
    r_right_hand: Query<(&Transform, &RightHand), (Without<Body>, Without<LeftFoot>, Without<RightFoot>)>,
    r_left_hand: Query<(&Transform, &LeftHand), (Without<Body>, Without<LeftFoot>, Without<RightFoot>,Without<RightHand>)>,
    left_target: Res<LeftFootClosestHold>,
    right_target: Res<RightFootClosestHold>,
    mut gizmos: Gizmos,
) {
    // Predict the feet position based on the body position.
    let mut body = q_body.single_mut().unwrap();
    let mut left_foot = q_left_foot.single_mut().unwrap();
    let mut right_foot = q_right_foot.single_mut().unwrap();
    let left_hand = r_left_hand.single().unwrap();
    let right_hand = r_right_hand.single().unwrap();

    let distance_threshold = 8.0;
    let foot_offset = 0.0;
    // let foot_offset = if keys.pressed(KeyCode::Space) {
    //     LEG_LENGTH/3.0
    // } else {
    //     0.0
    // };

    // let both_and_on_hold = left_hand.1.is_holding && right_hand.1.is_holding;
    let feet_can_move = body.1.active_limb.is_none();
    // let feet_can_move = feet_can_move && adapt_feet_position;

    // Find best foot position
    // let body_to_hand_distance = body.0.translation.distance(left_hand.0.translation);
    let body_to_hand_distance = (body.0.translation.x - left_hand.0.translation.x).abs();
    // let left_foot_target = body.0.translation + Vec3::new(-BODY_WIDTH/3.0-body_to_hand_distance/2.0, -BODY_HEIGHT/2.0 - LEG_LENGTH/2.2 + foot_offset, 0.0);
    let left_foot_target = left_target.0.extend(0.0);
    gizmos.circle_2d(left_foot_target.truncate(), 4.0, RED_200);
    // Right foot
    // let body_to_hand_distance = body.0.translation.distance(right_hand.0.translation);
    let body_to_hand_distance = (body.0.translation.x - right_hand.0.translation.x).abs();
    // let right_foot_target = body.0.translation + Vec3::new(BODY_WIDTH/3.0 + body_to_hand_distance/2.0, -BODY_HEIGHT/2.0 - LEG_LENGTH/2.2 + foot_offset, 0.0);
    let right_foot_target = right_target.0.extend(0.0);
    gizmos.circle_2d(right_foot_target.truncate(), 4.0, BLUE_200);

    // Left foot
    let distance_to_target = left_foot.0.translation.distance(left_foot_target);
    let foot_is_moving = body.1.active_limb == Some(LimbType::LeftFoot);

    // if !left_foot.3.is_moving && distance_to_target > distance_threshold && feet_can_move{
    if !foot_is_moving && feet_can_move && distance_to_target > distance_threshold {
            left_foot.3.is_moving = true;
            body.1.active_limb = Some(LimbType::LeftFoot);
            commands.entity(left_foot.2).remove::<RigidBody>();
            commands.entity(left_foot.2).insert(RigidBody::KinematicVelocityBased);
    } else if foot_is_moving && distance_to_target < 2.0 {
            left_foot.3.is_moving = false;
            body.1.active_limb = None;
            left_foot.1.linvel = Vec2::ZERO;
            left_foot.1.angvel = 0.0;
    }

    if foot_is_moving {
        reach_smoothly_target(&left_foot.0, &mut left_foot.1, left_foot_target.truncate(), &mut gizmos);
    }

    // Right feet
    let distance_to_target = right_foot.0.translation.distance(right_foot_target);
    let foot_is_moving = body.1.active_limb == Some(LimbType::RightFoot);

    // if !right_foot.3.is_moving && distance_to_target > distance_threshold && feet_can_move{
    if !foot_is_moving && feet_can_move && distance_to_target > distance_threshold {
            right_foot.3.is_moving = true;
            body.1.active_limb = Some(LimbType::RightFoot);
            commands.entity(right_foot.2).remove::<RigidBody>();
            commands.entity(right_foot.2).insert(RigidBody::KinematicVelocityBased);
    } else if foot_is_moving && distance_to_target < 2.0 {
            right_foot.3.is_moving = false;
            body.1.active_limb = None;
            right_foot.1.linvel = Vec2::ZERO;
            right_foot.1.angvel = 0.0;
    }

    if foot_is_moving {
        reach_smoothly_target(&right_foot.0, &mut right_foot.1, right_foot_target.truncate(), &mut gizmos);
    }

    // Update body internal position tracking
    body.1.left_leg.end = *left_foot.0;
    body.1.right_leg.end = *right_foot.0;
}

fn body_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_body: Query<(&mut Transform, &mut Velocity, &mut Body)>,
    mut gizmos: Gizmos,
) {
    let mut body = q_body.single_mut().unwrap();

    let body_target_position = body.2.resting_position + Vec3::Y * HEAD_HEIGHT * 2.0;
    gizmos.circle_2d(body_target_position.truncate(), 8.0, YELLOW_200);

    let body_can_move = body.2.active_limb.is_none();

    if keys.just_pressed(KeyCode::Space) {
        // Save the resting position to calculate the maximum pulling effort
        body.2.resting_position = body.0.translation;
    }
    if keys.pressed(KeyCode::Space) {
        reach_smoothly_height(&body.0, &mut body.1, body_target_position.y, &mut gizmos);
    } else {
        body.2.resting_position = body.0.translation;
    }

    // update body internal position tracking
    body.2.position = *body.0;
}

fn hands_control(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<MousePosition>,
    mut q_body: Query<(&mut Transform, &mut Body)>,
    mut q_left_hand: Query<(&mut Transform, &mut Velocity, Entity, &mut LeftHand), (Without<Body>)>,
    mut q_right_hand: Query<(&mut Transform, &mut Velocity, Entity, &mut RightHand), (Without<LeftHand>, Without<Body>)>,
    mut r_r_hand_on_hold: ResMut<RightHandOnHold>,
    mut r_l_hand_on_hold: ResMut<LeftHandOnHold>,
    mut gizmos: Gizmos,
    mut commands: Commands,
) {
    let mut body = q_body.single_mut().unwrap();

    // if !buttons.pressed(MouseButton::Left) || !keys.pressed(KeyCode::KeyA) {
    let mut hand_components = q_left_hand.single_mut().unwrap();
    if keys.just_pressed(KeyCode::KeyA) {
        println!("Disable gravity on left hand");
        hand_components.3.is_holding = false;
        body.1.active_limb = Some(LimbType::LeftHand);
        hand::disable_gravity(&mut commands, hand_components.2);
    }

    if keys.pressed(KeyCode::KeyA) {
        update_hand_position(&body.1, &body.0, &hand_components.0, &mut hand_components.1, &mouse_position.world_position, &mut gizmos);
        // hand_components.1.linvel = Vec2::new(0.0, 0.0);
    }

    if keys.just_released(KeyCode::KeyA) {
        if r_l_hand_on_hold.0 {
            println!("Grabbing hold with left hand");
            hand_components.3.is_holding = true;
        } else {
            hand::enable_gravity(&mut commands, hand_components.2);
            println!("Releasing hold with left hand");
            hand_components.3.is_holding = false;
        }
        body.1.active_limb = None;
        hand_components.1.linvel = Vec2::ZERO;
        hand_components.1.angvel = 0.0;
    }

    // update body internal position tracking
    body.1.left_arm.end = *hand_components.0;

    // if !buttons.pressed(MouseButton::Right) || !keys.pressed(KeyCode::KeyS) {
    let mut hand_components = q_right_hand.single_mut().unwrap();

    if keys.just_pressed(KeyCode::KeyS) {
        println!("Disable gravity on right hand");
        hand_components.3.is_holding = false;
        body.1.active_limb = Some(LimbType::RightHand);
        hand::disable_gravity(&mut commands, hand_components.2);
    }

    if keys.pressed(KeyCode::KeyS) {
        update_hand_position(&body.1, &body.0, &hand_components.0, &mut hand_components.1, &mouse_position.world_position, &mut gizmos);
        // hand_components.1.linvel = Vec2::new(0.0, 0.0);
    }

    if keys.just_released(KeyCode::KeyS) {
        if r_r_hand_on_hold.0 {
            println!("Grabbing hold with right hand");
            hand_components.3.is_holding = true;
        } else {
            hand::enable_gravity(&mut commands, hand_components.2);
            println!("Releasing hold with right hand");
            hand_components.3.is_holding = false;
        }
        body.1.active_limb = None;
        hand_components.1.linvel = Vec2::ZERO;
        hand_components.1.angvel = 0.0;
    }

    // update body internal position tracking
    body.1.right_arm.end = *hand_components.0;

    // Hand can not overlap
    // let mut left_hand = q_left_hand.single_mut().unwrap();
    // let mut right_hand = q_right_hand.single_mut().unwrap();

    // if left_hand.0.translation.x > right_hand.0.translation.x {
    //     left_hand.0.translation.x = right_hand.0.translation.x;
    // }
    // if right_hand.0.translation.x < left_hand.0.translation.x {
    //     right_hand.0.translation.x = left_hand.0.translation.x;
    // }
}

fn update_hand_position(
    body: &Body,
    body_transform: &Transform,
    hand_transform: &Transform,
    hand_velocity: &mut Velocity,
    mouse_position: &Vec2,
    gizmos: &mut Gizmos,
) {
    // Keep track of multiple restriction distances, from shoulder and feet
    let mut distance_limiters = Vec::new();

    // Compute the max distance from body
    let shoulder_approximated_position = body_transform.translation + Vec3::Y * HEAD_HEIGHT;
    let shoulder_pointer_distance = shoulder_approximated_position.distance(mouse_position.extend(0.0));
    // allow a small overreach to improve the playability
    let arm_reachable_distance = HEAD_HEIGHT * 4.0;
    let ray = Ray2d {
        origin: shoulder_approximated_position.truncate(),
        direction: Dir2::new_unchecked((mouse_position - shoulder_approximated_position.truncate()).normalize()),
    };
    let restricted_point = ray.origin + *ray.direction * arm_reachable_distance;
    distance_limiters.push(shoulder_approximated_position.distance(restricted_point.extend(0.0)));

    gizmos.circle_2d(shoulder_approximated_position.truncate(), arm_reachable_distance, YELLOW_100);

    // Compute the max distance from foot
    let max_foot_hand_distance = HEAD_HEIGHT * 9.0;
    let ray = Ray2d {
        origin: body.left_leg.end.translation.truncate(),
        direction: Dir2::new_unchecked((mouse_position - body.left_leg.end.translation.truncate()).normalize()),
    };
    let restricted_point = ray.origin + *ray.direction * max_foot_hand_distance;
    distance_limiters.push(shoulder_approximated_position.distance(restricted_point.extend(0.0)));
    gizmos.circle_2d(body.left_leg.end.translation.truncate(), max_foot_hand_distance, YELLOW_100);

    let ray = Ray2d {
        origin: body.right_leg.end.translation.truncate(),
        direction: Dir2::new_unchecked((mouse_position - body.right_leg.end.translation.truncate()).normalize()),
    };
    let restricted_point = ray.origin + *ray.direction * max_foot_hand_distance;
    distance_limiters.push(shoulder_approximated_position.distance(restricted_point.extend(0.0)));
    gizmos.circle_2d(body.right_leg.end.translation.truncate(), max_foot_hand_distance, YELLOW_100);

    // keep only the shortest position
    let min_limiter = distance_limiters.into_iter().min_by(|a, b|
        a.partial_cmp(&b).unwrap_or(Ordering::Less)).unwrap();
    let new_position = if shoulder_pointer_distance > min_limiter {
        let ray = Ray2d {
            origin: shoulder_approximated_position.truncate(),
            direction: Dir2::new_unchecked((mouse_position - shoulder_approximated_position.truncate()).normalize()),
        };
        ray.origin + *ray.direction * min_limiter
    } else {
        *mouse_position
    };
    
    reach_smoothly_target(hand_transform, hand_velocity, new_position, gizmos);
}

fn reach_smoothly_target(
    body_transform: &Transform,
    body_velocity: &mut Velocity,
    target_position: Vec2,
    gizmos: &mut Gizmos,
) {
    let hand_target_distance = body_transform.translation.distance(target_position.extend(0.0));
    let ray = Ray2d {
        origin: body_transform.translation.truncate(),
        direction: Dir2::new_unchecked((target_position - body_transform.translation.truncate()).normalize()),
    };
    gizmos.ray_2d(ray.origin, *ray.direction * hand_target_distance, Color::srgb(1.0, 1.0, 0.0));

    body_velocity.linvel = ray.direction * hand_target_distance.min(ARM_LENGTH/2.0) * 8.0;
}

fn reach_smoothly_height(
    body_transform: &Transform,
    body_velocity: &mut Velocity,
    target_height: f32,
    gizmos: &mut Gizmos,
) {
    let current_target = Vec3::new(body_transform.translation.x, target_height, body_transform.translation.z);
    let target_distance = body_transform.translation.distance(current_target);
    let ray = Ray2d {
        origin: body_transform.translation.truncate(),
        direction: Dir2::new_unchecked((current_target - body_transform.translation).truncate().normalize()),
    };
    gizmos.ray_2d(ray.origin, *ray.direction * target_distance, Color::srgb(1.0, 1.0, 0.0));

    body_velocity.linvel = ray.direction * target_distance.min(ARM_LENGTH/2.0) * 8.0;
}