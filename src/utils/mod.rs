pub mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Empty `CollisionGroups` for Rapier.
/// Useful if you want disable collisions for an entity completely.
pub const COLLISION_GROUPS_NONE: CollisionGroups = CollisionGroups::new(Group::NONE, Group::NONE);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug::DebugPlugin);
    }
}

/// Convert `Vec2` to `Quat` by taking angle bettwen `Vec2::X`.
/// Returns `Quat::IDENTITY` for `Vec2::ZERO`.
pub fn quat_from_vec2(direction: Vec2) -> Quat {
    if direction == Vec2::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, Vec2::X.angle_between(direction))
}

/// Convert `Vec3` to `Quat` by truncating the z value,
/// then calculates the rotation in the x-y-plane.
pub fn quat_from_vec3(direction: Vec3) -> Quat {
    quat_from_vec2(direction.truncate())
}
