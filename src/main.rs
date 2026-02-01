use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

mod body;
mod hand;
mod holds;
// mod physics;

use body::*;
use hand::*;
use holds::*;
// use physics::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        // .add_plugins(PhysicsPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_system)
        // .add_systems(Startup, setup_chain)
        .add_systems(Update, follow_mouse)
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

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

fn follow_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_left_hand: Query<(&mut Transform, &mut Velocity), With<LeftHand>>,
    mut q_right_hand: Query<(&mut Transform, &mut Velocity), (With<RightHand>, Without<LeftHand>)>,
    // mut head: Single<&mut Transform, (With<Head>, Without<LeftHand>, Without<RightHand>)>,
    // mut tail: Single<&mut Transform, (With<Tail>, Without<LeftHand>, Without<RightHand>, Without<Head>)>,
) {
    // if !buttons.pressed(MouseButton::Left) || !keys.pressed(KeyCode::KeyA) {
    if keys.pressed(KeyCode::KeyA) {

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
            // println!("World position: {:?}", world_position);

            let mut hand_transform = q_left_hand.single_mut().unwrap();
            hand_transform.0.translation.x = world_position.x;
            hand_transform.0.translation.y = world_position.y;
            hand_transform.1.linvel = Vec2::new(0.0, 0.0);

            // head.translation.x = world_position.x;
            // head.translation.y = world_position.y;
        }
    }

    // if !buttons.pressed(MouseButton::Right) || !keys.pressed(KeyCode::KeyS) {
    if keys.pressed(KeyCode::KeyS) {

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
            // println!("World position: {:?}", world_position);

            let mut hand_transform = q_right_hand.single_mut().unwrap();
            hand_transform.0.translation.x = world_position.x;
            hand_transform.0.translation.y = world_position.y;
            hand_transform.1.linvel = Vec2::new(0.0, 0.0);

            // tail.translation.x = world_position.x;
            // tail.translation.y = world_position.y;
        }
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
