mod player;
mod bot;
mod world;

use std::env::set_var;
use bevy::{input::{common_conditions::input_toggle_active, mouse::MouseMotion}, prelude::*, render::{settings::{PowerPreference, WgpuSettings}, RenderPlugin}, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use world::Cell;

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
        .add_plugins((player::PlayerPlugin, bot::BotPlugin, world::WorldGenPlugin))

        // https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())

        .insert_resource(Game{
            board_size: 50,
            bot_count: 10,
            ..Default::default()
        })

        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .run();
}

#[derive(Resource, Default, Reflect)]
struct Game {
    map: Vec<Vec<Cell>>,
    finish_loc: Vec3,
    start_loc: Vec3,
    bot_count: u8,
    board_size: u8,
}

fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}