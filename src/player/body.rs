/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::hand::{LeftHand, RightHand, HAND_SIZE};
use crate::corbusier_colors::*;
use crate::mesh_drawing::*;
// use crate::hand::spawn;
// use crate::physics::RigidBody;

// An adult body is composed of 8 head heights
pub static HEAD_HEIGHT: f32 = 40.0;
pub static BODY_WIDTH: f32 = HEAD_HEIGHT * 1.6;
pub static BODY_HEIGHT: f32 = HEAD_HEIGHT * 2.5;
pub static ARM_LENGTH: f32 = HEAD_HEIGHT * 2.75;
pub static ARM_WIDTH: f32 = ARM_LENGTH / 4.0;
pub static HAND_HEIGHT: f32 = HEAD_HEIGHT / 1.25;
pub static HAND_WIDTH: f32 = HAND_HEIGHT / 2.0;
pub static LEG_LENGTH: f32 = HEAD_HEIGHT * 4.0;
pub static LEG_WIDTH: f32 = HEAD_HEIGHT / 1.25;
pub static JOINT_SIZE: f32 = 5.0;

static PHY_TO_PIX: f32 = 1.0;
static BODY_WIDTH_PHY: f32 = BODY_WIDTH / PHY_TO_PIX;
static BODY_HEIGHT_PHY: f32 = BODY_HEIGHT / PHY_TO_PIX;
static ARM_LENGTH_PHY: f32 = ARM_LENGTH / PHY_TO_PIX;

#[derive(PartialEq)]
pub enum LimbType {
    LeftHand,
    RightHand,
    LeftFoot,
    RightFoot,
}

#[derive(Component, Default)]
pub struct Body {
    pub position: Transform,
    pub resting_position: Vec3,
    pub active_limb: Option<LimbType>,
    pub left_arm: Limb,
    pub right_arm: Limb,
    pub left_leg: Limb,
    pub right_leg: Limb,
}

#[derive(Default)]
pub struct Limb {
    pub anchor_point: Transform,
    pub joint: Transform,
    pub end: Transform,
}

#[derive(Component)]
pub struct LeftFoot {
    pub is_moving: bool,
}

#[derive(Component)]
pub struct RightFoot {
    pub is_moving: bool,
}

impl Body {
    pub fn spawn(
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
    ) {

        let body_handle = asset_server.load("torso.png");
        let head_handle = asset_server.load("head.png");
        let forarm_handle = asset_server.load("forarm.png");
        let arm_handle = asset_server.load("arm.png");
        let hand_handle = asset_server.load("hand.png");
        let thigh_handle = asset_server.load("thigh.png");
        let calf_handle = asset_server.load("calf.png");
        let foot_handle = asset_server.load("foot.png");

        let body = commands.spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            Body::default(),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Velocity::zero(),
            // Collider::capsule(vec2(0.0, -BODY_HEIGHT/2.0), vec2(0.0, BODY_HEIGHT/2.0), BODY_WIDTH/2.0),
            Collider::ball(BODY_WIDTH/2.0),
            // Collider::cuboid(15.0, 50.0),
            // Collider::capsule(vec2(0.0, -BODY_HEIGHT/2.0), vec2(0.0, BODY_HEIGHT), BODY_WIDTH/2.0),
            ColliderMassProperties::Density(8.0)
        )).with_child((
            // The head
            Sprite {
                image: head_handle,
                custom_size: Some(Vec2::new(HEAD_HEIGHT*0.8, HEAD_HEIGHT)),
                ..Default::default()
            },
            Transform::from_xyz(0.0, HEAD_HEIGHT*1.6, 1.0),
        )).with_child((
            Sprite {
                image: body_handle,
                custom_size: Some(Vec2::new(BODY_WIDTH, BODY_HEIGHT)),
                ..Default::default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
        )).id();

        // Right arm
        let shoulder = commands.spawn((
            Transform::from_xyz(BODY_WIDTH/2.0, BODY_HEIGHT/2.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: arm_handle.clone(),
                custom_size: Some(Vec2::new(ARM_LENGTH/1.4, ARM_WIDTH)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_y(180_f32.to_radians())).with_translation(Vec3::new(ARM_LENGTH/4.0, 0.0, 1.0)),
        )).id();
        let shoulder_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(BODY_WIDTH/4.0, BODY_HEIGHT/2.8))
            // shoulder anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(shoulder).insert(ImpulseJoint::new(body, shoulder_joint));

        let forarm_points = [
            // Start with the top part and mirror it on y
            Vec2::new(-ARM_LENGTH/4.0, ARM_WIDTH/1.5),
            Vec2::new(ARM_LENGTH/4.0, ARM_WIDTH/2.0),
        ];
        let forarm_points = mirror_mesh_on_x_axis(&forarm_points);
        let forarm_shape = shapes::Polygon {
            points: forarm_points.clone().into_iter().collect(),
            closed: false,
        };
        let elbow = commands.spawn((
            Transform::from_xyz(ARM_LENGTH/2.0, 0.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: forarm_handle.clone(),
                custom_size: Some(Vec2::new(ARM_LENGTH/2.0, ARM_WIDTH)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_y(180_f32.to_radians())).with_translation(Vec3::new(ARM_LENGTH/4.0, 0.0, 2.0)),
        )).id();
        let elbow_joint = RevoluteJointBuilder::new()
            // elbow anchor
            .local_anchor1(Vec2::new(ARM_LENGTH_PHY/2.0, 0.0))
            // hand anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(elbow).insert(ImpulseJoint::new(shoulder, elbow_joint));

        let hand = commands.spawn((
            Sprite {
                image: hand_handle.clone(),
                custom_size: Some(Vec2::new(HAND_HEIGHT, HAND_WIDTH)),
                ..Default::default()
            },
            Transform::from_xyz(ARM_LENGTH, 0.0, 3.0),
            RightHand {
                is_holding: true,
            },
            Pickable::IGNORE,
            RigidBody::KinematicPositionBased,
            Velocity::zero(),
            Collider::ball(HAND_SIZE/2.0),
        )).id();
        let hand_joint = RevoluteJointBuilder::new()
            // hand anchor
            .local_anchor1(Vec2::new(ARM_LENGTH_PHY/2.0, 0.0))
            // hand anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(hand).insert(ImpulseJoint::new(elbow, hand_joint));


        // Left arm
        let shoulder = commands.spawn((
            Transform::from_xyz(-BODY_WIDTH/2.0, BODY_HEIGHT/2.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: arm_handle,
                custom_size: Some(Vec2::new(ARM_LENGTH/1.4, ARM_WIDTH)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_z(0_f32.to_radians())).with_translation(Vec3::new(-ARM_LENGTH/4., 0.0, 1.0)),
        )).id();
        let shoulder_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(-BODY_WIDTH/4.0, BODY_HEIGHT/3.0))
            // shoulder anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(shoulder).insert(ImpulseJoint::new(body, shoulder_joint));

        let elbow = commands.spawn((
            Transform::from_xyz(-ARM_LENGTH/2.0, 0.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: forarm_handle,
                custom_size: Some(Vec2::new(ARM_LENGTH/2.0, ARM_WIDTH)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_z(0_f32.to_radians())).with_translation(Vec3::new(-ARM_LENGTH/4.0, 0.0, 2.0)),
        )).id();
        let elbow_joint = RevoluteJointBuilder::new()
            // shoulder anchor
            .local_anchor1(Vec2::new(-ARM_LENGTH_PHY/2.0, 0.0))
            // elbow anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(elbow).insert(ImpulseJoint::new(shoulder, elbow_joint));

        let hand = commands.spawn((
            Sprite {
                image: hand_handle.clone(),
                custom_size: Some(Vec2::new(HAND_HEIGHT, HAND_WIDTH)),
                ..Default::default()
            },
            Transform::from_xyz(-ARM_LENGTH, 0.0, 3.0),
            LeftHand {
                is_holding: true,
            },
            Pickable::IGNORE,
            RigidBody::KinematicPositionBased,
            Velocity::zero(),
            Collider::ball(HAND_SIZE/2.0),
        )).id();
        let elbow_joint = RevoluteJointBuilder::new()
            // hand anchor
            .local_anchor1(Vec2::new(-ARM_LENGTH_PHY/2.0, 0.0))
            // elbow anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(hand).insert(ImpulseJoint::new(elbow, elbow_joint));

        let hip = commands.spawn((
            Transform::from_xyz(BODY_WIDTH/2.0, -BODY_HEIGHT/2.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(2.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: calf_handle.clone(),
                custom_size: Some(Vec2::new(LEG_WIDTH, LEG_LENGTH/1.8)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_y(180_f32.to_radians())).with_translation(Vec3::new(0., -LEG_LENGTH/4.0, 0.0)),
        )).id();

        let hip_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(BODY_WIDTH_PHY/5.0, -BODY_HEIGHT_PHY/2.8))
            // hip anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(hip).insert(ImpulseJoint::new(body, hip_joint));

        let knee = commands.spawn((
            Transform::from_xyz(-BODY_WIDTH/2.0, -BODY_HEIGHT + LEG_LENGTH/2.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: thigh_handle.clone(),
                custom_size: Some(Vec2::new(LEG_WIDTH, LEG_LENGTH/2.0)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_y(180_f32.to_radians())).with_translation(Vec3::new(0., -LEG_LENGTH/4.2, 0.0)),
        )).id();
        let knee_joint = RevoluteJointBuilder::new()
            // knee anchor
            .local_anchor1(Vec2::new(0.0, -LEG_LENGTH/2.0))
            // knee anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(knee).insert(ImpulseJoint::new(hip, knee_joint));

        let foot = commands.spawn((
            Transform::from_xyz(BODY_WIDTH/2.0, -BODY_HEIGHT - LEG_LENGTH, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            // RigidBody::KinematicPositionBased,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
            RightFoot {
                is_moving: false,
            },
        )).with_child((
            Sprite {
                image: foot_handle.clone(),
                custom_size: Some(Vec2::new(HAND_HEIGHT*1.25, HAND_WIDTH*1.25)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_y(180_f32.to_radians())),

        )).id();
        let foot_joint = RevoluteJointBuilder::new()
            // foot anchor
            .local_anchor1(Vec2::new(0.0, -LEG_LENGTH/2.0))
            // foot anchor
            .local_anchor2(Vec2::ZERO)
            .build();
        commands.entity(foot).insert(ImpulseJoint::new(knee, foot_joint));

        // Left leg
        let hip = commands.spawn((
            Transform::from_xyz(-BODY_WIDTH/2.0, -BODY_HEIGHT/2.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(2.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: calf_handle,
                custom_size: Some(Vec2::new(LEG_WIDTH, LEG_LENGTH/1.8)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_z(0_f32.to_radians())).with_translation(Vec3::new(0., -LEG_LENGTH/4.2, 0.0)),
        )).id();
        let hip_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(-BODY_WIDTH_PHY/5., -BODY_HEIGHT_PHY/2.8))
            // hip anchor
            .local_anchor2(Vec2::ZERO)
            .build();
        commands.entity(hip).insert(ImpulseJoint::new(body, hip_joint));

        let knee = commands.spawn((
            Transform::from_xyz(-BODY_WIDTH/2.0, -BODY_HEIGHT + LEG_LENGTH/2.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).with_child((
            Sprite {
                image: thigh_handle,
                custom_size: Some(Vec2::new(LEG_WIDTH, LEG_LENGTH/2.0)),
                ..Default::default()
            },
            Transform::from_rotation(Quat::from_rotation_y(0_f32.to_radians())).with_translation(Vec3::new(0., -LEG_LENGTH/4.0, 0.0)),
        )).id();
        let knee_joint = RevoluteJointBuilder::new()
            // knee anchor
            .local_anchor1(Vec2::new(0.0, -LEG_LENGTH/2.0))
            // knee anchor
            .local_anchor2(Vec2::ZERO)
            .build();
        commands.entity(knee).insert(ImpulseJoint::new(hip, knee_joint));

        let foot = commands.spawn((
            Transform::from_xyz(-BODY_WIDTH/2.0, -BODY_HEIGHT - LEG_LENGTH, 0.0),
            Pickable::IGNORE,
            // RigidBody::Dynamic,
            RigidBody::KinematicPositionBased,
            Collider::ball(4.0),
            Velocity::zero(),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
            LeftFoot{
                is_moving: false,
            },
        )).with_child((
            Sprite {
                image: foot_handle.clone(),
                custom_size: Some(Vec2::new(HAND_HEIGHT*1.25, HAND_WIDTH*1.25)),
                ..Default::default()
            },
        )).id();
        let foot_joint = RevoluteJointBuilder::new()
            // foot anchor
            .local_anchor1(Vec2::new(0.0, -LEG_LENGTH/2.0))
            // foot anchor
            .local_anchor2(Vec2::new(0.0, 0.0))
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(foot).insert(ImpulseJoint::new(knee, foot_joint));
    }
}

pub fn body_parts_speed_limiter(
    mut q_body_parts: Query<&mut Velocity>,
) {
    for mut velocity in q_body_parts.iter_mut() {
        let speed = velocity.linvel.length();
        let max_speed = 200.0;
        if speed > max_speed {
            velocity.linvel = velocity.linvel.normalize() * max_speed;
        }
    }
}

pub fn movement(
    mut q_body: Query<&mut Transform, With<Body>>,
    mut q_l_hand: Query<&mut Transform, (With<LeftHand>, Without<Body>)>,
    mut q_r_hand: Query<&mut Transform, (With<RightHand>, Without<Body>, Without<LeftHand>)>,
    time: Res<Time>,
) {
    let delta_t = time.delta_secs();

    let mut body = q_body.single_mut().unwrap();

    // if transform.translation.y <= -200.0 {
    //     transform.translation.y = -200.0;
    // } else {
    //     // Gravity is applied by the rigid body system
    //     // rigid_body.acceleration.y = -450.0;
    // }

    // Restrict the position at a maximum distance from the hands
    let mut l_hand_transform = q_l_hand.single_mut().unwrap();
    let mut r_hand_transform = q_r_hand.single_mut().unwrap();

    let l_hand_distance = body.translation.distance(l_hand_transform.translation);
    if l_hand_distance > ARM_LENGTH {
        let direction = Dir3::new_unchecked((body.translation - l_hand_transform.translation).normalize());
        l_hand_transform.translation = body.translation + direction * ARM_LENGTH;
    }

    // let r_hand_distance = body.translation.distance(r_hand_transform.translation);
    // if r_hand_distance > ARM_LENGTH {
    //     let direction = Dir3::new_unchecked((body.translation - r_hand_transform.translation).normalize());
    //     r_hand_transform.translation = body.translation + direction * ARM_LENGTH;
    // }
    
}