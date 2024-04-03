use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};


#[derive(Component)]
pub struct Player {
    speed: f32,
    look_multiplier: f32,
    noise_multiplier: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, create_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, player_look);
    }
}

fn create_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(), 
        Player {
            speed: 2.,
            noise_multiplier: 1.,
            look_multiplier: 0.00004,
        },
        Name::new("Player"))
    );
}

fn player_movement(
    mut player: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for(mut transform, player) in &mut player {
        let mut velocity = Vec3::ZERO;
        let local_z = transform.local_z();
        let forward = -Vec3::new(local_z.x, 0., local_z.z);
        let right = Vec3::new(local_z.z, 0., -local_z.x);
        let up = Vec3::new(0., local_z.y.abs(), 0.);

        if input.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if input.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if input.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if input.pressed(KeyCode::KeyD) {
            velocity += right;
        }

        if input.pressed(KeyCode::Space) {
            velocity += up;
        }
        if input.pressed(KeyCode::KeyC) {
            velocity -= up;
        }
        
        velocity = velocity.normalize_or_zero() * time.delta_seconds() * player.speed;
        transform.translation += velocity;
    }
}

fn player_look(
    mut player: Query<(&mut Transform, &Player)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut motion: EventReader<MouseMotion>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (mut transform, player) in &mut player {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            
            let window_scale = window.height().min(window.width());
            for ev in motion.read() {
                pitch -= (player.look_multiplier * ev.delta.y * window_scale).to_radians();
                yaw -= (player.look_multiplier * ev.delta.x * window_scale).to_radians();
            }
    
            pitch = pitch.clamp(-1.54, 1.54);
            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}
