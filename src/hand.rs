/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;

// #[derive(Component)]
// pub struct Hand;

#[derive(Component)]
pub struct LeftHand {
    pub is_holding: bool,
}

#[derive(Component)]
pub struct RightHand{
    pub is_holding: bool,
}

#[derive(Component)]
pub struct RightShoulder;

#[derive(Component)]
pub struct LeftShoulder;

// impl Hand {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(BLUE_300))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            LeftHand{
                is_holding: true,
            },
            Pickable::IGNORE
        ));

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(RED_300))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RightHand {
                is_holding: true,
            },
            Pickable::IGNORE
        ));
            // .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            // .observe(update_material_on::<Pointer<Out>>(default_matl.clone()));
    }
// }

pub fn enable_gravity(
    mut commands: &mut Commands,
    hand: Entity,
) {
    commands.entity(hand).remove::<RigidBody>();
    commands.entity(hand).insert(RigidBody::Dynamic);
}

pub fn disable_gravity(
    mut commands: &mut Commands,
    hand: Entity,
) {
    commands.entity(hand).remove::<RigidBody>();
    commands.entity(hand).insert(RigidBody::KinematicVelocityBased);
}
