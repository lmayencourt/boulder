/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};

use crate::player::Body;

pub struct UiPlugin;

#[derive(Resource)]
pub struct CurrentPlayerHeight(f32);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentPlayerHeight(0.0));
        app.add_systems(Startup,setup_ui);
        app.add_systems(Update, update_ui);
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
        })
    );
}

fn update_ui(
    mut text: Single<&mut Text>,
    body: Single<&mut Transform, With<Body>>,
) {
    text.0 = format!("Current height: {:.1}", body.translation.y);
}