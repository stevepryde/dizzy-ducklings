//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::{
    game::{
        frames::ResetFrameCounter,
        score::{OverallScore, Score, UpdateScore},
    },
    screen::Screen,
    systems::fade::{FadeCompleted, FadeIn, FadeOut},
};

use super::{
    duckling::{Duckling, SpawnDuckling},
    player::{Player, SpawnPlayer},
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<LevelState>();
    app.observe(start_new_game);
    app.observe(spawn_level);
    app.observe(cleanup_level);
    app.observe(on_end_level);
    app.observe(on_game_completed);
    app.add_systems(Update, on_level_added.run_if(in_state(Screen::Playing)));
    app.add_systems(Update, on_fade_completed.run_if(in_state(Screen::Playing)));
    app.add_systems(OnExit(Screen::Playing), exit_playing);
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum LevelState {
    #[default]
    Inactive,
    EndLevelFadeOut,
    StartLevelFadeIn,
    Active,
    CompletedFadeOut,
}

fn on_fade_completed(
    mut events: EventReader<FadeCompleted>,
    mut commands: Commands,
    mut level: ResMut<CurrentLevel>,
    levels: Res<Levels>,
    state: Res<State<LevelState>>,
    mut next_state: ResMut<NextState<LevelState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for _ in events.read() {
        match state.get() {
            LevelState::EndLevelFadeOut => {
                commands.trigger(CleanupLevel);

                level.0 += 1;
                match levels.current(*level) {
                    Some(_) => {
                        commands.trigger(SpawnLevel);
                        commands.trigger(FadeIn { duration: 0.5 });
                        next_state.set(LevelState::StartLevelFadeIn);
                    }
                    None => {
                        // TODO: finish the game!
                        commands.trigger(GameCompleted);
                        return;
                    }
                }
            }
            LevelState::StartLevelFadeIn => {
                next_state.set(LevelState::Active);
            }
            LevelState::CompletedFadeOut => {
                next_screen.set(Screen::GameOver);
                next_state.set(LevelState::Inactive);
            }
            _ => {}
        }
    }
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct CurrentLevel(i32);

#[derive(Event, Debug)]
pub struct StartNewGame;

fn start_new_game(_trigger: Trigger<StartNewGame>, mut commands: Commands) {
    commands.init_resource::<OverallScore>();
    commands.init_resource::<Levels>();
    commands.insert_resource(CurrentLevel(0));
    commands.trigger(FadeOut { duration: 0.5 });
    commands.trigger(SpawnLevel);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component, Debug)]
pub struct LevelLoaded;

#[derive(Component)]
pub struct LevelMarker;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    current_level: Res<CurrentLevel>,
    levels: Res<Levels>,
    mut score: ResMut<Score>,
) {
    let level = levels.current(*current_level).unwrap();
    let mapx = level.size.x as f32 * 16. - 16.;
    let mapy = level.size.y as f32 * 16. - 16.;
    let map_handle: Handle<TiledMap> = asset_server.load(&level.map);

    // Set up score details.
    score.score = 0;
    score.ducklings_total = level.duckling_spawn_points.len() as u32;
    score.ducklings_collected = 0;
    score.stopwatch.reset();

    commands
        .spawn((
            Name::new("Level"),
            LevelMarker,
            SpatialBundle {
                transform: Transform::from_xyz(0., 0., 0.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TiledMapBundle {
                tiled_map: map_handle,
                tiled_settings: TiledMapSettings {
                    collision_layer_names: ObjectNames::None,
                    collision_object_names: ObjectNames::All,
                },
                transform: Transform::from_xyz(-mapx, -mapy, 0.0),
                ..Default::default()
            });

            // Set player spawn point.
            parent.spawn((
                PlayerSpawnPoint,
                SpatialBundle {
                    transform: Transform::from_xyz(
                        level.start_tile.x as f32 * 32.,
                        level.start_tile.y as f32 * 32.,
                        0.,
                    ),
                    ..default()
                },
            ));

            for p in &level.duckling_spawn_points {
                parent.spawn((
                    DucklingSpawnPoint,
                    SpatialBundle {
                        transform: Transform::from_xyz(p.x as f32 * 32., p.y as f32 * 32., 0.),
                        ..default()
                    },
                ));
            }
        });

    log::warn!("SPAWNED LEVEL");

    // commands.trigger(ResetFrameCounter);
    commands.trigger(UpdateScore);
}

fn on_level_added(
    mut events: EventReader<AssetEvent<TiledMap>>,
    mut commands: Commands,
    map: Query<(Entity, &Handle<TiledMap>), Without<LevelLoaded>>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            for (entity, handle) in map.iter() {
                if handle.id() == *id {
                    log::warn!("MAP LOADED");
                    commands.entity(entity).insert(LevelLoaded);
                    commands.trigger(SpawnPlayer);
                    commands.trigger(SpawnDuckling);
                    commands.trigger(ResetFrameCounter);
                    return;
                }
            }
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct PlayerSpawnPoint;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct DucklingSpawnPoint;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct LevelFinishPoint;

#[derive(Event, Debug)]
pub struct EndLevel;

#[derive(Event, Debug)]
pub struct CleanupLevel;

fn exit_playing(mut commands: Commands) {
    commands.trigger(CleanupLevel);
}

fn cleanup_level(
    _trigger: Trigger<CleanupLevel>,
    mut commands: Commands,
    query: Query<Entity, Or<(With<LevelMarker>, With<Player>, With<Duckling>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn on_end_level(
    _trigger: Trigger<EndLevel>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    commands.trigger(FadeOut { duration: 0.5 });
    next_state.set(LevelState::EndLevelFadeOut);
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Level {
    pub map: String,
    pub size: IVec2,
    pub start_tile: IVec2,
    pub duckling_spawn_points: Vec<IVec2>,
}

#[derive(Debug, Clone, PartialEq, Reflect, Resource)]
pub struct Levels {
    levels: Vec<Level>,
}

impl Default for Levels {
    fn default() -> Self {
        Self {
            levels: vec![
                Level {
                    map: "level1.tmx".to_string(),
                    size: IVec2::new(21, 21),
                    start_tile: IVec2::new(-2, 2),
                    duckling_spawn_points: vec![
                        IVec2::new(0, -6),
                        IVec2::new(0, 8),
                        IVec2::new(6, 3),
                    ],
                },
                Level {
                    map: "level2.tmx".to_string(),
                    size: IVec2::new(21, 21),
                    start_tile: IVec2::new(-8, -8),
                    duckling_spawn_points: vec![
                        IVec2::new(-8, 8),
                        IVec2::new(8, -5),
                        IVec2::new(4, 0),
                        IVec2::new(-6, 1),
                    ],
                },
            ],
        }
    }
}

impl Levels {
    pub fn current(&self, level: CurrentLevel) -> Option<&Level> {
        self.levels.get(level.0 as usize)
    }
}

#[derive(Event, Debug)]
pub struct GameCompleted;

fn on_game_completed(
    _trigger: Trigger<GameCompleted>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    commands.trigger(FadeOut { duration: 0.5 });
    next_state.set(LevelState::CompletedFadeOut);
}
