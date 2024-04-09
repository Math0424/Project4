use std::collections::HashSet;

use bevy::{prelude::*, render::render_phase::Draw, window::PrimaryWindow};
use bevy_rapier3d::{dynamics::RigidBody, geometry::{Collider, Restitution}, plugin::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use rand::Rng;

use crate::{camera::MainCamera, Game};

#[derive(Component)]
struct MapParent;

#[derive(Component)]
struct MapPhysicsParent;

#[derive(Clone, Default)]
pub struct Cell {
    height: f32,
    x: usize,
    y: usize,
    children: HashSet<(usize, usize)>,
}

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, hide_far_away)
        .add_systems(Startup, create_map);
    }
}

#[derive(Component)]
struct HideFarAway;

struct Direction(isize, isize);

fn hide_far_away(
    mut commands: Commands, 
    query: Query<(Entity, &GlobalTransform), With<HideFarAway>>,
    camera_query: Query<&GlobalTransform, With<MainCamera>>,
) {
    let cam_transform = camera_query.single();

    for (entity, transform) in &query {
        let distance = (cam_transform.translation() - transform.translation()).length();
        
        if distance < 20.0 {
            commands.entity(entity).insert(Visibility::Visible);
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

fn create_map(
    mut commands: Commands, 
    mut game: ResMut<Game>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let mut map_entity = commands.spawn((SpatialBundle::default(), MapParent, Name::new("Map")));

    let mut starting_cells: Vec<Vec<Option<Cell>>> = Vec::new();
    let mut cells_to_visit: Vec<Cell> = Vec::new();
    let mut finished_cells: Vec<Vec<Cell>> = Vec::new();

    for x in 0..game.board_size {
        let mut inner_vec: Vec<Option<Cell>> = Vec::new();
        for y in 0..game.board_size {
            inner_vec.push(Option::Some(Cell {
                height: 0.,
                x: x,
                y: y,
                children: HashSet::new(),
            }));
        }
        starting_cells.push(inner_vec);
    }
    
    for _ in 0..game.board_size {
        finished_cells.push(vec![Cell::default(); game.board_size])
    }

    let half_width = (game.board_size / 2) as usize;
    cells_to_visit.push(starting_cells[half_width][half_width].clone().unwrap());
    starting_cells[half_width][half_width] = None;

    while let Some(mut curr) = cells_to_visit.pop() {
        let mut dirs: Vec<Direction> = Vec::new();
        if curr.x + 1 < game.board_size && starting_cells[curr.x + 1][curr.y].is_some() {
            dirs.push(Direction(1, 0));
        }
        if curr.x != 0 && starting_cells[curr.x - 1][curr.y].is_some() {
            dirs.push(Direction(-1, 0));
        }
        if curr.y + 1 < game.board_size && starting_cells[curr.x][curr.y + 1].is_some() {
            dirs.push(Direction(0, 1));
        }
        if curr.y != 0 && starting_cells[curr.x][curr.y - 1].is_some() {
            dirs.push(Direction(0, -1));
        }

        if dirs.len() == 0 {
            finished_cells[curr.x][curr.y] = curr.clone();
            continue;
        }

        let random_index = rng.gen_range(0..dirs.len());
        let dir = &dirs[random_index];
        
        let mut cell = starting_cells[(curr.x as isize + dir.0) as usize][(curr.y as isize + dir.1) as usize].clone().unwrap();
        cell.children.insert((curr.x, curr.y)); // point cell -> curr
        curr.children.insert((cell.x, cell.y)); // point curr -> cell
        starting_cells[(curr.x as isize + dir.0) as usize][(curr.y as isize + dir.1) as usize] = None;

        cells_to_visit.push(curr);
        cells_to_visit.push(cell);
    }
    
    for x in 0..game.board_size * 2 {
        for y in 0..game.board_size * 2 {
            let mut height = rand::thread_rng().gen_range(0.8..1.0);

            if x % 2 == 0 && y % 2 == 0 {
                let cell = &finished_cells[x / 2][y / 2];
                height = cell.height;
            } else if x % 2 == 0 && y % 2 != 0 {
                if finished_cells[x / 2][y / 2].children.contains(&(x / 2, (y / 2) + 1)) {
                    height = 0.;
                }
            } else if x % 2 != 0 && y % 2 == 0 {
                if finished_cells[x / 2][y / 2].children.contains(&((x / 2) + 1, y / 2)) {
                    height = 0.;
                }
            }

            map_entity.with_children(|commands| {
                commands.spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(x as f32 * 2., height, y as f32 * 2.),
                        mesh: meshes.add(Cuboid::new(2.0, height * 4., 2.0)),
                        material: materials.add(Color::rgb(1., 1., 1.)),
                        ..default()
                    },
                    RigidBody::Fixed,
                    HideFarAway,
                    Collider::cuboid(1., height * 2., 1.),
                    Name::new(format!("Cell {}:{}", x, y))
                ));
            });
        }
    }

    game.map = finished_cells;

}