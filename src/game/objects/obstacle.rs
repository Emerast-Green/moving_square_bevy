use bevy::{
    prelude::*, //sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

// use crate::game::Size;

// ==== PLUGIN ====

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, _app: &mut App) {
        
    }
}

// ==== COMPONENTS ====

#[derive(Component, Default)]
pub struct ObstacleComponent;


// ==== SYSTEMS ====

// pub fn spawn_obstacle(
//     pos: Vec2,
//     size: Vec2,
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<ColorMaterial>>,
// ) {
//     commands.spawn(
//         (
//             MaterialMesh2dBundle {
//                 mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
//                 material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
//                 transform: Transform::from_xyz(pos.x, pos.y, 0.0),
//                 ..default()
//             },
//             ObstacleComponent::default(),
//             Size { 0: size },
//         ),
//     );
// }