use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub fn button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: button_style(),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }
}

pub fn button_style() -> Style {
    Style {
        width: Val::Px(170.0),
        height: Val::Px(55.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn button_text_style() -> TextStyle {
    TextStyle {
        font_size: 32.0,
        color: TEXT_COLOR,
        ..default()
    }
}

pub fn button_text(text: &str) -> TextBundle {
    TextBundle::from_section(text, button_text_style())
}

pub fn centered_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

pub fn centered() -> NodeBundle {
    NodeBundle {
        style: centered_style(),
        ..default()
    }
}

pub fn menu() -> NodeBundle {
    NodeBundle {
        background_color: Color::CRIMSON.into(),
        ..vsizer()
    }
}

pub fn menu_title(title: &str) -> TextBundle {
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

pub fn hsizer() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}

pub fn vsizer() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }
}
