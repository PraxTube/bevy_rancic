use chrono::Utc;

use bevy::{prelude::*, transform::TransformSystem};
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
#[derive(Resource)]
pub struct CameraShake {
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

impl CameraShake {
    /// Add trauma to the camera shake.
    /// Trauma value is capped at `1.0`.
    pub fn add_trauma(&mut self, trauma: f32) {
        if self.trauma == 0.0 {
            // Get the milliseconds only.
            // We do this to get a pseudo random seed.
            self.seed = (Utc::now().timestamp_millis() & 0xFFFF) as f32;
        }
        self.trauma = (self.trauma + trauma.abs()).min(1.0);
    }

    /// Add trauma with an additional local threshold.
    /// If the trauma is already above this threshold, then return.
    /// Useful if you want to make sure that incremental
    /// trauma additions don't escalate.
    pub fn add_trauma_with_threshold(&mut self, trauma: f32, threshold: f32) {
        if self.trauma >= threshold {
            return;
        }
        self.add_trauma(trauma);
    }

    /// Update the `noise_strength` value.
    pub fn set_noise_strength(&mut self, noise_strength: f32) {
        self.noise_strength = noise_strength;
    }

    /// Update the `translation_shake_strength` value.
    pub fn set_translation_shake_strength(&mut self, translation_shake_strength: f32) {
        self.translation_shake_strength = translation_shake_strength;
    }

    /// Update the `rotation_shake_strength` value.
    pub fn set_rotation_shake_strength(&mut self, rotation_shake_strength: f32) {
        self.rotation_shake_strength = rotation_shake_strength;
    }

    /// Update the camera target position.
    /// This will set the camera's `Transform.translation`
    /// to this value right before the `TransformPropagate` system.
    ///
    /// You need to use this function to move the camera.
    pub fn update_target(&mut self, target: Vec2) {
        self.target = target;
    }

    fn reduce_trauma(&mut self, delta: f32) {
        self.trauma = (self.trauma - delta.abs()).max(0.0)
    }

    fn noise_value(&self, stack: u32) -> f32 {
        simplex_noise_2d_seeded(
            Vec2::new(self.trauma * self.noise_strength, 0.0),
            self.seed + stack as f32,
        )
    }
}

fn decay_shake_trauma(time: Res<Time>, mut shake: ResMut<CameraShake>) {
    shake.reduce_trauma(time.delta_seconds());
}

fn update_camera(mut q_camera: Query<&mut Transform, With<MainCamera>>, shake: Res<CameraShake>) {
    let mut transform = match q_camera.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let translation_offset = Vec3::new(shake.noise_value(0), shake.noise_value(1), 0.0)
        * shake.trauma.powi(2)
        * shake.translation_shake_strength;
    let rotation_offset = Quat::from_rotation_z(
        (shake.noise_value(2) * shake.trauma.powi(2) * shake.rotation_shake_strength).to_radians(),
    );

    transform.translation = shake.target.extend(transform.translation.z) + translation_offset;
    transform.rotation = rotation_offset;
}

pub struct CameraShakePlugin;

impl Plugin for CameraShakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraShake>()
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
