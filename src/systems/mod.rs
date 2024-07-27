use bevy::prelude::*;

pub mod fade;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((fade::plugin,));
}
