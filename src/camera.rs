use std::f32;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct InternalCameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for InternalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(CursorLock(false))
            .add_systems(
                Update,
                (
                    lock_cursor_position,
                    // move_camera
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

#[derive(Resource, Deref, DerefMut)]
pub struct CursorLock(pub bool);

pub fn lock_cursor_position(
    mut primary: Single<&mut Window, With<PrimaryWindow>>,
    cursor_lock_state: Res<CursorLock>,
) {
    let cursor_options = &mut primary.cursor_options;
    if **cursor_lock_state {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}

pub fn move_camera(
    cursor_lock_state: Res<CursorLock>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if **cursor_lock_state {
        for mut transform in camera_query.iter_mut() {
            for ev in motion_evr.read() {
                transform.rotation *=
                    Quat::from_rotation_x(-ev.delta.y * f32::consts::PI / 180. / 20.);
            }
        }
    }
}
