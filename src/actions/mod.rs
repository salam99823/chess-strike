use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions.
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (
                set_movement_actions.run_if(in_state(GameState::Playing)),
                rotate_camera.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Vec2,
    pub cameara_rotation: Vec2,
}

pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    actions.player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );
}

pub fn rotate_camera(mut actions: ResMut<Actions>, mut motion_evr: EventReader<MouseMotion>) {
    for ev in motion_evr.read() {
        actions.cameara_rotation.x -= ev.delta.y;
    }
}
