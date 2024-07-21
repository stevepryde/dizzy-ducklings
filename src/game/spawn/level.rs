//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::AppSet;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
    app.add_systems(FixedUpdate, (rotate_world,).chain().in_set(AppSet::Update));
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

#[derive(Component)]
pub struct LevelMarker;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let mapx = 10. * 32. + 16.;
    let mapy = 10. * 32. + 16.;
    let map_handle: Handle<TiledMap> = asset_server.load("level1.tmx");

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
                    // This is the default, but we're setting it explicitly here for clarity.
                    collision_object_names: ObjectNames::All,
                    ..default()
                },
                transform: Transform::from_xyz(-mapx, -mapy, 0.0),
                ..Default::default()
            });
        });

    commands.trigger(SpawnPlayer);
}

fn rotate_world(time: Res<Time>, mut query: Query<&mut Transform, With<LevelMarker>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(f32::to_radians(time.delta_seconds())));
    }
}
