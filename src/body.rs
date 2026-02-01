use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;

use crate::hand::{LeftHand, RightHand};
// use crate::hand::spawn;
// use crate::physics::RigidBody;

static BODY_WIDTH: f32 = 40.0;
static BODY_HEIGHT: f32 = 50.0;
static ARM_LENGTH: f32 = 80.0;

static PHY_TO_PIX: f32 = 1.0;
static BODY_WIDTH_PHY: f32 = 40.0 / PHY_TO_PIX;
static BODY_HEIGHT_PHY: f32 = 50.0 / PHY_TO_PIX;
static ARM_LENGTH_PHY: f32 = 80.0 / PHY_TO_PIX;

#[derive(Component)]
pub struct Body {
    position: Vec2,
    velocity: Vec2,
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
                position: Vec2::ZERO,
                velocity: Vec2::ZERO,
            },
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::capsule(vec2(0.0, -BODY_HEIGHT/2.0), vec2(0.0, BODY_HEIGHT/2.0), BODY_WIDTH/2.0),
            // RigidBody::default(),
        )).id();

        // Right arm
        let hand = commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(BLUE_300))),
            Transform::from_xyz(ARM_LENGTH, 0.0, 0.0),
            RightHand,
            Pickable::IGNORE,
            RigidBody::KinematicPositionBased,
            Velocity::zero(),
            Collider::ball(4.0),
        )).id();

        let elbow = commands.spawn((
            Mesh2d(meshes.add(Circle::new(8.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(ARM_LENGTH/2.0, 0.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).id();
        let elbow_joint = RevoluteJointBuilder::new()
            // elbow anchor
            .local_anchor1(Vec2::ZERO)
            // hand anchor
            .local_anchor2(Vec2::new(ARM_LENGTH_PHY/2.0, 0.0))
            .build();
        commands.entity(elbow).insert(ImpulseJoint::new(hand, elbow_joint));

        let shoulder_joint = RevoluteJointBuilder::new()
            // elbow anchor
            .local_anchor1(Vec2::new(0.0, 0.0))
            // shoulder anchor
            .local_anchor2(Vec2::new(ARM_LENGTH_PHY/2.0, 0.0))
            .build();
        commands.entity(body).insert(ImpulseJoint::new(elbow, shoulder_joint));

        // Left arm
        let elbow = commands.spawn((
            Mesh2d(meshes.add(Circle::new(8.0))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(-ARM_LENGTH/2.0, 0.0, 0.0),
            Pickable::IGNORE,
            RigidBody::Dynamic,
            Collider::ball(4.0),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
        )).id();
        let shoulder_joint = RevoluteJointBuilder::new()
            // shoulder anchor
            .local_anchor1(Vec2::new(-ARM_LENGTH_PHY/2.0, BODY_HEIGHT/2.0))
            // elbow anchor
            .local_anchor2(Vec2::ZERO)
            .build();
        commands.entity(elbow).insert(ImpulseJoint::new(body, shoulder_joint));

        let hand = commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(RED_300))),
            Transform::from_xyz(-ARM_LENGTH, 0.0, 0.0),
            LeftHand,
            Pickable::IGNORE,
            RigidBody::KinematicPositionBased,
            Velocity::zero(),
            Collider::ball(4.0),
        )).id();
        let elbow_joint = RevoluteJointBuilder::new()
            // hand anchor
            .local_anchor1(Vec2::new(-ARM_LENGTH_PHY/2.0, 0.0))
            // elbow anchor
            .local_anchor2(Vec2::ZERO)
            .build();
        commands.entity(hand).insert(ImpulseJoint::new(elbow, elbow_joint));

    }
}

pub fn movement(
    mut query: Query<&mut Transform, With<Body>>,
    q_l_hand: Query<&Transform, (With<LeftHand>, Without<Body>)>,
    q_r_hand: Query<&Transform, (With<RightHand>, Without<Body>, Without<LeftHand>)>,
    time: Res<Time>,
) {
    let delta_t = time.delta_secs();

    let mut transform = query.single_mut().unwrap();

    // if transform.translation.y <= -200.0 {
    //     transform.translation.y = -200.0;
    // } else {
    //     // Gravity is applied by the rigid body system
    //     // rigid_body.acceleration.y = -450.0;
    // }

    // Restrict the position at a maximum distance from the hands
    // let l_hand_transform = q_l_hand.single().unwrap();
    // let r_hand_transform = q_r_hand.single().unwrap();

    
}