use bevy::prelude::*;

use crate::{AppSet, CameraMarker};

use super::spawn::player::Player;

pub(super) fn plugin(app: &mut App) {
    // Apply movement based on controls.
    // app.add_systems(FixedUpdate, (move_camera,).chain().in_set(AppSet::Update));
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<(&mut Transform,), (With<CameraMarker>, Without<Player>)>,
    player_query: Query<(&Transform,), (With<Player>, Without<CameraMarker>)>,
) {
    let Ok(mut camera_transform) = query.get_single_mut() else {
        return;
    };
    let Ok(player) = player_query.get_single() else {
        return;
    };

    // Make the camera follow the player.

    let camera_tf = &mut camera_transform.0;
    let player = player.0;

    let player_position = player.translation;
    let camera_position = camera_tf.translation;

    let delta = player_position - camera_position;
    let distance = delta.length();

    // Adjust the speed based on the distance to the player
    let base_speed = 20.0; // Base speed when at a distance of 1 unit
    let speed_multiplier = 5.0; // Adjust this value to control how much the speed increases with distance
    let speed = base_speed + (distance * speed_multiplier);

    if distance > 1.0 {
        let direction = delta.normalize();
        let movement = direction * speed * time.delta_seconds();
        camera_tf.translation += movement;
    }
}
