/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

mod body;
mod hand;
mod holds;
mod mouse;
// mod physics;

use body::*;
use hand::*;
use holds::*;
use mouse::*;
// use physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        // .add_plugins(PhysicsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(MousePlugin)
        .insert_resource(holds::LeftHandOnHold(false))
        .insert_resource(holds::RightHandOnHold(false))
        .add_systems(Startup, setup_system)
        // .add_systems(Startup, setup_chain)
        .add_systems(Update, follow_mouse)
        .add_systems(Update, hand_on_holds_detection)
        // .add_systems(Update, body::movement)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a 2D camera
    commands.spawn((Camera2d, MainCamera));

    // Spwan a few shapes randomly on the screen
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 100.0;
        let y = (i as f32 - 2.0) * 50.0;

        let hold = Hold::new(Vec2::new(x, y));
        hold.spawn(&mut commands, &mut meshes, &mut materials);
    }

    // Spawn the body
    Body::spawn(&mut commands, &mut meshes, &mut materials);

    // Spawn a box collider as ground
    commands.spawn((
        Collider::cuboid(500.0, 10.0),
        Transform::from_translation(Vec3::new(0.0, -250.0, 0.0)),
    ));
}

fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<MousePosition>,
    mut q_body: Query<&mut Transform, With<Body>>,
    mut q_left_hand: Query<(&mut Transform, &mut Velocity), (With<LeftHand>, Without<Body>)>,
    mut q_right_hand: Query<(&mut Transform, &mut Velocity), (With<RightHand>, Without<LeftHand>, Without<Body>)>,
    mut r_r_hand_on_hold: ResMut<RightHandOnHold>,
    mut r_l_hand_on_hold: ResMut<LeftHandOnHold>,
    mut gizmos: Gizmos,
) {
    // if !buttons.pressed(MouseButton::Left) || !keys.pressed(KeyCode::KeyA) {
    if keys.pressed(KeyCode::KeyA) {
        let mut hand_transform = q_left_hand.single_mut().unwrap();
        let body = q_body.single().unwrap();

        update_hand_position(body, &mut hand_transform.0, &mouse_position.world_position, &mut gizmos);

        hand_transform.1.linvel = Vec2::new(0.0, 0.0);
    }

    // if !buttons.pressed(MouseButton::Right) || !keys.pressed(KeyCode::KeyS) {
    if keys.pressed(KeyCode::KeyS) {

        let mut hand_transform = q_right_hand.single_mut().unwrap();
        let body = q_body.single().unwrap();

        update_hand_position(body, &mut hand_transform.0, &mouse_position.world_position, &mut gizmos);
    }
}

fn update_hand_position(
    body: &Transform,
    hand_transform: &mut Transform,
    mouse_position: &Vec2,
    gizmos: &mut Gizmos,
) {
    let body_pointer_distance = body.translation.distance(mouse_position.extend(0.0));

    let ray = Ray2d {
            origin: body.translation.truncate(),
            direction: Dir2::new_unchecked((mouse_position - body.translation.truncate()).normalize()),
        };
    gizmos.ray_2d(ray.origin, *ray.direction, Color::srgb(1.0, 1.0, 0.0));

    if body_pointer_distance > ARM_LENGTH {
        let new_position = ray.origin + *ray.direction * ARM_LENGTH;
        hand_transform.translation.x = new_position.x;
        hand_transform.translation.y = new_position.y;
    } else {
        hand_transform.translation.x = mouse_position.x;
        hand_transform.translation.y = mouse_position.y;
    }
}

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Tail;

fn setup_chain(mut commands: Commands) {
    let n = 5;                 // number of links
    let link_length = 40.0;     // distance between links
    let start = Vec2::new(0.0, 0.0);

    let mut previous: Option<Entity> = None;

    for i in 0..n {
        // let position = start - Vec2::Y * link_length * i as f32 - Vec2::X * link_length * i as f32 * 0.1;
        // let position = start - Vec2::X * link_length/2.0 * i as f32;
        // let position = start + Vec2::X * link_length/4.0 * i as f32;
        let position = start + Vec2::X;

        let entity = commands
            .spawn((
                Collider::ball(4.0),
                Transform::from_translation(position.extend(0.0)),
                Damping {
                    linear_damping: 0.5,
                    angular_damping: 0.5,
                },
            ))
            .id();

        // Attach to previous link
        if let Some(prev) = previous {
            let joint = RevoluteJointBuilder::new()
                .local_anchor1(Vec2::ZERO)
                .local_anchor2(Vec2::new(0.0, -link_length))
                .build();

            commands.entity(entity).insert((ImpulseJoint::new(prev, joint), RigidBody::Dynamic));
        } else {
            // First element is fixed (anchor)
            commands.entity(entity).insert((RigidBody::Fixed, Head));
        }

        previous = Some(entity);
    }

    // last link is fixed (anchor)
    if let Some(last) = previous {
        commands.entity(last).insert((RigidBody::Fixed, Tail));
    }
}
