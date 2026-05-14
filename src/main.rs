/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    window::PrimaryWindow,
    color::palettes::tailwind::*,
};
use bevy_rapier2d::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use rand::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod camera;
mod corbusier_colors;
mod menu;
mod mesh_drawing;
mod mouse;
mod player;
mod ui;
mod wall;

use camera::*;
use player::*;
use menu::MenuPlugin;
use mouse::*;
use ui::UiPlugin;
use route::PlayerReachedLastHold;
use wall::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    EnterMenu,
    Menu,
    Playing,
    LevelSelection,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .add_plugins((EmbeddedAssetPlugin::default()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ShapePlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(MousePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WallPlugin)
        .init_state::<GameState>()
        .insert_resource(wall::holds::LeftHandOnHold(false))
        .insert_resource(wall::holds::RightHandOnHold(false))
        .insert_resource(wall::holds::LeftFootClosestHold(Vec2::default()))
        .insert_resource(wall::holds::RightFootClosestHold(Vec2::default()))
        .add_systems(Startup, setup_system)
        .add_systems(Update, game_state_system)
        .add_systems(Update, camera::follow_player.run_if(in_state(GameState::Playing)))
        .add_systems(Update, camera::zoom)
        .add_systems(Update, wall::holds::hand_on_holds_detection)
        .add_systems(Update, toogle_gizmos_visibility)
        .run();
}

#[derive(Component)]
pub struct LastHold {
    fixed_position: Option<Vec2>,
}

#[derive(Component)]
pub struct FirstHold;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config_store: ResMut<GizmoConfigStore>,
    asset_server: Res<AssetServer>,
) {
    // Spawn a 2D camera
    commands.spawn((Camera2d, MainCamera, SmoothFollower::default()));

    // By default, disable the debug gizmos
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.enabled = false;

    // Spawn the body
    Body::spawn(&mut commands, &mut meshes, &mut materials, asset_server);
}

fn toogle_gizmos_visibility(
    mut config_store: ResMut<GizmoConfigStore>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    if keys.just_pressed(KeyCode::Digit1) {
        config.enabled = !config.enabled;
    }
}

fn game_state_system(
    mut game_state: ResMut<NextState<GameState>>,
    mut last_hold_reached: MessageReader<PlayerReachedLastHold>,
) {
    if !last_hold_reached.is_empty() {
        // Player reached the end the wall, launch the menu
        game_state.set(GameState::EnterMenu);
        last_hold_reached.clear();
    }
}
