#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod camera;
mod loading;
mod map;
mod menu;
mod player;

use crate::camera::InternalCameraPlugin;
use crate::{
    actions::ActionsPlugin, audio::InternalAudioPlugin, loading::LoadingPlugin, map::MapPlugin,
    menu::MenuPlugin, player::PlayerPlugin,
};

use bevy::app::App;
#[cfg(feature = "diagnostic")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((
                ActionsPlugin,
                InternalCameraPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
                MapPlugin,
                MenuPlugin,
                LoadingPlugin,
            ))
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        #[cfg(debug_assertions)]
        {
            app.add_plugins(RapierDebugRenderPlugin::default());
        }
        #[cfg(feature = "diagnostic")]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}
