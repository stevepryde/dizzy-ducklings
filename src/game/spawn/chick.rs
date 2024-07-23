use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::{
        animation::PlayerAnimation,
        assets::{HandleMap, ImageKey},
        movement::{PreviousPhysicalTranslation, SpriteOffset, VisualTranslation},
        spawn::player::{SpriteMarker, Velocity},
    },
    screen::Screen,
};

use super::level::ChickSpawnPoint;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_chick);
    app.register_type::<Chick>();
}

#[derive(Event, Debug)]
pub struct SpawnChick;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Chick;

fn spawn_chick(
    _trigger: Trigger<SpawnChick>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    spawn_points: Query<&Transform, With<ChickSpawnPoint>>,
) {
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    for spawn_point in spawn_points.iter() {
        let startx = spawn_point.translation.x;
        let starty = spawn_point.translation.y;
        log::info!("SPAWN CHICK AT: {}, {}", startx, starty);

        commands
            .spawn((
                Name::new("Chick"),
                Chick,
                SpatialBundle {
                    transform: Transform::from_xyz(startx, starty, 0.),
                    ..default()
                },
                Velocity::default(),
                StateScoped(Screen::Playing),
                Collider::ball(11.5),
                Friction::coefficient(0.0),
                Restitution::coefficient(1.0),
                RigidBody::Dynamic,
                PreviousPhysicalTranslation(Vec2::new(startx, starty)),
                VisualTranslation(Vec2::new(startx, starty)),
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
                    player_animation.clone(),
                    SpriteOffset(Vec2::new(0.0, 4.0)),
                    SpriteMarker,
                ));
            });
    }
}
