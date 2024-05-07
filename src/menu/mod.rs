mod display_menu;
mod main_menu;
mod pause_menu;
mod settings_menu;
mod volume_menu;

use crate::ecs::*;
use bevy::prelude::*;

use self::main_menu::MainMenuState;

const BACKGROUND_COLOR: Color = Color::BLACK;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

fn button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: button_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

fn button_style() -> Style {
    Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn button_text_style() -> TextStyle {
    TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    }
}

fn button_text(text: &str) -> TextBundle {
    TextBundle::from_section(text, button_text_style())
}

fn main_panel_center() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
}

fn menu_vertical() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::CRIMSON.into(),
        ..default()
    }
}

fn menu_title(title: &str) -> TextBundle {
    TextBundle::from_section(
        title,
        TextStyle {
            font_size: 80.0,
            color: TEXT_COLOR,
            ..default()
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(50.0)),
        ..default()
    })
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    PlayGame,
    Settings,
    SettingsSound,
    SettingsDisplay,
    BackToMainMenu,
    BackToSettings,
    ExitApplication,
    QuitGame,
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        main_menu::plugin,
        pause_menu::plugin,
        display_menu::DisplaySettingsPlugin(MainMenuState::SettingsDisplay),
        volume_menu::SoundSettingsPlugin(MainMenuState::SettingsSound),
        settings_menu::SettingsPlugin(MainMenuState::Settings),
        display_menu::DisplaySettingsPlugin(PauseMenuState::SettingsDisplay),
        volume_menu::SoundSettingsPlugin(PauseMenuState::SettingsSound),
        settings_menu::SettingsPlugin(PauseMenuState::Settings),
    ))
    // Common systems to all screens that handles buttons behavior
    .add_systems(
        Update,
        // (button_system).run_if(in_state(GameState::Menu).or_else(in_state(InGameState::Pause))),
        (button_system).run_if(in_menu),
    );
}

// run condition
fn in_menu(game_state: Res<State<GameState>>, in_game_state: Res<State<InGameState>>) -> bool {
    *game_state == GameState::Menu || *in_game_state == InGameState::Pause
}

// State used for the main menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PauseMenuState {
    Main,
    Settings,
    SettingsSound,
    SettingsDisplay,
    #[default]
    Disabled,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the button as the one currently selected
fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            if let Ok((previous_button, mut previous_color)) = selected_query.get_single_mut() {
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
            }
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}
