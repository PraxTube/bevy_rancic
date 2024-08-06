use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::IntegrationParameters};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, configure_physics);
    }
}

fn configure_physics(
    mut rapier_config: ResMut<RapierConfiguration>,
    mut rapier_context: ResMut<RapierContext>,
) {
    rapier_config.gravity = Vec2::ZERO;
    rapier_context.integration_parameters = IntegrationParameters {
        normalized_max_corrective_velocity: f32::MAX,
        contact_damping_ratio: 1.0,
        ..default()
    };
}
