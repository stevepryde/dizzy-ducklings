//! Handle player input and translate it into movement.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/latest/examples/movement/physics_in_fixed_timestep.rs).

use bevy::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};

pub const GRAVITY: f32 = -9.81 * 32.0 * 4.0;
pub const TERMINAL_VELOCITY: f32 = -420.0;

use crate::AppSet;

use super::{ground::IsOnGround, spawn::player::Velocity};

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
        FixedUpdate,
        (apply_movement, detect_ground)
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
                && output.effective_translation.y >= 0.0;
        } else if !output.grounded {
            is_on_ground.is_on_ground = false;
        }

        // Did we hit our head?
        if output.desired_translation.y > 0.0 && output.effective_translation.y <= 0.0 {
            log::info!("Bumped head!");
            velocity.y = 0.0;
        }
    }
}
