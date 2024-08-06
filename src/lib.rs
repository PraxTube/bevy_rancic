#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, dead_code)]
#![warn(unused_imports, missing_docs)]

mod audio;
mod camera;
mod physics;
mod utils;

/// Use this whenever you need a RNG.
pub type GameRng = rand_xoshiro::Xoshiro256PlusPlus;

use bevy::prelude::{App, Plugin};

/// Adds highly opionated common functionality for 2D top down games.
pub struct RancicPlugin;

impl Plugin for RancicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            utils::UtilsPlugin,
            audio::GameAudioPlugin,
            physics::PhysicsPlugin,
            camera::CameraPlugin,
        ));
    }
}

/// `use bevy_rancic::prelude::*;` to import common components and plugins.
pub mod prelude {
    pub use crate::audio::{GameAudio, PlaySound, SpacialSound};
    pub use crate::camera::{
        CameraShake, CameraSystem, MainCamera, ToggleFullscreenEvent, YSort, YSortChild,
        YSortStatic, YSortStaticChild, ZoomCameraScaleEvent,
    };
    pub use crate::utils::{
        debug::{DebugState, ToggleDebugStateEvent},
        quat_from_vec2, quat_from_vec3, COLLISION_GROUPS_NONE,
    };
    pub use crate::RancicPlugin;
}
