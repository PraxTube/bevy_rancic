use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::GameAudio;

/// Add this to any entity you want to have spacial audio on.
/// This will adjust the volume of the corresponding audio clip
/// based on the distance between the `Transform` of this entity
/// and that of the main camera.
///
/// This requires the entity to have a `Transform`
/// and there to be exactly one `AudioReceiver`
/// (probably on the main camera).
#[derive(Component)]
pub struct SpacialSound {
    volume: f64,
}

impl SpacialSound {
    /// Create new `SpacialSound` with given volume.
    /// This volume will be multiplied by the distance to the `AudioReceiver`.
    pub fn new(volume: f64) -> Self {
        Self { volume }
    }
}

fn update(
    game_audio: &Res<GameAudio>,
    receiver_transform: &GlobalTransform,
    emitters: &Query<(&GlobalTransform, &AudioEmitter, &SpacialSound)>,
    audio_instances: &mut Assets<AudioInstance>,
) {
    for (emitter_transform, emitter, sound) in emitters {
        let distance = (emitter_transform.translation() - receiver_transform.translation())
            .truncate()
            .length_squared();
        let multiplier =
            (1.0 - distance as f64 / game_audio.max_spacial_distance.powi(2)).clamp(0.0, 1.0);
        let volume: f64 = sound.volume * multiplier.powi(2) * game_audio.global_volume;

        for instance in emitter.instances.iter() {
            if let Some(instance) = audio_instances.get_mut(instance) {
                instance.set_volume(volume, AudioTween::default());
            }
        }
    }
}

fn update_volumes(
    game_audio: Res<GameAudio>,
    receiver: Query<&GlobalTransform, With<AudioReceiver>>,
    emitters: Query<(&GlobalTransform, &AudioEmitter, &SpacialSound)>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    match receiver.get_single() {
        Ok(r) => update(&game_audio, r, &emitters, &mut audio_instances),
        Err(err) => error!(
            "There must be exactly one entity with an `AudioReceiver`. {}",
            err
        ),
    };
}

fn cleanup_stopped_spacial_instances(
    mut emitters: Query<&mut AudioEmitter>,
    instances: Res<Assets<AudioInstance>>,
) {
    for mut emitter in emitters.iter_mut() {
        let handles = &mut emitter.instances;

        handles.retain(|handle| {
            if let Some(instance) = instances.get(handle) {
                instance.state() != PlaybackState::Stopped
            } else {
                true
            }
        });
    }
}

pub struct SpacialAudioPlugin;

impl Plugin for SpacialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_volumes, cleanup_stopped_spacial_instances));
    }
}
