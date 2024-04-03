mod player;
mod bot;

use std::env::set_var;
use bevy::{input::{common_conditions::input_toggle_active, mouse::MouseMotion}, prelude::*, render::{settings::{PowerPreference, WgpuSettings}, RenderPlugin}, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use rand::Rng;

fn main() {
    // fix for my PC having multiple GPUs
    set_var("DISABLE_LAYER_AMD_SWITCHABLE_GRAPHICS_1", "1");
    App::new()
        .add_plugins(DefaultPlugins
            //.set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Project 4".into(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(RenderPlugin {
                render_creation: WgpuSettings {
                    power_preference: PowerPreference::HighPerformance,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
        )
        .add_plugins((player::PlayerPlugin, bot::BotPlugin))

        // https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(Game::default())
        .add_systems(Startup, create_map)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}

#[derive(Component)]
struct MapParent;

struct Cell {
    height: f32
}

#[derive(Resource, Default)]
struct Game {
    map: Vec<Vec<Cell>>,
    finish_loc: Vec3,
    start_loc: Vec3,
    bot_count: u8,
}

const BOARDSIZE: usize = 100;

fn create_map(
    mut commands: Commands, 
    mut game: ResMut<Game>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut map_entity = commands.spawn((SpatialBundle::default(), MapParent, Name::new("Map")));

    game.map = (0..BOARDSIZE).map(|x| {
        (0..BOARDSIZE).map(|y| {
            let height = rand::thread_rng().gen_range(0.0..1.0);

            map_entity.with_children(|commands| {
                commands.spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(x as f32, height, y as f32),
                        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
                        ..default()
                    },
                    Name::new(format!("Cell {}:{}", x, y))
                ));
            });
            Cell { height }
        }).collect()
    }).collect();

}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}