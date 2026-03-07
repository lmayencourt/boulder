/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use std::fmt::Debug;

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
};

use bevy_rapier2d::prelude::*;

use crate::hand::{LeftHand, RightHand, HAND_SIZE};

static HOLD_SIZE: Vec2 = Vec2::new(30.0, 10.0);

#[derive(Resource)]
pub struct LeftHandOnHold(pub bool);

#[derive(Resource)]
pub struct RightHandOnHold(pub bool);

#[derive(Component, Clone, Copy)]
pub struct Hold {
    pub position: Vec2,
}

#[derive(Component)]
pub struct LastHold;

impl Hold {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        println!("Spawing hold at {}", self.position);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(HOLD_SIZE.x, HOLD_SIZE.y))),
            MeshMaterial2d(materials.add(Color::from(GRAY_300))),
            Transform::from_xyz(self.position.x, self.position.y, 0.0),
            self,
        ));
    }

    pub fn spawn_last(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        println!("Spawing last hold at {}", self.position);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(HOLD_SIZE.x, HOLD_SIZE.y))),
            MeshMaterial2d(materials.add(Color::from(RED_500))),
            Transform::from_xyz(self.position.x, self.position.y, 0.0),
            LastHold,
            self,
        ));
    }
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

pub fn hand_on_holds_detection(
    mut q_holds: Query<(&mut MeshMaterial2d<ColorMaterial>, &Transform), With<Hold>>,
    q_r_hand: Query<&Transform, With<RightHand>>,
    q_l_hand: Query<&Transform, With<LeftHand>>,
    mut r_r_hand_on_hold: ResMut<RightHandOnHold>,
    mut r_l_hand_on_hold: ResMut<LeftHandOnHold>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    let default_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));

    let mut l_hand_on_any_hold = false;
    let mut r_hand_on_any_hold = false;
    for (mut material, transform) in q_holds.iter_mut() {
        let l_hand_on_hold = is_hand_on_hold(q_l_hand.single().unwrap(), transform, &mut gizmos);
        let r_hand_on_hold = is_hand_on_hold(q_r_hand.single().unwrap(), transform, &mut gizmos);

        if l_hand_on_hold || r_hand_on_hold {
            material.0 = hover_matl.clone();
        } else {
            material.0 = default_matl.clone();
        }

        if l_hand_on_hold {
            l_hand_on_any_hold = true;
        }
        if r_hand_on_hold {
            r_hand_on_any_hold = true;
        }

    }
    r_l_hand_on_hold.0 = l_hand_on_any_hold;
    r_r_hand_on_hold.0 = r_hand_on_any_hold;

    // println!("Left hand on hold: {}, Right hand on hold: {}", r_l_hand_on_hold.0, r_r_hand_on_hold.0);
}

pub fn is_hand_on_hold(
    hand_transform: &Transform,
    hold_transform: &Transform,
    mut gizmos: &mut Gizmos,
) -> bool {
    let hand_box = BoundingCircle::new(hand_transform.translation.truncate(), HAND_SIZE);
    gizmos.circle_2d(hand_transform.translation.truncate(), HAND_SIZE, Color::from(YELLOW_200));

    let bounding_box = Aabb2d::new(hold_transform.translation.truncate(), HOLD_SIZE / 2.0);
    gizmos.rect_2d(hold_transform.translation.truncate(), HOLD_SIZE, Color::from(GRAY_400));
    hand_box.intersects(&bounding_box)
}