/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::PrimaryWindow,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

mod camera;
mod corbusier_colors;
mod mouse;
mod player;
mod ui;
mod wall;
// mod physics;

use player::*;
use mouse::*;
use ui::UiPlugin;
use wall::*;
// use physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        // .add_plugins(PhysicsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(MousePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WallPlugin)
        .insert_resource(wall::holds::LeftHandOnHold(false))
        .insert_resource(wall::holds::RightHandOnHold(false))
        .add_systems(Startup, setup_system)
        .add_systems(Update, camera::follow_player)
        // .add_systems(Startup, setup_chain)
        .add_systems(Update, wall::holds::hand_on_holds_detection)
        // .add_systems(Update, move_hold)
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
