use bevy::prelude::*;

use crate::game::spawn::level::EndLevel;

use super::spawn::chick::Chick;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Score>();
    app.observe(on_chick_collected);
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct Score {
    pub score: u32,
    pub chicks_total: u32,
    pub chicks_collected: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            // Prevent game from ending immediately.
            chicks_total: 100,
            chicks_collected: 0,
        }
    }
}

#[derive(Event, Debug)]
pub struct ChickCollected(pub Entity);

fn on_chick_collected(
    trigger: Trigger<ChickCollected>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    chicks: Query<&Chick>,
) {
    if chicks.get(trigger.event().0).is_ok() {
        score.chicks_collected += 1;
        commands.entity(trigger.event().0).despawn_recursive();

        if score.chicks_collected == score.chicks_total {
            commands.trigger(EndLevel);
        }
    }
}
