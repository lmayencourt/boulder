
use std::fmt::Debug;

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
};

use bevy_rapier2d::prelude::*;

use crate::hand::{LeftHand, RightHand};

#[derive(Resource)]
pub struct LeftHandOnHold(pub bool);

#[derive(Resource)]
pub struct RightHandOnHold(pub bool);

#[derive(Component)]
pub struct Hold {
    pub position: Vec2,
}

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
        println!("Spawning hold at position: {:?}", self.position);

        let default_matl = materials.add(Color::from(GRAY_300));
        let hover_matl = materials.add(Color::from(CYAN_100));

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
            MeshMaterial2d(default_matl.clone()),
            Transform::from_xyz(self.position.x, self.position.y, 0.0),
            self,
        ));
            // .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            // .observe(update_material_on::<Pointer<Out>>(default_matl.clone()));
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
) {
    let default_matl = materials.add(Color::from(GRAY_300));
    let hover_matl = materials.add(Color::from(CYAN_300));

    for (mut material, transform) in q_holds.iter_mut() {
        let l_hand_on_hold = is_hand_on_hold(q_l_hand.single().unwrap(), transform);
        let r_hand_on_hold = is_hand_on_hold(q_r_hand.single().unwrap(), transform);

        if l_hand_on_hold || r_hand_on_hold {
            material.0 = hover_matl.clone();
        } else {
            material.0 = default_matl.clone();
        }

        if l_hand_on_hold {
            r_l_hand_on_hold.0 = true;
        }
        if r_hand_on_hold {
            r_r_hand_on_hold.0 = true;
        }

    }
}

fn is_hand_on_hold(
    hand_transform: &Transform,
    hold_transform: &Transform,
) -> bool {
    let hand_box = BoundingCircle::new(hand_transform.translation.truncate(), 10.0);

    let bounding_box = Aabb2d::new(hold_transform.translation.truncate(), Vec2::new(15.0, 15.0));
    hand_box.intersects(&bounding_box)
}