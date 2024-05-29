use bevy::prelude::*;

use super::fullscreen_style;

#[derive(Bundle)]
pub struct Fader {
    name: Name,
    fade: Fade,
    node: NodeBundle,
    timer: FadeTimer,
}

impl Fader {
    pub fn new(from: Color, to: Color, duration_secs: f32) -> Self {
        Fader {
            name: Name::new("Fade"),
            fade: Fade { from, to },
            node: NodeBundle {
                style: fullscreen_style(),
                ..Default::default()
            },
            timer: FadeTimer(Timer::from_seconds(duration_secs, TimerMode::Once)),
        }
    }
}

#[derive(Component)]
struct Fade {
    from: Color,
    to: Color,
}

#[derive(Component, Deref, DerefMut)]
struct FadeTimer(Timer);

pub fn plugin(app: &mut App) {
    app.add_systems(Update, fade);
}

fn fade(
    mut commands: Commands,
    mut fades: Query<(Entity, &Fade, &mut FadeTimer, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (entity, fade, mut timer, mut bgcolor) in &mut fades {
        timer.tick(time.delta());
        let percent = timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32();
        let from = fade.from.as_rgba_f32();
        let to = fade.to.as_rgba_f32();
        let c = |i| from[i] + (to[i] - from[i]) * percent;
        let r = c(0);
        let g = c(1);
        let b = c(2);
        let a = c(3);
        let color = Color::rgba(r, g, b, a);
        bgcolor.0 = color;

        if timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
