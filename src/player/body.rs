/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;

use crate::hand::{LeftHand, RightHand, HAND_SIZE};
// use crate::hand::spawn;
// use crate::physics::RigidBody;

// An adult body is composed of 8 head heights
pub static BODY_HEAD_HEIGHT: f32 = 20.0;
pub static BODY_WIDTH: f32 = BODY_HEAD_HEIGHT * 2.0;
pub static BODY_HEIGHT: f32 = BODY_HEAD_HEIGHT * 1.8;
pub static ARM_LENGTH: f32 = BODY_HEAD_HEIGHT * 3.0;
pub static LEG_LENGTH: f32 = BODY_HEAD_HEIGHT * 4.0;
pub static JOINT_SIZE: f32 = 5.0;

static PHY_TO_PIX: f32 = 1.0;
static BODY_WIDTH_PHY: f32 = 40.0 / PHY_TO_PIX;
static BODY_HEIGHT_PHY: f32 = 50.0 / PHY_TO_PIX;
static ARM_LENGTH_PHY: f32 = 80.0 / PHY_TO_PIX;

#[derive(PartialEq)]
pub enum Limb {
    LeftHand,
    RightHand,
    LeftFoot,
    RightFoot,
}

#[derive(Component)]
pub struct Body {
    pub resting_position: Vec3,
    velocity: Vec2,
    pub active_limb: Option<Limb>,
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
    ) {

        let body = commands.spawn((
            Mesh2d(meshes.add(Capsule2d::new(BODY_WIDTH/2.0, BODY_HEIGHT))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Body {
                resting_position: Vec3::ZERO,
                velocity: Vec2::ZERO,
                active_limb: None,
            },
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Velocity::zero(),
            // Collider::capsule(vec2(0.0, -BODY_HEIGHT/2.0), vec2(0.0, BODY_HEIGHT/2.0), BODY_WIDTH/2.0),
            Collider::ball(BODY_WIDTH/2.0),
            // Collider::cuboid(15.0, 50.0),
            // Collider::capsule(vec2(0.0, -BODY_HEIGHT/2.0), vec2(0.0, BODY_HEIGHT), BODY_WIDTH/2.0),
            ColliderMassProperties::Density(8.0)
        )).with_child((
            // Mesh2d(meshes.add(Circle::new(BODY_HEAD_HEIGHT))),
            // Mesh2d(meshes.add(Ellipse::new(BODY_HEAD_HEIGHT/1.5, BODY_HEAD_HEIGHT))),
            Mesh2d(meshes.add(Capsule2d::new(BODY_HEAD_HEIGHT/1.5, BODY_HEAD_HEIGHT/1.8))),
            MeshMaterial2d(materials.add(Color::from(GRAY_400))),
            Transform::from_xyz(0.0, BODY_HEIGHT, 0.0),
        )).id();

        // Right arm
        let shoulder = commands.spawn((
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(ARM_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(ARM_LENGTH/4.0, 0.0, 0.0),
        )).id();
        let shoulder_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(BODY_WIDTH/2.0, BODY_HEIGHT/2.0))
            // shoulder anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(shoulder).insert(ImpulseJoint::new(body, shoulder_joint));

        let elbow = commands.spawn((
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(ARM_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(ARM_LENGTH/4.0, 0.0, 0.0),
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
            Mesh2d(meshes.add(Circle::new(HAND_SIZE))),
            MeshMaterial2d(materials.add(Color::from(BLUE_300))),
            Transform::from_xyz(ARM_LENGTH, 0.0, 0.0),
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
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(ARM_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(-ARM_LENGTH/4.0, 0.0, 0.0),
        )).id();
        let shoulder_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(-BODY_WIDTH/2.0, BODY_HEIGHT/2.0))
            // shoulder anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(shoulder).insert(ImpulseJoint::new(body, shoulder_joint));

        let elbow = commands.spawn((
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(ARM_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(-ARM_LENGTH/4.0, 0.0, 0.0),
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
            Mesh2d(meshes.add(Circle::new(HAND_SIZE))),
            MeshMaterial2d(materials.add(Color::from(RED_300))),
            Transform::from_xyz(-ARM_LENGTH, 0.0, 0.0),
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
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(LEG_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(0.0, -LEG_LENGTH/4.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0)),
        )).id();

        let hip_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(BODY_WIDTH_PHY/2.0, -BODY_HEIGHT_PHY/2.0))
            // hip anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(hip).insert(ImpulseJoint::new(body, hip_joint));

        let knee = commands.spawn((
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(LEG_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(0.0, -LEG_LENGTH/4.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0)),
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
            // Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            // MeshMaterial2d(materials.add(Color::from(BLUE_200))),
            Mesh2d(meshes.add(Rectangle::new(20.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
        )).id();
        let foot_joint = RevoluteJointBuilder::new()
            // foot anchor
            .local_anchor1(Vec2::new(0.0, -LEG_LENGTH/2.0))
            // foot anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(foot).insert(ImpulseJoint::new(knee, foot_joint));

        // Left leg
        let hip = commands.spawn((
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(LEG_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(0.0, -LEG_LENGTH/4.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0)),
        )).id();
        let hip_joint = RevoluteJointBuilder::new()
            // body anchor
            .local_anchor1(Vec2::new(-BODY_WIDTH_PHY/2.0, -BODY_HEIGHT_PHY/2.0))
            // hip anchor
            .local_anchor2(Vec2::ZERO)
            // .limits([0.0, 180.0_f32.to_radians()])
            .build();
        commands.entity(hip).insert(ImpulseJoint::new(body, hip_joint));

        let knee = commands.spawn((
            Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
            Mesh2d(meshes.add(Rectangle::new(LEG_LENGTH/2.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(0.0, -LEG_LENGTH/4.0, 0.0).with_rotation(Quat::from_rotation_z(std::f32::consts::PI/2.0)),
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
            // Mesh2d(meshes.add(Circle::new(JOINT_SIZE))),
            // MeshMaterial2d(materials.add(Color::from(RED_200))),
            Mesh2d(meshes.add(Rectangle::new(20.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
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
        )).id();
        let foot_joint = RevoluteJointBuilder::new()
            // foot anchor
            .local_anchor1(Vec2::new(0.0, -LEG_LENGTH/2.0))
            // foot anchor
            .local_anchor2(Vec2::ZERO)
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