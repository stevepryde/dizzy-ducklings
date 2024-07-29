use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::{
    assets::{HandleMap, SfxKey},
    settings::GameSettings,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(
    trigger: Trigger<PlaySfx>,
    mut commands: Commands,
    settings: Res<GameSettings>,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    if !settings.sound_enabled {
        return;
    }

    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => *key,
        PlaySfx::Jump => SfxKey::Jump,
        PlaySfx::CollectDuckling => SfxKey::CollectDuckling,
    };
    commands.spawn(AudioSourceBundle {
        source: sfx_handles[&sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[allow(dead_code)]
#[derive(Event)]
pub enum PlaySfx {
    Key(SfxKey),
    Jump,
    CollectDuckling,
}
