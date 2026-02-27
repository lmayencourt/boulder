/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::PrimaryWindow,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

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
        .add_systems(Update, body::body_speed_limiter)
        .add_systems(Update, hands_control)
        .add_systems(Update, feet_control)
        .add_systems(Update, hand_on_holds_detection)
        .add_systems(Update, move_hold)
        // .add_systems(Update, body::movement)
        .run();
}

#[derive(Component)]
pub struct LastHold {
    fixed_position: Option<Vec2>,
}

#[derive(Component)]
pub struct FirstHold;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a 2D camera
    commands.spawn((Camera2d, MainCamera));

    // Spwan a few shapes randomly on the screen
    for i in 0..20 {
        let x = rand::rng().random_range(-400.0..400.0);
        let y = rand::rng().random_range(-200.0..200.0);

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

    // Test spawning a body like chains

    let body_width = 40.0;
    let body_height = 20.0;
    let body = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(body_width, body_height))),
        MeshMaterial2d(materials.add(Color::from(GRAY_500))),
        Transform::from_xyz(250.0+0.0, 0.0, 0.0),
        // RigidBody::KinematicPositionBased,
        RigidBody::Dynamic,
        Collider::cuboid(body_width/2.0, body_height/2.0),
    )).id();

    // spawn a few links attached to the body
    let cube_size = 10.0;
    let cube = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(cube_size, cube_size))),
        MeshMaterial2d(materials.add(Color::from(GRAY_200))),
        Transform::from_xyz(250.0+body_width/2.0, -2.0 * body_height, 0.0),
        RigidBody::Dynamic,
        // RigidBody::KinematicPositionBased,
        Collider::cuboid(cube_size/2.0, cube_size/2.0),
    )).id();
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(body_width/2.0, -body_height/2.0))
        .local_anchor2(Vec2::new(0.0, body_height))
        .build();
    commands.entity(cube).insert(ImpulseJoint::new(body, joint));

    let small_cube_size = 8.0;
    let small_cube = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(small_cube_size, small_cube_size))),
        MeshMaterial2d(materials.add(Color::from(GRAY_200))),
        Transform::from_xyz(250.0+body_width/2.0, -3.0 * body_height, 0.0),
        // RigidBody::Dynamic,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(small_cube_size/2.0, small_cube_size/2.0),
        FirstHold,
    )).id();
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(0.0, body_height))
        .build();
    commands.entity(small_cube).insert(ImpulseJoint::new(cube, joint));

    // spawn a second link
    let circle_size = 15.0;
    let circle = commands.spawn((
        Mesh2d(meshes.add(Circle::new(circle_size/2.0))),
        MeshMaterial2d(materials.add(Color::from(GRAY_200))),
        Transform::from_xyz(250.0+-body_width/2.0, -2.0 * body_height, 0.0),
        RigidBody::Dynamic,
        // RigidBody::KinematicPositionBased,
        Collider::ball(circle_size/2.0),
    )).id();
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-body_width/2.0, -body_height/2.0))
        .local_anchor2(Vec2::new(0.0, body_height))
        // .local_anchor1(Vec2::new(0.0, body_height))
        // .local_anchor2(Vec2::new(body_width/2.0, body_height/2.0))
        .build();
    commands.entity(circle).insert(ImpulseJoint::new(body, joint));
    // commands.entity(body).insert(ImpulseJoint::new(circle, joint));

    let small_circle_size = 15.0;
    let small_circle = commands.spawn((
        Mesh2d(meshes.add(Circle::new(small_circle_size/2.0))),
        MeshMaterial2d(materials.add(Color::from(GRAY_200))),
        Transform::from_xyz(250.0+-body_width/2.0, -3.0 * body_height, 0.0),
        RigidBody::Dynamic,
        // RigidBody::KinematicPositionBased,
        Collider::ball(small_circle_size/2.0),
        LastHold{ fixed_position: None },
        Velocity::zero(),
    )).id();
    let joint = RevoluteJointBuilder::new()
        // .local_anchor1(Vec2::new(0.0, 0.0))
        // .local_anchor2(Vec2::new(0.0, body_height))
        .local_anchor1(Vec2::new(0.0, body_height))
        .local_anchor2(Vec2::new(0.0, 0.0))
        .build();
    commands.entity(small_circle).insert(ImpulseJoint::new(circle, joint));
    // commands.entity(circle).insert(ImpulseJoint::new(small_circle, joint));
}

fn move_hold(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_last_hold: Query<(&mut Transform, Entity, &mut LastHold, &mut Velocity)>,
    mut q_first_hold: Query<(&mut Transform, Entity), (With<FirstHold>, Without<LastHold>)>,
    mouse_position: Res<MousePosition>,
) {
    let mut last_hold = q_last_hold.single_mut().unwrap();
    let mut first_hold = q_first_hold.single_mut().unwrap();

    if keys.just_pressed(KeyCode::KeyD) {
        // commands.entity(last_hold.1).insert(Collider::ball(15.0));
        commands.entity(last_hold.1).remove::<RigidBody>();
        commands.entity(last_hold.1).insert(RigidBody::KinematicVelocityBased);
    }
    if keys.pressed(KeyCode::KeyD) {
        last_hold.0.translation = mouse_position.world_position.extend(0.0);
        last_hold.2.fixed_position = Some(mouse_position.world_position);
        last_hold.3.angvel = 0.0;
    // } else if let Some(fixed_position) = last_hold.2.fixed_position {
    //         last_hold.0.translation = fixed_position.extend(0.0);
    //         commands.entity(last_hold.1).remove::<Collider>();
    }
    if keys.just_released(KeyCode::KeyD) {
        commands.entity(last_hold.1).remove::<RigidBody>();
        commands.entity(last_hold.1).insert(RigidBody::Dynamic);
    }

    if keys.just_pressed(KeyCode::KeyF) {
        commands.entity(first_hold.1).remove::<RigidBody>();
        commands.entity(first_hold.1).insert(RigidBody::KinematicPositionBased);
    }
    if keys.pressed(KeyCode::KeyF) {
        first_hold.0.translation = mouse_position.world_position.extend(0.0);
    }
    if keys.just_released(KeyCode::KeyF) {
        commands.entity(first_hold.1).remove::<RigidBody>();
        commands.entity(first_hold.1).insert(RigidBody::Dynamic);
    }
}

fn feet_control(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut q_body: Query<&mut Transform, With<Body>>,
    mut q_left_foot: Query<(&mut Transform, &mut Velocity, Entity, &mut LeftFoot), (Without<Body>)>,
    mut q_right_foot: Query<(&mut Transform, &mut Velocity, Entity, &mut RightFoot), (Without<Body>, Without<LeftFoot>)>,
    r_right_hand: Query<(&Transform, &RightHand), (Without<Body>, Without<LeftFoot>, Without<RightFoot>)>,
    r_left_hand: Query<(&Transform, &LeftHand), (Without<Body>, Without<LeftFoot>, Without<RightFoot>,Without<RightHand>)>,
    mut gizmos: Gizmos,
) {
    // Predict the feet position based on the body position.
    let body = q_body.single_mut().unwrap();
    let mut left_foot = q_left_foot.single_mut().unwrap();
    let mut right_foot = q_right_foot.single_mut().unwrap();
    let left_hand = r_left_hand.single().unwrap();
    let right_hand = r_right_hand.single().unwrap();

    let both_and_on_hold = left_hand.1.is_holding && right_hand.1.is_holding;

    // Left foot
    let body_to_hand_distance = body.translation.distance(left_hand.0.translation);
    let foot_target = body.translation + Vec3::new(-BODY_WIDTH/2.0 - body_to_hand_distance/2.0, -BODY_HEIGHT/2.0 - LEG_LENGTH/1.5, 0.0);
    gizmos.circle_2d(foot_target.truncate(), 4.0, RED_200);

    let distance_to_target = left_foot.0.translation.distance(foot_target);

    if !left_foot.3.is_moving && distance_to_target > 40.0 && both_and_on_hold{
            left_foot.3.is_moving = true;
            commands.entity(left_foot.2).remove::<RigidBody>();
            commands.entity(left_foot.2).insert(RigidBody::KinematicVelocityBased);
    } else if left_foot.3.is_moving && distance_to_target < 2.0 {
            left_foot.3.is_moving = false;
            left_foot.1.linvel = Vec2::ZERO;
            left_foot.1.angvel = 0.0;
    }

    if left_foot.3.is_moving {
        reach_smoothly_target(&left_foot.0, &mut left_foot.1, foot_target.truncate(), &mut gizmos);
    }

    // Right feet
    let body_to_hand_distance = body.translation.distance(right_hand.0.translation);
    let foot_target = body.translation + Vec3::new(BODY_WIDTH/2.0 + body_to_hand_distance/2.0, -BODY_HEIGHT/2.0 - LEG_LENGTH/1.5, 0.0);
    gizmos.circle_2d(foot_target.truncate(), 4.0, BLUE_200);

    let distance_to_target = right_foot.0.translation.distance(foot_target);

    if !right_foot.3.is_moving && distance_to_target > 40.0 && both_and_on_hold{
            right_foot.3.is_moving = true;
            commands.entity(right_foot.2).remove::<RigidBody>();
            commands.entity(right_foot.2).insert(RigidBody::KinematicVelocityBased);
    } else if right_foot.3.is_moving && distance_to_target < 2.0 {
            right_foot.3.is_moving = false;
            right_foot.1.linvel = Vec2::ZERO;
            right_foot.1.angvel = 0.0;
    }

    if right_foot.3.is_moving {
        reach_smoothly_target(&right_foot.0, &mut right_foot.1, foot_target.truncate(), &mut gizmos);
    }
}

fn hands_control(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<MousePosition>,
    mut q_body: Query<&mut Transform, With<Body>>,
    mut q_left_hand: Query<(&mut Transform, &mut Velocity, Entity, &mut LeftHand), (Without<Body>)>,
    mut q_right_hand: Query<(&mut Transform, &mut Velocity, Entity, &mut RightHand), (Without<LeftHand>, Without<Body>)>,
    mut r_r_hand_on_hold: ResMut<RightHandOnHold>,
    mut r_l_hand_on_hold: ResMut<LeftHandOnHold>,
    mut gizmos: Gizmos,
    mut commands: Commands,
) {
    // if !buttons.pressed(MouseButton::Left) || !keys.pressed(KeyCode::KeyA) {
    let mut hand_components = q_left_hand.single_mut().unwrap();
    if keys.just_pressed(KeyCode::KeyA) {
        println!("Disable gravity on left hand");
        hand_components.3.is_holding = false;
        hand::disable_gravity(&mut commands, hand_components.2);
    }

    if keys.pressed(KeyCode::KeyA) {
        let body = q_body.single().unwrap();

        update_hand_position(body, &hand_components.0, &mut hand_components.1, &mouse_position.world_position, &mut gizmos);
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
        hand_components.1.linvel = Vec2::ZERO;
        hand_components.1.angvel = 0.0;
    }

    // if !buttons.pressed(MouseButton::Right) || !keys.pressed(KeyCode::KeyS) {
    let mut hand_components = q_right_hand.single_mut().unwrap();

    if keys.just_pressed(KeyCode::KeyS) {
        println!("Disable gravity on right hand");
        hand_components.3.is_holding = false;
        hand::disable_gravity(&mut commands, hand_components.2);
    }

    if keys.pressed(KeyCode::KeyS) {
        let body = q_body.single().unwrap();

        update_hand_position(body, &hand_components.0, &mut hand_components.1, &mouse_position.world_position, &mut gizmos);
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
        hand_components.1.linvel = Vec2::ZERO;
        hand_components.1.angvel = 0.0;
    }

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
    body: &Transform,
    hand_transform: &Transform,
    hand_velocity: &mut Velocity,
    mouse_position: &Vec2,
    gizmos: &mut Gizmos,
) {
    let body_pointer_distance = body.translation.distance(mouse_position.extend(0.0));

    if body_pointer_distance > ARM_LENGTH {
        let ray = Ray2d {
                origin: body.translation.truncate(),
                direction: Dir2::new_unchecked((mouse_position - body.translation.truncate()).normalize()),
            };

        let new_position = ray.origin + *ray.direction * ARM_LENGTH;
        
        reach_smoothly_target(hand_transform, hand_velocity, new_position, gizmos);
    } else {
        reach_smoothly_target(hand_transform, hand_velocity, *mouse_position, gizmos);
    }
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
