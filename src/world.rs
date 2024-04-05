use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use rand::Rng;

use crate::Game;

#[derive(Component)]
struct MapParent;

#[derive(Component)]
struct MapPhysicsParent;

#[derive(Reflect)]
pub struct Cell {
    height: f32,
    x: i32,
    y: i32,
}

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, create_map);
    }
}

fn create_map(
    mut commands: Commands, 
    mut game: ResMut<Game>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut map_entity = commands.spawn((SpatialBundle::default(), MapParent, Name::new("Map")));

    game.map = (0..game.board_size).map(|x| {
        (0..game.board_size).map(|y| {
            let height = rand::thread_rng().gen_range(0.0..1.0);

            map_entity.with_children(|commands| {
                commands.spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(x as f32, height, y as f32),
                        mesh: meshes.add(Cuboid::new(1.0, height * 2., 1.0)),
                        material: materials.add(Color::rgb(1., 1., 1.)),
                        ..default()
                    },
                    Name::new(format!("Cell {}:{}", x, y))
                ));
            });
            Cell { 
                height: height, 
                x: x as i32, 
                y: y as i32
            }
        }).collect()
    }).collect();

    create_map_physics(commands, game);
}

fn create_map_physics(mut commands: Commands, game: ResMut<Game>) {
    let mut map_physics_entity = commands.spawn((SpatialBundle::default(), MapPhysicsParent, Name::new("MapPhysics")));

    for x in &game.map {
        for y in x {
            map_physics_entity.with_children(|commands| {
                    commands.spawn((
                        Name::new(format!("PhysicsCell {}:{}", y.x, y.y)),
                        RigidBody::Fixed,
                        Collider::cuboid(0.5, y.height, 0.5),
                        TransformBundle::from(Transform::from_xyz(y.x as f32, y.height, y.y as f32))
                    ));
                }
            );
        }
    }
}