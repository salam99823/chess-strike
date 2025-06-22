use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct MapPlugin;

#[derive(Component)]
pub struct World;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(100., 4., 100.))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0., -5.0, 0.),
        World,
        RigidBody::Fixed,
        Collider::cuboid(50., 2., 50.),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10., 4., 10.))),
        Transform::from_xyz(0., 5.0, 0.),
        World,
        RigidBody::Dynamic,
        Collider::cuboid(5., 2., 5.),
    ));
}
