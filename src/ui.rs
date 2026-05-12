/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    input::ButtonState,
    input::keyboard::{Key, KeyboardInput},
    color::palettes::tailwind::*,
};

use crate::GameState;
use crate::player::Body;
use crate::wall::route::SwitchToRoute;

pub struct UiPlugin;

#[derive(Resource)]
pub struct CurrentPlayerHeight(f32);

#[derive(Component)]
struct CurrentHeightText;

#[derive(Default, PartialEq)]
enum LevelEditingMode {
    #[default]
    Disable,
    Enable,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentPlayerHeight(0.0));
        app.add_systems(Startup,setup_ui);
        app.add_systems(Update, update_current_height);
    }
}

fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn((Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
        CurrentHeightText,
    ));

    commands.spawn((Text::default(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn update_current_height(
    mut text: Single<&mut Text, With<CurrentHeightText>>,
    body: Single<&mut Transform, With<Body>>,
) {
    text.0 = format!("Current height: {:.1}", body.translation.y);
}