/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::prelude::*;

use crate::player::Body;
use crate::menu::LevelSelector;

pub struct UiPlugin;

#[derive(Resource)]
pub struct CurrentPlayerHeight(f32);

#[derive(Component)]
struct CurrentHeightText;

/// Marker component to retrieve the current route text field
#[derive(Component)]
struct CurrentRouteName;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentPlayerHeight(0.0));
        app.add_systems(Startup,setup_ui);
        app.add_systems(Update, (update_current_height,
            display_route_name));
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
        CurrentRouteName,
    ));
}

fn update_current_height(
    mut text: Single<&mut Text, With<CurrentHeightText>>,
    body: Single<&mut Transform, With<Body>>,
) {
    text.0 = format!("Current height: {:.1}", body.translation.y);
}

fn display_route_name(
    mut text: Single<&mut Text, With<CurrentRouteName>>,
    level_selector: Res<LevelSelector>,
) {
    text.0 = level_selector.current_level.clone();
}