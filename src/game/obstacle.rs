use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use super::Size;

// ==== PLUGIN ====

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app
        // testing
        .add_systems(Startup, spawn_test_level)
        //
        ;
    }
}

// ==== COMPONENTS ====

#[derive(Component,Default)]
pub struct ObstacleComponent;

// ==== SYSTEMS ====

pub fn spawn_obstacle(
    pos: Vec2,
    size: Vec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        },
        ObstacleComponent::default(),
        Size{0:size}
    ));
}

pub fn spawn_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_obstacle(Vec2::new(350.0,50.0), Vec2::new(600.0,50.0), &mut commands, &mut meshes, &mut materials);
    spawn_obstacle(Vec2::new(350.0,100.0), Vec2::new(100.0,10.0), &mut commands, &mut meshes, &mut materials);
}