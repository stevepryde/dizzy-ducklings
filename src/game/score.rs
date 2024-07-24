use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch};

use crate::{game::spawn::level::EndLevel, AppSet};

use super::spawn::{duckling::Duckling, level::SpawnLevel};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Score>();
    app.init_resource::<OverallScore>();
    app.observe(on_duckling_collected);
    app.observe(on_update_score);
    app.observe(on_start_level);
    app.observe(on_end_level);
    app.observe(carry_over_stopwatch);
    app.observe(resume_stopwatch);
    app.add_systems(Update, update_stopwatch.in_set(AppSet::Update));
}

#[derive(Resource, Clone, Debug, Default)]
pub struct OverallScore {
    pub total_seconds: u32,
}

#[derive(Resource, Clone, Debug)]
pub struct Score {
    pub score: u32,
    pub ducklings_total: u32,
    pub ducklings_collected: u32,
    pub stopwatch: Stopwatch,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            // Prevent game from ending immediately.
            ducklings_total: 100,
            ducklings_collected: 0,
            stopwatch: Stopwatch::new(),
        }
    }
}

#[derive(Event, Debug)]
pub struct DucklingCollected(pub Entity);

fn on_duckling_collected(
    trigger: Trigger<DucklingCollected>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    ducklings: Query<&Duckling>,
) {
    if ducklings.get(trigger.event().0).is_ok() {
        score.ducklings_collected += 1;
        commands.entity(trigger.event().0).despawn_recursive();
        commands.trigger(UpdateScore);

        if score.ducklings_collected == score.ducklings_total {
            commands.trigger(EndLevel);
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct ScoreMarker;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct StopwatchMarker;

#[derive(Event, Debug)]
pub struct UpdateScore;

fn on_update_score(
    _trigger: Trigger<UpdateScore>,
    score: Res<Score>,
    mut text_query: Query<(Entity, &ScoreMarker, &mut Text)>,
) {
    for (_, _, mut text) in text_query.iter_mut() {
        text.sections[0].value = format!(
            "Ducklings collected: {} / {}",
            score.ducklings_collected, score.ducklings_total
        );
    }
}

fn update_stopwatch(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut text_query: Query<(Entity, &StopwatchMarker, &mut Text)>,
) {
    score.stopwatch.tick(time.delta());
    for (_, _, mut text) in text_query.iter_mut() {
        text.sections[0].value =
            format!("Seconds elapsed: {}", score.stopwatch.elapsed().as_secs());
    }
}

fn on_start_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.spawn((
        TextBundle {
            text: Text::from_section(
                "Ducklings collected: 0 / 1".to_string(),
                TextStyle::default(),
            ),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        ScoreMarker,
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section("Seconds elapsed: 0".to_string(), TextStyle::default()),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(40.0),
                ..default()
            },
            ..default()
        },
        StopwatchMarker,
    ));

    commands.trigger(UpdateScore);
}

fn on_end_level(
    _trigger: Trigger<EndLevel>,
    mut commands: Commands,
    query: Query<(Entity, &Text)>,
) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn carry_over_stopwatch(
    _trigger: Trigger<EndLevel>,
    score: Res<Score>,
    mut overall_score: ResMut<OverallScore>,
) {
    overall_score.total_seconds += score.stopwatch.elapsed().as_secs() as u32;
}

fn resume_stopwatch(
    _trigger: Trigger<SpawnLevel>,
    mut score: ResMut<Score>,
    overall_score: Res<OverallScore>,
) {
    score
        .stopwatch
        .set_elapsed(Duration::from_secs(overall_score.total_seconds as u64));
}
