//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::{game::frames::ResetFrameCounter, screen::Screen};

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(start_new_game);
    app.observe(spawn_level);
    app.observe(cleanup_level);
    app.observe(end_level);
    app.observe(on_game_completed);
    app.add_systems(OnExit(Screen::Playing), exit_playing);
}

#[derive(Resource, Clone, Copy, Debug)]
pub struct CurrentLevel(i32);

#[derive(Event, Debug)]
pub struct StartNewGame;

fn start_new_game(_trigger: Trigger<StartNewGame>, mut commands: Commands) {
    commands.init_resource::<Levels>();
    commands.insert_resource(CurrentLevel(0));
    commands.trigger(SpawnLevel);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component)]
pub struct LevelMarker;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    current_level: Res<CurrentLevel>,
    levels: Res<Levels>,
) {
    let level = levels.current(*current_level).unwrap();
    let mapx = level.size.x as f32 * 16. - 16.;
    let mapy = level.size.y as f32 * 16. - 16.;
    let map_handle: Handle<TiledMap> = asset_server.load(&level.map);

    commands
        .spawn((
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
                    ..default()
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

            // Set level finish point.
            parent.spawn((
                LevelFinishPoint,
                SpatialBundle {
                    transform: Transform::from_xyz(
                        level.end_tile.x as f32 * 32.,
                        level.end_tile.y as f32 * 32.,
                        0.,
                    ),
                    ..default()
                },
            ));
        });

    commands.trigger(ResetFrameCounter);
    commands.trigger(SpawnPlayer);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct PlayerSpawnPoint;

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
    query: Query<Entity, With<LevelMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn end_level(
    _trigger: Trigger<EndLevel>,
    mut level: ResMut<CurrentLevel>,
    levels: Res<Levels>,
    mut commands: Commands,
) {
    commands.trigger(CleanupLevel);

    level.0 += 1;
    match levels.current(*level) {
        Some(_) => {
            commands.trigger(SpawnLevel);
        }
        None => {
            // TODO: finish the game!
            commands.trigger(GameCompleted);
            return;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
pub struct Level {
    pub map: String,
    pub size: IVec2,
    pub start_tile: IVec2,
    pub end_tile: IVec2,
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
                    end_tile: IVec2::new(-7, 6),
                },
                // Level {
                //     map: "level2.tmx".to_string(),
                //     start_tile: Vec2::new(0., 0.),
                //     end_tile: Vec2::new(10., 10.),
                // },
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

fn on_game_completed(_trigger: Trigger<GameCompleted>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
