use crate::actions::Actions;
use crate::camera::{CursorLock, MainCamera};
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera: Single<(Entity, &mut Transform), With<MainCamera>>,
    mut cursor_lock_state: ResMut<CursorLock>,
) {
    let player = commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(2., 2., 2.))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0., 0.5, 0.),
            Player,
            RigidBody::KinematicPositionBased,
            Collider::cuboid(1., 1., 1.),
            KinematicCharacterController::default(),
            GravityScale(1.),
        ))
        .id();

    cursor_lock_state.0 = true;
    camera.1.translation = Vec3::new(0., 1., 0.);
    *camera.1 = camera.1.looking_to(Dir3::Z, Dir3::Y);
    if let Ok(mut entity_commands) = commands.get_entity(camera.0) {
        entity_commands.insert(ChildOf(player));
    }
}

const SPEED: f32 = 10.;

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let movement = Vec3::new(actions.player_movement.x, 0., actions.player_movement.y)
        * SPEED
        * time.delta_secs();

    for mut player_controller in &mut player_query {
        player_controller.translation = Some(movement);
    }
}
