mod sound;
mod spacial;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub use sound::PlaySound;
pub use spacial::SpacialSound;

const DEFAULT_VOLUME: f64 = 0.5;
const MAX_SPACIAL_DISTANCE: f64 = 250.0;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_plugins((spacial::SpacialAudioPlugin, sound::GameSoundPlugin))
            .init_resource::<GameAudio>();
    }
}

/// Global properties for all audio clips.
#[derive(Resource)]
pub struct GameAudio {
    /// The volume that all sounds will be multiplied by.
    global_volume: f64,
    /// The maximum distance for any spacial audio.
    /// Any sounds further away than this value will be muted.
    pub max_spacial_distance: f64,
}

impl Default for GameAudio {
    fn default() -> Self {
        Self {
            global_volume: DEFAULT_VOLUME,
            max_spacial_distance: MAX_SPACIAL_DISTANCE,
        }
    }
}

impl GameAudio {
    fn set_global_volume_clamped(&mut self, volume: f64) {
        self.global_volume = volume.clamp(0.0, 1.0);
    }

    /// Increment the global volume of the game by the given amount.
    /// The volume will always be clamped between `0.0..1.0`.
    pub fn increment_global_volume(&mut self, increment: f64) {
        self.set_global_volume_clamped(self.global_volume + increment);
    }

    /// Get the global volume of the game.
    pub fn global_volume(&self) -> f64 {
        self.global_volume
    }

    /// Set the global volume of the game.
    /// The volume will always be clamped between `0.0..1.0`.
    pub fn set_global_volume(&mut self, volume: f64) {
        self.set_global_volume_clamped(volume);
    }
}
