use bevy::{input::{common_conditions::input_toggle_active, mouse::MouseMotion}, prelude::*, render::{settings::{PowerPreference, WgpuSettings}, RenderPlugin}, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use std::{clone, default, sync::Mutex};
use lazy_static::lazy_static;

#[derive(Component)]
pub struct PlayerCamera {
    capture_cursor: bool,

    rotation_enabled: bool,
    rotation_speed: f32,

    translate_enabled: bool,
    translate_speed: f32,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            capture_cursor: true,
            rotation_enabled: true,
            rotation_speed: 1.,
            translate_enabled: true,
            translate_speed: 1.,
        }
    }
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, camera_create)
            .add_systems(PreUpdate, camera_rotation)
            .add_systems(PreUpdate, camera_translate);
    }
}

lazy_static! {
    static ref MAIN: Mutex<Option<Entity>> = Mutex::new(None);
}

pub fn camera_set_parent(commands: &mut Commands, entity: Entity) {
    let main_guard = MAIN.lock().unwrap();
    if let Some(cam) = *main_guard {
        commands.entity(cam).set_parent(entity);
    }
}

pub fn camera_remove_parent(commands: &mut Commands) {
    let main_guard = MAIN.lock().unwrap();
    if let Some(cam) = *main_guard {
        commands.entity(cam).remove_parent();
    }
}

pub fn camera_set_settings(commands: &mut Commands, new_settings: PlayerCamera) {
    let main_guard = MAIN.lock().unwrap();
    if let Some(cam) = *main_guard {
        commands.entity(cam).insert(new_settings);
    }
}

fn camera_create(mut commands: Commands) {
    let mut main_guard = MAIN.lock().unwrap();
    main_guard.replace(
        commands.spawn(
            (Camera3dBundle::default(), PlayerCamera::default())
        ).id()
    );
}

fn camera_rotation(
    mut player: Query<(&mut Transform, &PlayerCamera)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut motion: EventReader<MouseMotion>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut transform, player) in &mut player {
            if !player.rotation_enabled {
                return;
            }

            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            
            let window_scale = window.height().min(window.width());
            for ev in motion.read() {
                pitch -= (player.rotation_speed * ev.delta.y * window_scale).to_radians();
                yaw -= (player.rotation_speed * ev.delta.x * window_scale).to_radians();
            }
    
            pitch = pitch.clamp(-1.54, 1.54);
            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn camera_translate() {

}