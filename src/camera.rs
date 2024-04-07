use bevy::{input::{common_conditions::input_toggle_active, mouse::MouseMotion}, prelude::*, render::{settings::{PowerPreference, WgpuSettings}, RenderPlugin}, window::{CursorGrabMode, PrimaryWindow}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use std::{borrow::Borrow, clone, default, sync::Mutex};
use lazy_static::lazy_static;

#[derive(Component)]
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
            rotation_speed: 0.8,
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
            .add_systems(PreStartup, camera_create)
            .add_systems(PreUpdate, camera_movement);
    }
}

fn camera_create(mut commands: Commands) {
    commands.spawn(
        (
            Camera3dBundle::default(), 
            MainCamera::default(),
            Name::new("Main_Camera"),)
    );
}

fn camera_movement(
    mut query_camera: Query<(&mut Transform, &mut MainCamera)>,
    mut query_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    if let Ok(mut window) = query_windows.get_single_mut() {
        for (mut transform, main_camera) in &mut query_camera {
            
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

            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            
            let window_scale = window.height().min(window.width()) / 4096.0;
            for ev in mouse_motion_events.read() {
                pitch -= (main_camera.rotation_speed * ev.delta.y * window_scale).to_radians();
                yaw -= (main_camera.rotation_speed * ev.delta.x * window_scale).to_radians();
            }
    
            pitch = pitch.clamp(-1.5, 1.5);
            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            transform.translation = main_camera.translate_offset;
        }
    }
}