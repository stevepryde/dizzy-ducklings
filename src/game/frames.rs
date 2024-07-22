use bevy::prelude::*;

use crate::AppSet;

#[derive(Default, Resource)]
pub struct FrameCounter {
    pub count: u32,
}

#[derive(Event, Debug)]
pub struct ResetFrameCounter;

pub(super) fn plugin(app: &mut App) {
    app.observe(reset_frame_counter);
    app.insert_resource(FrameCounter { count: 0 })
        .add_systems(FixedUpdate, frame_count.in_set(AppSet::Update));
}

fn reset_frame_counter(
    _trigger: Trigger<ResetFrameCounter>,
    mut frame_counter: ResMut<FrameCounter>,
) {
    frame_counter.count = 0;
}

// The system to wait a few frames
fn frame_count(mut frame_counter: ResMut<FrameCounter>) {
    frame_counter.count += 1;
}
