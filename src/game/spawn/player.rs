//! Spawn the player.

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{Movement, MovementController},
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<Player>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct IsOnGround {
    pub is_on_ground: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct SpriteMarker;

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands
        .spawn((
            Name::new("Player"),
            Player,
            SpatialBundle {
                transform: Transform::from_xyz(48.0, 48.0, 0.),
                ..default()
            },
            Velocity::default(),
            MovementController::default(),
            Movement {
                speed: 200.0,
                jump_speed: 400.0,
            },
            StateScoped(Screen::Playing),
            Collider::cuboid(9.0, 11.5),
            Friction::coefficient(0.0),
            // Restitution::coefficient(0.0),
            RigidBody::KinematicPositionBased,
            KinematicCharacterController {
                offset: CharacterLength::Absolute(1.0),
                ..default()
            },
            IsOnGround::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: image_handles[&ImageKey::Ducky].clone_weak(),
                    transform: Transform::from_xyz(0.0, 4.0, 0.0),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: player_animation.get_atlas_index(),
                },
                player_animation,
                SpriteMarker,
            ));
        });
}
