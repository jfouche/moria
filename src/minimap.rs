use bevy::prelude::*;

#[derive(Component)]
pub struct Minimap;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum MinimapState {
    Hide,
    Show,
}

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MinimapState::Hide)
            .add_systems(Update, toggle_minimap)
            .add_systems(OnEnter(MinimapState::Show), spawn_minimap)
            .add_systems(OnExit(MinimapState::Show), delete_minimap);
    }
}

fn toggle_minimap(
    state: Res<State<MinimapState>>,
    mut next_state: ResMut<NextState<MinimapState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        match state.get() {
            MinimapState::Hide => next_state.set(MinimapState::Show),
            MinimapState::Show => next_state.set(MinimapState::Hide),
        }
    }
}

fn spawn_minimap(mut commands: Commands) {
    commands.spawn((
        Minimap,
        Name::new("Minimap"),
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(90.0),
                height: Val::Percent(80.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: Color::rgba(0.9, 0.9, 0.9, 0.3).into(),
            ..Default::default()
        },
    ));
}

fn delete_minimap(mut commands: Commands, minimap: Query<Entity, With<Minimap>>) {
    for entity in minimap.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
