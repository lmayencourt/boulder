/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use rand::prelude::*;

use crate::hand::{LeftHand, RightHand, HAND_SIZE};
use super::holds::*;

static WALL_WIDTH: f32 = 150.0;

#[derive(Message, Default)]
pub struct PlayerReachedLastHold;

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
}

impl Path {
    pub fn new(height: f32) -> Self {
        let vertical_spacing = 50.0;
        let number_of_holds = height/vertical_spacing;

        let mut holds = Vec::new();

        for i in 0..number_of_holds as i32 {
            let x = rand::rng().random_range(-WALL_WIDTH/2.0..WALL_WIDTH/2.0);
            let y = i as f32 * vertical_spacing;
            holds.push(Hold::new(Vec2::new(x, y)));
        }

        Self { holds }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        for hold in self.holds.iter().take(self.holds.len().saturating_sub(1)) {
            hold.spawn(commands, meshes, materials);
        }
        // Spawn last hold
        if let Some(last_hold) = self.holds.last() {
            last_hold.spawn_last(commands, meshes, materials);
        }
    }
}

pub fn two_hands_on_last_holds(
    mut last_hold: Single<(&Transform), With<LastHold>>,
    r_hand: Single<&Transform, With<RightHand>>,
    l_hand: Single<&Transform, With<LeftHand>>,
    mut r_r_hand_on_hold: ResMut<RightHandOnHold>,
    mut r_l_hand_on_hold: ResMut<LeftHandOnHold>,
    mut event_writer: EventWriter<PlayerReachedLastHold>,
    mut gizmos: Gizmos,
) {
    let l_hand_on_last_hold = is_hand_on_hold(&l_hand, &last_hold, &mut gizmos);
    let r_hand_on_last_hold = is_hand_on_hold(&r_hand, &last_hold, &mut gizmos);

    if l_hand_on_last_hold && r_hand_on_last_hold {
        println!("Player reached top!");
        event_writer.write_default();
    }
}