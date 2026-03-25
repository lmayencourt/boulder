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
        app.add_systems(Update, find_foot_closest_hold);
    }
}

fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
) {
    // Spawn the first hold at 0, 0
    let hold = Hold::new(Vec2::new(0.0, 0.0));
    hold.spawn(&mut commands, &mut meshes, &mut materials);

    // Spawn a simple wall with holds to climb
    evw_new_route.write(SwitchToRoute { route_name: "First challenge".to_owned() });
}

fn update_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: MessageReader<PlayerReachedLastHold>,
    mut evr_new_route: MessageReader<SwitchToRoute>,
    mut q_holds: Query<Entity, With<Hold>>,
) {
    // Spawn the new wall
    for event in evr_new_route.read() {
        // Clear the current stage
        for hold in q_holds.iter() {
            commands.entity(hold).despawn();
        }

        println!("Creating a new route {}", event.route_name);
        let path = route::Path::new(&event.route_name);
        path.spawn(&mut commands, &mut meshes, &mut materials);
    }
}