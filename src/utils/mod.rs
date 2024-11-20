pub mod debug;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Empty `CollisionGroups` for Rapier.
/// Useful if you want disable collisions for an entity completely.
pub const COLLISION_GROUPS_NONE: CollisionGroups = CollisionGroups::new(Group::NONE, Group::NONE);
/// Transparent `ColliderDebugColor` for Rapier colliders.
/// Useful if you want to hide colliders (when they are inactive for example).
pub const COLLIDER_COLOR_TRANSPARENT: ColliderDebugColor = ColliderDebugColor(Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 0.0,
});
/// White `ColliderDebugColor` for Rapier colliders.
/// Useful if you want to highlight colliders (when they are active for example).
pub const COLLIDER_COLOR_WHITE: ColliderDebugColor = ColliderDebugColor(Hsla {
    hue: 1.0,
    saturation: 1.0,
    lightness: 1.0,
    alpha: 1.0,
});
/// Black `ColliderDebugColor` for Rapier colliders.
/// Useful if you want to highlight colliders (when they are active for example).
pub const COLLIDER_COLOR_BLACK: ColliderDebugColor = ColliderDebugColor(Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 1.0,
});

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug::DebugPlugin);
    }
}

/// Convert `Vec2` to `Quat` by taking angle between `Vec2::X`.
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
