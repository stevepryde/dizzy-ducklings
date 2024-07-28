//! The title screen that appears when the game starts.

use bevy::prelude::*;

#[cfg(feature = "dev")]
use bevy::dev_tools::states::log_transitions;

use super::Screen;
use crate::{
    game::settings::{GameSettings, ToggleSound},
    systems::fade::{FadeCompleted, FadeIn, FadeOut},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<TitleState>();
    app.add_systems(OnEnter(Screen::Title), fade_on_enter);
    app.add_systems(OnEnter(TitleState::EnterFadingIn), enter_title);
    app.add_systems(Update, on_fade_completed.run_if(in_state(Screen::Title)));

    app.register_type::<TitleAction>();
    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));

    #[cfg(feature = "dev")]
    app.add_systems(Update, log_transitions::<TitleState>);

    app.observe(toggle_sound);
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum TitleState {
    #[default]
    Inactive,
    EnterFadingOut,
    EnterFadingIn,
    Active,
    ActionPlayFadingOut,
    ActionExitFadingOut,
}

fn fade_on_enter(mut commands: Commands, mut next_state: ResMut<NextState<TitleState>>) {
    commands.trigger(FadeOut { duration: 0.5 });
    next_state.set(TitleState::EnterFadingOut);
}

fn on_fade_completed(
    mut events: EventReader<FadeCompleted>,
    mut commands: Commands,
    state: Res<State<TitleState>>,
    mut next_state: ResMut<NextState<TitleState>>,
    mut next_screen: ResMut<NextState<Screen>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for _ in events.read() {
        match state.get() {
            TitleState::EnterFadingOut => {
                commands.trigger(FadeIn { duration: 0.5 });
                next_state.set(TitleState::EnterFadingIn);
            }
            TitleState::EnterFadingIn => {
                next_state.set(TitleState::Active);
            }
            TitleState::ActionPlayFadingOut => {
                next_screen.set(Screen::Playing);
                next_state.set(TitleState::Inactive);
            }
            TitleState::ActionExitFadingOut => {
                app_exit.send(AppExit::Success);
            }
            _ => {}
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    SoundToggle,
    Credits,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.title("DIZZY DUCKLINGS");
            children.button("Play").insert(TitleAction::Play);
            children
                .button("Sound: ON")
                .insert((TitleAction::SoundToggle, SoundButton));
            children.button("Credits").insert(TitleAction::Credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut commands: Commands,
    mut settings: ResMut<GameSettings>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut next_state: ResMut<NextState<TitleState>>,
    mut button_query: InteractionQuery<&TitleAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => {
                    next_state.set(TitleState::ActionPlayFadingOut);
                    commands.trigger(FadeOut { duration: 0.5 });
                }
                TitleAction::SoundToggle => {
                    settings.sound_enabled = !settings.sound_enabled;
                    commands.trigger(ToggleSound);
                }
                TitleAction::Credits => next_screen.set(Screen::Credits),

                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    next_state.set(TitleState::ActionExitFadingOut);
                    commands.trigger(FadeOut { duration: 0.5 });
                }
            }
        }
    }
}

#[derive(Component)]
struct SoundButton;

fn toggle_sound(
    _trigger: Trigger<ToggleSound>,
    settings: Res<GameSettings>,
    mut query: Query<(&Children, &SoundButton)>,
    mut text_query: Query<&mut Text>,
) {
    for (children, _) in &mut query.iter_mut() {
        for child in children {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].value = if settings.sound_enabled {
                    "Sound: ON".to_string()
                } else {
                    "Sound: OFF".to_string()
                };
            }
        }
    }
}
