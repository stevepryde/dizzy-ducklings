use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameSettings {
    pub sound_enabled: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
        }
    }
}

#[derive(Event, Debug)]
pub struct ToggleSound;
