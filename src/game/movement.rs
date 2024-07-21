//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use bevy_rapier2d::{
    control::{KinematicCharacterController, KinematicCharacterControllerOutput},
    geometry::Collider,
};

pub const GRAVITY: f32 = -9.81 * 32.0 * 4.0;
pub const TERMINAL_VELOCITY: f32 = -420.0;

use crate::AppSet;

use super::spawn::{
    level::LevelMarker,
    player::{IsOnGround, Player, SpriteMarker, Velocity},
};

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<Movement>();
    app.add_systems(
        Update,
        (apply_sprite_direction,).chain().in_set(AppSet::Update),
    );
    app.add_systems(
        FixedUpdate,
        (
            apply_movement,
            detect_ground,
            rotate_world,
            read_character_controller_collisions,
        )
            .chain()
            .in_set(AppSet::Update),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
    pub jump_speed: f32,
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(
        &MovementController,
        &Movement,
        &mut Velocity,
        &mut KinematicCharacterController,
        &IsOnGround,
    )>,
) {
    for (controller, movement, mut velocity, mut char_controller, is_on_ground) in
        &mut movement_query
    {
        // X velocity doesn't accumulate.
        velocity.x = movement.speed * controller.0.x;

        if controller.0.y > 0.0 {
            // Jumping.
            if is_on_ground.is_on_ground {
                velocity.y = movement.jump_speed;
            }
        }

        // Y velocity does, but only up to terminal velocity.
        if is_on_ground.is_on_ground {
            if velocity.y < 0.0 {
                velocity.y = 0.0;
            }
        }

        velocity.y += GRAVITY * time.delta_seconds();
        if velocity.y < TERMINAL_VELOCITY {
            velocity.y = TERMINAL_VELOCITY;
        }

        char_controller.translation =
            Some(Vec2::new(velocity.x, velocity.y) * time.delta_seconds());
    }
}

fn detect_ground(
    mut controllers: Query<(
        Entity,
        &mut IsOnGround,
        &mut Velocity,
        &KinematicCharacterControllerOutput,
    )>,
) {
    for (_, mut is_on_ground, mut velocity, output) in controllers.iter_mut() {
        if !is_on_ground.is_on_ground {
            is_on_ground.is_on_ground = output.grounded
                && output.desired_translation.y < 0.0
                && output.effective_translation.y >= -0.5;
        } else if !output.grounded {
            is_on_ground.is_on_ground = false;
        }

        // Did we hit our head?
        if output.desired_translation.y > 0.0 && output.effective_translation.y <= 0.5 {
            velocity.y = 0.0;
        }
    }
}

fn apply_sprite_direction(
    mut query_sprite: Query<(&Parent, &mut Transform), (With<SpriteMarker>, Without<Player>)>,
    query_player: Query<&MovementController, With<Player>>,
) {
    for (parent, mut transform) in &mut query_sprite.iter_mut() {
        if let Ok(movement) = query_player.get(parent.get()) {
            if movement.0.x < 0.0 {
                transform.scale = Vec3::new(-1.0, 1.0, 1.0);
            } else if movement.0.x > 0.0 {
                transform.scale = Vec3::new(1.0, 1.0, 1.0);
            }
        }
    }
}

fn rotate_world(time: Res<Time>, mut query: Query<&mut Transform, With<LevelMarker>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(f32::to_radians(
            5. * time.delta_seconds(),
        )));
    }
}

fn read_character_controller_collisions(
    time: Res<Time>,
    mut character_controller_outputs: Query<(
        &GlobalTransform,
        &mut Transform,
        &KinematicCharacterControllerOutput,
    )>,
    colliders: Query<(&GlobalTransform, &Collider)>,
) {
    for (global_transform, mut transform, output) in character_controller_outputs.iter_mut() {
        for collision in &output.collisions {
            // move the ball away from the collision.
            if let Ok((collider_tf, _)) = colliders.get(collision.entity) {
                let delta = global_transform.translation() - collider_tf.translation();
                let distance = delta.length();
                let direction = delta.normalize();
                let movement = direction * distance;
                transform.translation += movement * time.delta_seconds();
            }
        }
    }
}
