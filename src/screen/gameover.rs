//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, score::OverallScore},
    systems::fade::FadeIn,
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), enter_gameover);
    app.add_systems(OnExit(Screen::GameOver), exit_gameover);

    app.add_systems(
        Update,
        handle_gameover_action.run_if(in_state(Screen::GameOver)),
    );
    app.register_type::<GameOverAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum GameOverAction {
    Menu,
}

fn enter_gameover(mut commands: Commands, overall_score: Res<OverallScore>) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::GameOver))
        .with_children(|children| {
            children.header("Congratulations");
            children.label(" ");
            children.label("You completed the game in");
            children.label(" ");
            children.label(" ");
            children.big_label(format!("{:.1}", overall_score.total_seconds));
            children.label(" ");
            children.label(" ");
            children.label("seconds");
            children.label(" ");
            children.label(" ");
            children.button("Continue").insert(GameOverAction::Menu);
        });

    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Credits));
    commands.trigger(FadeIn { duration: 0.5 });
}

fn exit_gameover(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Disable);
}

fn handle_gameover_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&GameOverAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                &GameOverAction::Menu => next_screen.set(Screen::Title),
            }
        }
    }
}
