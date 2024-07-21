//! Spawn the main level by triggering other observers.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let mapx = 10. * 32. + 16.;
    let mapy = 10. * 32. + 16.;
    let map_handle: Handle<TiledMap> = asset_server.load("level1.tmx");
    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        tiled_settings: TiledMapSettings {
            // This is the default, but we're setting it explicitly here for clarity.
            collision_object_names: ObjectNames::All,
            ..default()
        },
        transform: Transform::from_xyz(-mapx, -mapy, 0.0),
        ..Default::default()
    });

    commands.trigger(SpawnPlayer);
}
