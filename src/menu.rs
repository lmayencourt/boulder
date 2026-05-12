/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    app::AppExit,
    input::ButtonState,
    input::keyboard::{Key, KeyboardInput},
};

use super::GameState;
use crate::wall::route::SwitchToRoute;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

 // All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            welcome_screen_setup.run_if(in_state(GameState::EnterMenu)),
        );
        // Common systems to all screens that handles buttons behavior
        app.add_systems(
            Update,
            // (menu_action, interactive_button_system).run_if(in_state(GameState::Menu)),
            (interactive_button_system,
            menu_action,
            level_selector_system).run_if(in_state(GameState::Menu)),
        );
        // app.add_systems(Update, level_selector.run_if(in_state(GameState::Menu)));
        // app.add_systems(Update, level_selector);
    }
}

fn welcome_screen_setup(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // This will display its children in a column, from top to bottom
                flex_direction: FlexDirection::Column,
                ..default()
            },
        children![
            (
                Text::new("Boulder"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(px(50)),
                    ..default()
                },
            ),
            (
                Button,
                button_node.clone(),
                BackgroundColor(NORMAL_BUTTON),
                MenuButtonAction::Play,
                children![
                    // (ImageNode::new(right_icon), button_icon_node.clone()),
                    (
                        Text::new("Let's do it!"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),
                ]
            ),
            (
                Text::new("Road to climb:"),
                button_text_font.clone(),
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(px(50)),
                    ..default()
                },
            ), 
            (
                Text::default(),
                LevelSelector {
                    current_level: String::from("First challenge"),
                    ..default()
                },
            )
        ]
    ));

    // Exit the menu enter to guarantee a single spawn
    game_state.set(GameState::Menu);
}

/// This system handles changing all buttons color based on mouse interaction
fn interactive_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

/// Handle button press action
fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
    level_selector: Single<&LevelSelector>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Playing);
                    start_climbing(&level_selector.current_level, &mut game_state, &mut evw_new_route);
                }
            }
        }
    }
}

/// Contains the user defined level name info
#[derive(Component, Default)]
struct LevelSelector {
    pub current_level: String,
    pub user_input: String,
}

/// Read keyboard for the level name and display it inside the related text field
fn level_selector_system(
    mut level_selector: Single<(&mut Text, &mut LevelSelector)>,
    mut evr_keyboard: MessageReader<KeyboardInput>,
    mut next_state: ResMut<NextState<GameState>>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
) {
    level_selector.0.0 = level_selector.1.user_input.clone();

    for ev in evr_keyboard.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Enter => {
                    level_selector.1.current_level = level_selector.1.user_input.clone();
                    level_selector.1.user_input = String::default();
                    start_climbing(&level_selector.1.current_level, &mut next_state, &mut evw_new_route);
            }
            Key::Backspace => {
                level_selector.1.user_input.pop();
            }
            Key::Character(user_input) => {
                debug!("New char {}", &user_input);
                level_selector.1.user_input.push_str(user_input);
            },
            _ => {},
        }
    }
}

/// Start the game by exiting the menu state and switching to the new route
fn start_climbing(
    route_name: &str,
    next_state: &mut ResMut<NextState<GameState>>,
    evw_new_route: &mut MessageWriter<SwitchToRoute>,
) {
    info!("Switching to new level {}", route_name);
    next_state.set(GameState::Playing);
    evw_new_route.write(SwitchToRoute::new(route_name.to_string()));
}