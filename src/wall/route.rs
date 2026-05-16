/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use rand::prelude::*;
use rand::rngs::ChaCha8Rng;

use crate::corbusier_colors::*;
use crate::{GameStateEvent, EndOfGameReason};
use crate::Body;
use crate::hand::{LeftHand, RightHand, HAND_SIZE, enable_gravity};
use super::holds::*;

static ROUTE_WIDTH_MIN: f32 = 150.0;
static ROUTE_WIDTH_MAX: f32 = ROUTE_WIDTH_MIN * 2.0;
static ROUTE_HEIGHT_MIN: f32 = 200.0;
static ROUTE_HEIGHT_MAX: f32 = ROUTE_HEIGHT_MIN * 10.0;

#[derive(Message, Default)]
pub struct SwitchToRoute {
    pub route_name: String,
}

impl SwitchToRoute {
    pub fn new(route_name: String) -> Self {
        SwitchToRoute { route_name }
    }
}

pub struct Path {
    pub holds: Vec<Hold>,
    pub params: RouteParams,
}

#[derive(Component)]
pub struct WallPart;

enum WallShape {
    Rectangle,
    Triangle,
    RegularPolygon,
}

#[derive(Debug)]
struct RouteParams {
    seed: u64,
    height: f32,
    width: f32,
    vertical_spacing: f32,
    number_of_holds: u32,
    rng: ChaCha8Rng,
}

impl RouteParams {
    pub fn new(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let height = rng.random_range(ROUTE_HEIGHT_MIN..ROUTE_HEIGHT_MAX);
        let width = rng.random_range(ROUTE_WIDTH_MIN..ROUTE_WIDTH_MAX);
        let vertical_spacing = 50.0;
        let number_of_holds = (height/vertical_spacing) as u32;

        Self {
            seed,
            height,
            width,
            vertical_spacing,
            number_of_holds,
            rng,
        }
    }
}

impl Path {
    pub fn new(name: &str) -> Self {
        let seed = Self::name_to_seed(&name);
        info!("Seed for {} is {}", name, seed);
        let mut params = RouteParams::new(seed);
        info!("Creating a new route {:?}", params);

        let mut holds = Vec::new();

        for i in 0..params.number_of_holds as i32 {
            let x = params.rng.random_range(-params.width/2.0..params.width/2.0);
            let y = i as f32 * params.vertical_spacing;
            let size = Vec2::new(params.rng.random_range(10.0..40.0), params.rng.random_range(5.0..15.0));
            holds.push(Hold::new(Vec2::new(x, y), size));
        }

        Self { holds, params }
    }

    fn name_to_seed(name: &str) -> u64 {
        // map the string to a numerical value
        let mut value: u64 = 0;
        for (i, char) in name.bytes().enumerate() {
            value += 10_u64.pow(i as u32) * char as u64;
        }

        value
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        asset_server: &Res<AssetServer>,
    ) {
        let mut hold_handles = Vec::new();
        for idx in 20..=29 {
            let handle = asset_server.load(format!("holds/hold_{}.png", idx));
            hold_handles.push(handle);
        }
        for hold in self.holds.iter().take(self.holds.len().saturating_sub(1)) {
            let hold_sprite = self.params.rng.random_range(0..10);
            hold.spawn(commands, meshes, materials, hold_handles[hold_sprite].clone());
        }
        // Spawn last hold
        if let Some(last_hold) = self.holds.last() {
            last_hold.spawn_last(commands, materials, hold_handles[0].clone());
        }

        // Spawn some shapes to visualize the route
        // Draw the founding blocks of the cliff
        let shape_count = self.params.rng.random_range(2..5);
        // The rotation is the same for all founding blocs
        for idx in 0..shape_count {
            let rotation = self.params.rng.random_range(-std::f32::consts::PI/8.0..std::f32::consts::PI/8.0);
            let shape_height = self.params.height / shape_count as f32;
            commands.spawn((
                // height is slightly bigger than the spacing between holds, so that it overlaps with the next one, creating a continuous path to follow
                Mesh2d(meshes.add(Rectangle::new(self.params.width * 1.2, shape_height * 1.1))),
                MeshMaterial2d(materials.add(COLOR_LIGHT_BROWN)),
                Transform::from_translation(Vec3::new(0.0, idx as f32 * shape_height + shape_height/2.0, 0.0))
                    .with_rotation(Quat::from_rotation_z(rotation)),
                WallPart,
            ));

            // Add smaller details in the founding blocks
            // let mut shapes = Self::create_shapes_in_block(
            //     self,
            //     Rectangle::new(self.params.width, shape_height),
            //     Vec2::new(0.0, idx as f32 * shape_height + shape_height/2.0),
            //     8,
            //     0.2,
            // );
            // let mut details = Vec::new();
            // for (_, transform, size) in &shapes {   
            //     // Add an extra layer of details with smaller shapes
            //     details = Self::create_shapes_in_block(
            //         self,
            //         Rectangle::new(size.half_size.x, size.half_size.y),
            //         transform.translation.truncate(),
            //         4,
            //         std::f32::consts::PI/8.0,
            //     );
            // }
            // // Combine the details with the shapes and spawn them
            // shapes.extend(details);
            // for (size, transform, _) in shapes {
            //     let mesh = Mesh::from(Rectangle::new(size.half_size.x, size.half_size.y));
            //     commands.spawn((
            //         Mesh2d(meshes.add(mesh)),
            //         MeshMaterial2d(materials.add(Color::from(RED_200).with_alpha(0.5))),
            //         transform,
            //     ));
            // }
            // List of possible shapes to draw for the route background
            // let shapes = [
            //     WallShape::Rectangle,
            //     WallShape::Triangle,
            //     WallShape::RegularPolygon,
            // ];
            // // Start with big shapes for the global route
            // let shape_count = 2;
            // for idx in 0..shape_count {
            //     let shape_height = shape_height / shape_count as f32;
            //     let x = self.params.rng.random_range(-self.params.width/2.0..self.params.width/2.0);
            //     let shape_size = Vec2::new(self.params.width, shape_height);
            //     let shape_pos = Vec3::new(x, idx as f32 * shape_height, 0.0);
            //     let shape = shapes.choose(&mut self.params.rng).unwrap();
            //     let mesh = match shape {
            //         WallShape::Rectangle => meshes.add(Rectangle::new(shape_size.x, shape_size.y)),
            //         WallShape::Triangle => meshes.add(Triangle2d::new(
            //             Vec2::Y * shape_size.y,
            //             Vec2::new(-shape_size.x/2.0, -shape_size.y/2.0),
            //             Vec2::new(shape_size.x/2.0, -shape_size.y/2.0),
            //         )),
            //         WallShape::RegularPolygon => meshes.add(RegularPolygon::new(shape_size.x/2.0, 6)),
            //     };
            //     commands.spawn((
            //         Mesh2d(mesh),
            //         MeshMaterial2d(materials.add(Color::from(RED_200).with_alpha(0.3))),
            //         Transform::from_translation(shape_pos).with_rotation(Quat::from_rotation_z(self.params.rng.random_range(0.0..std::f32::consts::TAU))),
            //     ));
            // }

        }
    }

    fn create_shapes_in_block ( 
        &mut self,
        size: Rectangle,
        pos: Vec2,
        count: u32,
        max_angle: f32,
    ) -> Vec<(Mesh, Transform, Rectangle)> {
        let mut shapes = Vec::new();
        for _ in 0..count {
            let shape_width = self.params.rng.random_range(size.half_size.x/4.0..size.half_size.x);
            let shape_height = self.params.rng.random_range(size.half_size.y/4.0..size.half_size.y);
            let x = self.params.rng.random_range(-size.half_size.x/1.2..size.half_size.x/1.2);
            let y = self.params.rng.random_range(-size.half_size.y/1.2..size.half_size.y/1.2);
            let rotation = self.params.rng.random_range(-max_angle..max_angle);
            // let shape_type = self.params.rng.gen_range(0..3);
            // let mesh = match shape_type {
            //     0 => Mesh::from(Rectangle::new(shape_width, shape_height)),
            //     1 => Mesh::from(Triangle2d::new(
            //         Vec2::Y * shape_height,
            //         Vec2::new(-shape_width/2.0, -shape_height/2.0),
            //         Vec2::new(shape_width/2.0, -shape_height/2.0),
            //     )),
            //     _ => Mesh::from(RegularPolygon::new(shape_width.min(shape_height)/2.0, 6)),
            // };
            let mesh = Mesh::from(Rectangle::new(shape_width, shape_height));
            let transform = Transform::from_translation(Vec3::new(pos.x + x, pos.y + y, 0.0)).with_rotation(Quat::from_rotation_z(rotation));
            shapes.push((mesh, transform, Rectangle::new(shape_width, shape_height)));
        }

        shapes
    }
}

pub fn two_hands_on_last_holds(
    mut commands: Commands,
    mut last_hold: Single<(&Transform), With<LastHold>>,
    r_hand: Single<(&Transform, Entity), With<RightHand>>,
    l_hand: Single<(&Transform, Entity), With<LeftHand>>,
    mut r_r_hand_on_hold: ResMut<RightHandOnHold>,
    mut r_l_hand_on_hold: ResMut<LeftHandOnHold>,
    mut event_writer: MessageWriter<GameStateEvent>,
    mut body: Single<&mut Body>,
    mut gizmos: Gizmos,
) {
    let l_hand_on_last_hold = is_hand_on_hold(&l_hand.0, &last_hold, &Vec2::new(20.0, 20.0), &mut gizmos);
    let r_hand_on_last_hold = is_hand_on_hold(&r_hand.0, &last_hold, &Vec2::new(20.0, 20.0), &mut gizmos);

    if l_hand_on_last_hold && r_hand_on_last_hold {
        info!("Player reached top!");
        // make sure both hands are in gravity mode for the fall
        enable_gravity(&mut commands, r_hand.1);
        enable_gravity(&mut commands, l_hand.1);
        body.left_hand_on_hold = false;
        body.right_hand_on_hold = false;

        event_writer.write(
                GameStateEvent::EndOfGame(EndOfGameReason::PlayerReachedTop)
            );
    }
}