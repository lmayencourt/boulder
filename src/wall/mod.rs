/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::prelude::*;

pub mod holds;
pub mod route;

use crate::{GameState, GameStateEvent, EndOfGameReason};
pub use holds::*;
use route::{SwitchToRoute, WallPart};


pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SwitchToRoute>();
        app.add_systems(Startup, setup_wall);
        app.add_systems(Update, update_wall);
        app.add_systems(Update, route::two_hands_on_last_holds.run_if(in_state(GameState::Playing)));
        app.add_systems(Update, find_foot_closest_hold);
    }
}

fn setup_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the first hold at 0, 0
    let handle = asset_server.load("embedded://holds/hold_20.png");
    let hold = Hold::new(Vec2::new(0.0, 0.0), Vec2::new(20.0, 20.0));
    hold.spawn(&mut commands, &mut meshes, &mut materials, handle);

    // Spawn a simple wall with holds to climb
    evw_new_route.write(SwitchToRoute { route_name: "First challenge".to_owned() });
}

fn update_wall(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut evr_new_route: MessageReader<SwitchToRoute>,
    mut q_holds: Query<Entity, With<Hold>>,
    mut q_wall: Query<Entity, (With<WallPart>, Without<Hold>)>,
) {
    // Spawn the new wall
    for event in evr_new_route.read() {
        // Clear the current stage
        for hold in q_holds.iter() {
            commands.entity(hold).despawn();
        }
        for wall in q_wall.iter() {
            commands.entity(wall).despawn();
        }

        debug!("Creating a new route {}", event.route_name);
        let mut path = route::Path::new(&event.route_name);
        path.spawn(&mut commands, &mut meshes, &mut materials, &asset_server);
    }
}