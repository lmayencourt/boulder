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

#[derive(Component, Default)]
struct LevelSelector {
    pub current_level: String,
    pub user_input: String,
    pub current_edit_mode: LevelEditingMode,
}

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
        app.add_systems(Update, level_selector);
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
        LevelSelector {
            current_level: String::from("0"),
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

fn level_selector(
    mut level_selector: Single<(&mut Text, &mut LevelSelector)>,
    mut evr_keyboard: MessageReader<KeyboardInput>,
    mut next_state: ResMut<NextState<GameState>>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
) {
    level_selector.0.0 = format!("Current level: {}\nNext level: {}\nPress Enter to change level", level_selector.1.current_level, level_selector.1.user_input);

    for ev in evr_keyboard.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Enter => {
                match level_selector.1.current_edit_mode {
                    LevelEditingMode::Disable => {
                        level_selector.1.current_edit_mode = LevelEditingMode::Enable;
                        next_state.set(GameState::LevelSelection);
                    },
                    LevelEditingMode::Enable => {
                        level_selector.1.current_edit_mode = LevelEditingMode::Disable;
                        level_selector.1.current_level = level_selector.1.user_input.clone();
                        level_selector.1.user_input = String::default();
                        next_state.set(GameState::Playing);
                        evw_new_route.write(SwitchToRoute::new(level_selector.1.current_level.clone()));
                    }
                }
            }
            Key::Character(user_input) => {
                if user_input.chars().any(|c| c.is_control()) {
                    continue;
                }
                if level_selector.1.current_edit_mode == LevelEditingMode::Enable {
                    level_selector.1.user_input.push_str(&user_input);
                }
            },
            _ => {},
        }
    }
}