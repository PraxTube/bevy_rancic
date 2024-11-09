use chrono::Utc;

use bevy::{math::bounding::Aabb2d, prelude::*, transform::TransformSystem};
use bevy_rapier2d::plugin::PhysicsSet;
use noisy_bevy::simplex_noise_2d_seeded;

use super::MainCamera;

/// Sets that are used to control the camera's transform.
/// They run after rapier's `PhysicsSet::Writeback`
/// and before bevy's `TransformSystem::TransformPropagate`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CameraSystem {
    /// Set that will run right before of the camera transform update.
    /// Run your camera target update in this set!
    TargetUpdate,
    /// Set that will update the transform of the camera.
    /// You should not run anything in this set.
    TransformUpdate,
}

/// Use to add trauma/shake to your camera.
/// You must use this resource to update the camera's position.
#[derive(Resource, Default)]
pub struct CameraSettings {
    shake: CameraShake,
    bounds: Option<Aabb2d>,
}

struct CameraShake {
    trauma: f32,
    seed: f32,
    target: Vec2,
    noise_strength: f32,
    translation_shake_strength: f32,
    rotation_shake_strength: f32,
}

impl Default for CameraShake {
    fn default() -> Self {
        Self {
            trauma: 0.0,
            seed: 0.0,
            target: Vec2::ZERO,
            noise_strength: 10.0,
            translation_shake_strength: 15.0,
            rotation_shake_strength: 2.5,
        }
    }
}

impl CameraSettings {
    /// Add trauma to the camera shake.
    /// Trauma value is capped at `1.0`.
    pub fn add_trauma(&mut self, trauma: f32) {
        if self.shake.trauma == 0.0 {
            // Get the milliseconds only.
            // We do this to get a pseudo random seed.
            self.shake.seed = (Utc::now().timestamp_millis() & 0xFFFF) as f32;
        }
        self.shake.trauma = (self.shake.trauma + trauma.abs()).min(1.0);
    }

    /// Add trauma with an additional local threshold.
    /// If the trauma is already above this threshold, then return.
    /// Useful if you want to make sure that incremental
    /// trauma additions don't escalate.
    pub fn add_trauma_with_threshold(&mut self, trauma: f32, threshold: f32) {
        if self.shake.trauma >= threshold {
            return;
        }
        self.add_trauma(trauma);
    }

    /// Update the `noise_strength` value.
    pub fn set_noise_strength(&mut self, noise_strength: f32) {
        self.shake.noise_strength = noise_strength;
    }

    /// Update the `translation_shake_strength` value.
    pub fn set_translation_shake_strength(&mut self, translation_shake_strength: f32) {
        self.shake.translation_shake_strength = translation_shake_strength;
    }

    /// Update the `rotation_shake_strength` value.
    pub fn set_rotation_shake_strength(&mut self, rotation_shake_strength: f32) {
        self.shake.rotation_shake_strength = rotation_shake_strength;
    }

    /// Update the camera target position.
    /// This will set the camera's `Transform.translation`
    /// to this value right before the `TransformPropagate` system.
    ///
    /// You need to use this function to move the camera.
    pub fn update_target(&mut self, target: Vec2) {
        self.shake.target = target;
    }

    /// Set the camera bounds.
    ///
    /// When the camera target is updated, it will enforce that the camera is within these bounds.
    pub fn set_bound(&mut self, bounds: Aabb2d) {
        self.bounds = Some(bounds);
    }

    fn reduce_trauma(&mut self, delta: f32) {
        self.shake.trauma = (self.shake.trauma - delta.abs()).max(0.0)
    }

    fn noise_value(&self, stack: u32) -> f32 {
        simplex_noise_2d_seeded(
            Vec2::new(self.shake.trauma * self.shake.noise_strength, 0.0),
            self.shake.seed + stack as f32,
        )
    }

    fn clamp_pos(&self, pos: Vec2, projection_area: Vec2) -> Vec2 {
        let area = projection_area / 2.0;
        match self.bounds {
            Some(b) => {
                if projection_area.x >= b.max.x - b.min.x || projection_area.y >= b.max.y - b.min.y
                {
                    return pos;
                }

                Vec2::new(
                    pos.x.clamp(b.min.x + area.x, b.max.x - area.x),
                    pos.y.clamp(b.min.y + area.y, b.max.y - area.y),
                )
            }
            None => pos,
        }
    }
}

fn decay_shake_trauma(time: Res<Time>, mut shake: ResMut<CameraSettings>) {
    shake.reduce_trauma(time.delta_seconds());
}

fn update_camera(
    mut q_camera: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    camera_settings: Res<CameraSettings>,
) {
    let (mut transform, projection) = match q_camera.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let translation_offset = Vec3::new(
        camera_settings.noise_value(0),
        camera_settings.noise_value(1),
        0.0,
    ) * camera_settings.shake.trauma.powi(2)
        * camera_settings.shake.translation_shake_strength;
    let rotation_offset = Quat::from_rotation_z(
        (camera_settings.noise_value(2)
            * camera_settings.shake.trauma.powi(2)
            * camera_settings.shake.rotation_shake_strength)
            .to_radians(),
    );

    let pos = camera_settings.shake.target + translation_offset.truncate();
    transform.translation = camera_settings
        .clamp_pos(pos, projection.area.size())
        .extend(transform.translation.z);
    transform.rotation = rotation_offset;
}

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_systems(Update, (decay_shake_trauma,))
            .configure_sets(
                PostUpdate,
                (CameraSystem::TargetUpdate, CameraSystem::TransformUpdate)
                    .chain()
                    .after(PhysicsSet::Writeback)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(
                PostUpdate,
                update_camera.in_set(CameraSystem::TransformUpdate),
            );
    }
}
