use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{control::{CharacterAutostep, CharacterLength, KinematicCharacterController, KinematicCharacterControllerOutput}, dynamics::{GravityScale, RigidBody, Sleeping, Velocity}, geometry::Collider, parry::math::{Point, Vector}, pipeline::QueryFilter, plugin::RapierContext, rapier::geometry::Ray};
use crate::camera::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    speed: f32,
    look_mul: f32,
    noise_mul: f32,
    move_speed_mul: f32,
    jump_height_mul: f32,
    grounded: bool,
    velocity: Vec3,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(crate::camera::PlayerCameraPlugin)
        .add_systems(Startup, create_player)
        .add_systems(PreUpdate, update_grounded)
        .add_systems(PreUpdate, update_player);
    }
}

fn create_player(
    mut commands: Commands,
    mut main_camera: Query<(Entity, &mut crate::camera::MainCamera)>,
) {
    let player = commands.spawn((
        Transform::from_xyz(25., 10., 25.),
        GlobalTransform::default(),
        Player {
            speed: 2.,
            noise_mul: 1.,
            look_mul: 0.00008,
            move_speed_mul: 1.5,
            jump_height_mul: 0.07,
            ..Default::default()
        },
        Name::new("Player"),
        RigidBody::KinematicPositionBased,
        Collider::capsule(Vec3::new(0., 0., 0.), Vec3::new(0., 0.5, 0.), 0.15),
        GravityScale(0.15),
        Velocity::default(),
        Sleeping::disabled(),
        KinematicCharacterController {
            max_slope_climb_angle: 45.0_f32.to_radians(),
            min_slope_slide_angle: 30.0_f32.to_radians(),
            offset: CharacterLength::Relative(0.01),
            slide: true,
            snap_to_ground: Some(CharacterLength::Absolute(0.02)),
            // offset: CharacterLength::Relative(0.01),
            // autostep: Some(CharacterAutostep {
            //     max_height: CharacterLength::Relative(0.5),
            //     min_width: CharacterLength::Relative(0.2),
            //     include_dynamic_bodies: true,
            // }),
            ..Default::default()
        },
        ),
    ).id();

    for (entity, mut camera) in &mut main_camera {
        camera.translate_offset = Vec3::new(0., 0.45, 0.);
        commands.entity(entity).set_parent(player);
    }
}

fn update_grounded(
    mut query: Query<(
        &Transform,
        &Velocity,
        &mut Player,
    )>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, velocity, mut player) in &mut query {
        if velocity.linvel.y < 0. {
            player.grounded = false;
            return;
        }
        
        if let Some((entity, toi)) = rapier_context.cast_ray(
            transform.translation, 
            Vec3::new(0.0, -1., 0.0), 
            0.52, false, QueryFilter::exclude_kinematic().into(),
        ) {
            player.grounded = true;
        } else {
            player.grounded = false;
        }
    }

}

fn update_player(
    mut query: Query<
        (
            &mut Player, 
            &GravityScale,
            &mut KinematicCharacterController,
        )>,
    query_camera: Query<&Transform, With<MainCamera>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (mut player, gravity, mut controller) in &mut query {
        for camera_translation in &query_camera {
        
            let mut final_translation = controller.translation.unwrap_or(Vec3::ZERO);
        
            let local_z = camera_translation.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);
    
            let mut position = Vec3::ZERO;
            if input.pressed(KeyCode::KeyW) {
                position += forward;
            }
            if input.pressed(KeyCode::KeyA) {
                position -= right;
            }
            if input.pressed(KeyCode::KeyS) {
                position -= forward;
            }
            if input.pressed(KeyCode::KeyD) {
                position += right;
            }
            position = position.normalize_or_zero() * player.move_speed_mul;
    
            if player.grounded {
                player.velocity.y = player.velocity.y.max(0.);
                if input.just_pressed(KeyCode::Space) {
                    let mul = player.jump_height_mul;
                    player.velocity += Vec3::Y * mul;
                    player.grounded = false;
                }
            } else {
                player.velocity -= Vec3::new(0., gravity.0, 0.) * time.delta_seconds();
            }
    
            final_translation += (position * time.delta_seconds()) + player.velocity;
            controller.translation = Some(final_translation);

        }
    }
}