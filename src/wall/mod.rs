/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::prelude::*;

pub mod holds;
pub mod route;

pub use holds::*;
use route::{PlayerReachedLastHold, SwitchToRoute};


pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PlayerReachedLastHold>();
        app.add_message::<SwitchToRoute>();
        app.add_systems(Startup, setup_wall);
        app.add_systems(Update, update_wall);
        app.add_systems(Update, route::two_hands_on_last_holds);
    }
}

fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the first hold at 0, 0
    let hold = Hold::new(Vec2::new(0.0, 0.0));
    hold.spawn(&mut commands, &mut meshes, &mut materials);

    // Spawn a simple wall with holds to climb
    let path = route::Path::new(200.0);
    path.spawn(&mut commands, &mut meshes, &mut materials);
}

fn update_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: MessageReader<PlayerReachedLastHold>,
    mut evr_new_route: MessageReader<SwitchToRoute>,
) {
    // If player reached the last hold of the current wall, spawn the next section
    for event in evr_new_route.read() {
        println!("Creating a new route {}", event.route_name);
    }
}