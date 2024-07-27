use bevy::prelude::*;

use crate::AppSet;

pub fn plugin(app: &mut App) {
    app.add_event::<FadeOut>();
    app.add_event::<FadeIn>();
    app.add_event::<FadeCompleted>();
    app.add_systems(Startup, setup)
        .add_systems(Update, fade_system.in_set(AppSet::Update));

    app.observe(fade_in);
    app.observe(fade_out);
}

#[derive(Component)]
struct FadeOverlay;

fn setup(mut commands: Commands) {
    commands.spawn((
        Name::new("Fade Overlay"),
        NodeBundle {
            style: Style {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: Color::BLACK.with_alpha(0.0).into(),
            ..Default::default()
        },
        FadeOverlay,
    ));
}

#[derive(Debug)]
pub enum FaderType {
    FadeIn,
    FadeOut,
}

#[derive(Component)]
pub struct Fader {
    fade_type: FaderType,
    duration: f32,
    elapsed: f32,
}

fn fade_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fader, &mut BackgroundColor), With<FadeOverlay>>,
    mut event_writer: EventWriter<FadeCompleted>,
) {
    for (entity, mut fader, mut color) in query.iter_mut() {
        fader.elapsed += time.delta_seconds();
        if fader.elapsed >= fader.duration {
            fader.elapsed = fader.duration;
        }
        let alpha = match fader.fade_type {
            FaderType::FadeIn => 1.0 - fader.elapsed / fader.duration,
            FaderType::FadeOut => fader.elapsed / fader.duration,
        };
        color.0.set_alpha(alpha);

        // If we reached the end, remove the fader
        if fader.elapsed >= fader.duration {
            commands.entity(entity).remove::<Fader>();
            event_writer.send(FadeCompleted);
            log::warn!("Fade Completed");
        }
    }
}

#[derive(Event, Debug)]
pub struct FadeOut {
    pub duration: f32,
}

#[derive(Event, Debug)]
pub struct FadeIn {
    pub duration: f32,
}

#[derive(Event, Debug)]
pub struct FadeCompleted;

fn fade_out(
    trigger: Trigger<FadeOut>,
    mut commands: Commands,
    query: Query<(Entity, &BackgroundColor), With<FadeOverlay>>,
) {
    let (entity, color) = query.single();
    let percent = color.0.alpha() / 1.0;
    let duration = trigger.event().duration;

    commands.entity(entity).insert(Fader {
        fade_type: FaderType::FadeOut,
        duration,
        elapsed: duration * percent,
    });
}

fn fade_in(
    trigger: Trigger<FadeIn>,
    mut commands: Commands,
    query: Query<(Entity, &BackgroundColor), With<FadeOverlay>>,
) {
    let (entity, color) = query.single();
    let percent = 1.0 - (color.0.alpha() / 1.0);
    let duration = trigger.event().duration;

    commands.entity(entity).insert(Fader {
        fade_type: FaderType::FadeIn,
        duration,
        elapsed: duration * percent,
    });
}
