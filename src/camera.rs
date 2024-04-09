use bevy::{core_pipeline::{fxaa::Fxaa, prepass::{DeferredPrepass, DepthPrepass, MotionVectorPrepass}}, input::{common_conditions::input_toggle_active, mouse::MouseMotion}, pbr::{DefaultOpaqueRendererMethod, DirectionalLightShadowMap}, prelude::*, render::{settings::{PowerPreference, WgpuSettings}, RenderPlugin}, window::{CursorGrabMode, PrimaryWindow}};
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
            capture_cursor: false,
            rotation_enabled: true,
            rotation_speed: 50.,
            translate_enabled: true,
            translate_speed: 1.,
            translate_offset: Vec3::ZERO,
        }
    }
}

pub struct PlayerCameraPlugin;

#[derive(Component)]
pub struct CopyCameraRotation;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Msaa::Off)
            .insert_resource(AmbientLight::NONE)
            .insert_resource(DefaultOpaqueRendererMethod::deferred())
            .insert_resource(DirectionalLightShadowMap { size: 4096 })

            .add_systems(Update, copy_rotation)
            .add_systems(PreStartup, camera_create)
            .add_systems(PreUpdate, camera_movement);
    }
}

fn copy_rotation(
    query_camera: Query<&Transform, (With<MainCamera>, Without<CopyCameraRotation>)>,
    mut query_copy: Query<&mut Transform, With<CopyCameraRotation>>
) {
    let transform = query_camera.single();
    for mut t_copy in &mut query_copy {
        t_copy.rotation = transform.rotation.clone();
    }
}

fn camera_create(mut commands: Commands) {
    commands.spawn(
        (
            Camera3dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainCamera::default(),

            DepthPrepass,
            MotionVectorPrepass,
            DeferredPrepass,
            Fxaa::default(),
            
            Name::new("Main_Camera"),
            FogSettings {
                color: Color::rgba(0., 0., 0., 1.0),
                falloff: FogFalloff::Linear {
                    start: 8.0,
                    end: 14.0,
                },
                ..default()
            },
        )
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

            if main_camera.translate_enabled {
                transform.translation = main_camera.translate_offset;
            }

            if main_camera.rotation_enabled {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                
                let window_scale = window.height().min(window.width());
                for ev in mouse_motion_events.read() {
                    pitch -= ((main_camera.rotation_speed * ev.delta.y) / window_scale).to_radians();
                    yaw -= ((main_camera.rotation_speed * ev.delta.x) / window_scale).to_radians();
                }
        
                pitch = pitch.clamp(-1.5, 1.5);
                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }

        }
    }
}