/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use rand::prelude::*;
use rand::rngs::ChaCha8Rng;

use crate::hand::{LeftHand, RightHand, HAND_SIZE};
use super::holds::*;

static ROUTE_WIDTH_MIN: f32 = 150.0;
static ROUTE_WIDTH_MAX: f32 = ROUTE_WIDTH_MIN * 2.0;
static ROUTE_HEIGHT_MIN: f32 = 200.0;
static ROUTE_HEIGHT_MAX: f32 = ROUTE_HEIGHT_MIN * 10.0;

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

#[derive(Debug)]
struct RouteParams {
    height: f32,
    width: f32,
}

impl RouteParams {
    pub fn new(seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let height = rng.random_range(ROUTE_HEIGHT_MIN..ROUTE_HEIGHT_MAX);
        let width = rng.random_range(ROUTE_WIDTH_MIN..ROUTE_WIDTH_MAX);

        Self {
            height,
            width
        }
    }
}

impl Path {
    pub fn new(name: &str) -> Self {
        let seed = Self::name_to_seed(&name);
        info!("Seed for {} is {}", name, seed);
        let params = RouteParams::new(seed);
        info!("Creating a new route {:?}", params);

        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let vertical_spacing = 50.0;
        let number_of_holds = params.height/vertical_spacing;

        let mut holds = Vec::new();

        for i in 0..number_of_holds as i32 {
            let x = rng.random_range(-params.width/2.0..params.width/2.0);
            let y = i as f32 * vertical_spacing;
            holds.push(Hold::new(Vec2::new(x, y)));
        }

        Self { holds }
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