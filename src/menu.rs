/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    app::AppExit,
    input::ButtonState,
    input::keyboard::{Key, KeyboardInput},
};

use super::{GameState, GameStateEvent, EndOfGameReason};
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
    ToMainMenu,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

/// Newtype to use a `Timer` as a count down before redrawing the wall after user input
#[derive(Resource, Deref, DerefMut)]
struct RefreshWall(Timer);

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            LevelSelector {
                    current_level: String::from("First challenge"),
                    ..default()
                });
        app.insert_resource(RefreshWall(Timer::from_seconds(1.0, TimerMode::Once)));
        app.add_systems(OnEnter(GameState::Menu), welcome_screen_setup);
        app.add_systems(OnEnter(GameState::EndOfGame(EndOfGameReason::PlayerFelt)), player_fell_menu);
        app.add_systems(OnEnter(GameState::EndOfGame(EndOfGameReason::PlayerReachedTop)), player_reached_top_menu);
        app.add_systems(
            Update,
            (interactive_button_system,
            menu_action,
            level_selector_system)
                // .run_if(in_state(GameState::Menu)),
        );
    }
}

fn welcome_screen_setup(
    mut commands: Commands,
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
                LevelSelectorText,
            ),
            (
                Button,
                button_node.clone(),
                BackgroundColor(NORMAL_BUTTON),
                MenuButtonAction::Quit,
                children![
                    (
                        Text::new("Exit"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),
                ]
            ),
        ]
    ));
}

fn player_fell_menu(
    mut commands: Commands,
) {
    end_of_game_menu_setup(&mut commands, "You fell...", GameState::EndOfGame(crate::EndOfGameReason::PlayerFelt));
}

fn player_reached_top_menu(
    mut commands: Commands,
) {
    end_of_game_menu_setup(&mut commands, "Well done!", GameState::EndOfGame(crate::EndOfGameReason::PlayerReachedTop));
}

fn end_of_game_menu_setup(
    commands: &mut Commands,
    end_reason: &str,
    despawn_on_exit_state: GameState,
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
        DespawnOnExit(despawn_on_exit_state),
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
                Text::new(end_reason),
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
                    (
                        Text::new("Let's retry it!"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),
                ]
            ),
            (
                Button,
                button_node.clone(),
                BackgroundColor(NORMAL_BUTTON),
                MenuButtonAction::ToMainMenu,
                children![
                    (
                        Text::new("Road selection"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),
                ]
            ),
        ]
    ));
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
    mut game_event: MessageWriter<GameStateEvent>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
    mut level_selector: ResMut<LevelSelector>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Button pressed");
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                },
                MenuButtonAction::Play => {
                    start_climbing(&mut level_selector, &mut game_event, &mut evw_new_route);
                },
                MenuButtonAction::ToMainMenu => {
                    info!("Back to main menu pressed");
                    game_event.write(GameStateEvent::BackToMainMenu);
                },
            }
        }
    }
}

/// Contains the user defined level name info
#[derive(Resource, Default)]
pub struct LevelSelector {
    pub current_level: String,
}

/// Marker component to retrieve the Text field that display the level name 
#[derive(Component)]
struct LevelSelectorText;

/// Read keyboard for the level name and display it inside the related text field
fn level_selector_system(
    mut level_selector: ResMut<LevelSelector>,
    mut level_selector_text: Single<&mut Text, With<LevelSelectorText>>,
    mut evr_keyboard: MessageReader<KeyboardInput>,
    mut game_event: MessageWriter<GameStateEvent>,
    mut evw_new_route: MessageWriter<SwitchToRoute>,
    time: Res<Time>,
    mut timer: ResMut<RefreshWall>,
) {
    level_selector_text.0 = level_selector.current_level.clone();

    for ev in evr_keyboard.read() {
        if ev.state == ButtonState::Released {
            continue;
        }
        match &ev.logical_key {
            Key::Enter => {
                    start_climbing(&mut level_selector, &mut game_event, &mut evw_new_route);
            }
            Key::Backspace => {
                level_selector.current_level.pop();
            }
            Key::Space => {
                level_selector.current_level.push_str(" ");
            }
            Key::Character(user_input) => {
                debug!("New char {}", &user_input);
                level_selector.current_level.push_str(user_input);
                *timer = RefreshWall(Timer::from_seconds(0.5, TimerMode::Once));
            },
            _ => {},
        }
    }

    if timer.tick(time.delta()).just_finished() {
        evw_new_route.write(SwitchToRoute::new(level_selector.current_level.clone()));
    }
}

/// Start the game by exiting the menu state and switching to the new route
fn start_climbing(
    level_selector: &mut LevelSelector,
    game_event: &mut MessageWriter<GameStateEvent>,
    evw_new_route: &mut MessageWriter<SwitchToRoute>,
) {
    info!("Switching to new level {}", level_selector.current_level);
    evw_new_route.write(SwitchToRoute::new(level_selector.current_level.clone()));
    game_event.write(GameStateEvent::StartGame);
}