use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::game::{
    assets::{HandleMap, SoundtrackKey},
    settings::{GameSettings, ToggleSound},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
    app.observe(sound_toggle);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    if !settings.sound_enabled {
        return;
    }

    let soundtrack_key = match trigger.event() {
        PlaySoundtrack::Key(key) => *key,
        PlaySoundtrack::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&soundtrack_key].clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.5),
                ..default()
            },
        },
        IsSoundtrack,
    ));
}

fn sound_toggle(
    _trigger: Trigger<ToggleSound>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    if !settings.sound_enabled {
        for entity in &soundtrack_query {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Key(SoundtrackKey),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;
