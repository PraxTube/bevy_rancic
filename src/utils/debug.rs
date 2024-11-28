use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Indicates whether the game is currently in debug mode.
/// This can be used for just debugging info to the player (developer),
/// or it can also act as a trigger to allow cheats etc.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct DebugState(pub bool);

/// Send this Event to toggle the `DebugState`
#[derive(Event)]
pub struct ToggleDebugStateEvent;

fn toggle_debug_mod(mut debug_active: ResMut<DebugState>) {
    **debug_active = !**debug_active;
}

fn toggle_rapier_debug(
    mut debug_context: ResMut<DebugRenderContext>,
    debug_active: Res<DebugState>,
) {
    if debug_context.enabled != **debug_active {
        debug_context.enabled = false;
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugState>()
            .add_event::<ToggleDebugStateEvent>()
            .add_systems(
                Update,
                (
                    toggle_debug_mod.run_if(on_event::<ToggleDebugStateEvent>()),
                    toggle_rapier_debug,
                ),
            );
    }
}
