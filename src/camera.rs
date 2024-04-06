use bevy::{input::{common_conditions::input_toggle_active, mouse::MouseMotion}, prelude::*, render::{settings::{PowerPreference, WgpuSettings}, RenderPlugin}, window::{CursorGrabMode, PrimaryWindow}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use std::{borrow::Borrow, clone, default, sync::Mutex};
use lazy_static::lazy_static;

#[derive(Resource)]
pub struct MainCamera {
    pub capture_cursor: bool,

    pub rotation_enabled: bool,
    pub rotation_speed: f32,

    pub translate_enabled: bool,
    pub translate_speed: f32,

    pub translate_offset: Vec3,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            capture_cursor: true,
            rotation_enabled: true,
            rotation_speed: 1.,
            translate_enabled: true,
            translate_speed: 1.,
            translate_offset: Vec3::ZERO,
        }
    }
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MainCamera::default())
            .add_systems(PreStartup, camera_create)
            .add_systems(PreUpdate, camera_movement);
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

fn camera_create(mut commands: Commands) {
    let mut main_guard = MAIN.lock().unwrap();
    main_guard.replace(
        commands.spawn(
            Camera3dBundle::default()
        ).id()
    );
}

fn camera_movement(
    main_camera: Res<MainCamera>,
    mut query_transforms: Query<&mut Transform>,
    mut query_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let main_guard = MAIN.lock().unwrap();
    if let Ok(mut window) = query_windows.get_single_mut() {

        if main_camera.capture_cursor {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        } else {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }

        if !main_camera.rotation_enabled {
            return;
        }
        if let Some(cam) = *main_guard {
            if let Ok(mut transform) = query_transforms.get_mut(cam) {
                
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                
                let window_scale = window.height().min(window.width()) / 4096.0;
                for ev in mouse_motion_events.read() {
                    pitch -= (main_camera.rotation_speed * ev.delta.y * window_scale).to_radians();
                    yaw -= (main_camera.rotation_speed * ev.delta.x * window_scale).to_radians();
                }
        
                pitch = pitch.clamp(-1.54, 1.54);
                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
                transform.translation = main_camera.translate_offset;
            }
        }
    }
}