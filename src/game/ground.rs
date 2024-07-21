use bevy::{prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::*;

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // // Apply movement based on controls.
    app.register_type::<GroundSensor>();
    app.register_type::<IsOnGround>();
    // app.add_systems(
    //     FixedUpdate,
    //     (handle_ground_intersections,)
    //         .chain()
    //         .in_set(AppSet::Update),
    // );
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct GroundSensor;

#[derive(Bundle)]
pub struct GroundCheckBundle {
    pub name: Name,
    pub spatial: SpatialBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub ground_check: GroundSensor,
    pub active_events: ActiveEvents,
}

impl Default for GroundCheckBundle {
    fn default() -> Self {
        Self {
            name: Name::new("GroundCheck"),
            spatial: Default::default(),
            collider: Default::default(),
            sensor: Sensor,
            ground_check: Default::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct IsOnGround {
    pub is_on_ground: bool,
}

fn handle_ground_intersections(
    rapier_context: Res<RapierContext>,
    mut ground_query: Query<(Entity, &Parent, &mut GroundSensor)>,
    mut parent_query: Query<&mut IsOnGround>,
) {
    for (entity, parent, _ground) in ground_query.iter_mut() {
        if let Ok(mut is_on_ground) = parent_query.get_mut(parent.get()) {
            let was_on_ground = is_on_ground.is_on_ground;
            is_on_ground.is_on_ground = rapier_context
                .intersection_pairs_with(entity)
                .any(|(_, _, intersecting)| intersecting);
            if !was_on_ground && is_on_ground.is_on_ground {
                if is_on_ground.is_on_ground {
                    log::info!("Entity {:?} is on the ground!", parent.get());
                } else {
                    log::info!("Entity {:?} is no longer on the ground!", parent.get());
                }
            }
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct HeadSensor;

#[derive(Bundle)]
pub struct HeadCheckBundle {
    pub name: Name,
    pub spatial: SpatialBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub head_check: HeadSensor,
}

impl Default for HeadCheckBundle {
    fn default() -> Self {
        Self {
            name: Name::new("HeadCheck"),
            spatial: Default::default(),
            collider: Default::default(),
            sensor: Sensor,
            head_check: Default::default(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct IsBumpHead {
    pub is_bump_head: bool,
}

fn handle_head_intersections(
    rapier_context: Res<RapierContext>,
    mut head_query: Query<(Entity, &Parent, &mut HeadSensor)>,
    mut parent_query: Query<&mut IsBumpHead>,
) {
    for (entity, parent, _ground) in head_query.iter_mut() {
        if let Ok(mut is_bump_head) = parent_query.get_mut(parent.get()) {
            is_bump_head.is_bump_head = rapier_context
                .intersection_pairs_with(entity)
                .any(|(_, _, intersecting)| intersecting);
        }
    }
}
