use std::{fs, io::Read};

use bevy::{ prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{game::{ObstacleComponent, PlayerComponent, Size, Speed}, AppState};

pub const COIN_SIZE: f32 = 20.0;
pub const DOOR_WIDTH: f32 = 20.0;
pub const DOOR_HEIGHT: f32 = 60.0;
pub const PLAYER_SIZE: f32 = 50.0;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_systems(Update, test_loading.run_if(in_state(AppState::Game)))
        //
        ;
    }
}


// ==== SYSTEMS ====

pub fn test_loading (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&mut Transform,&mut Speed), With<PlayerComponent>>,
    level_query: Query<Entity, With<Level>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        if let Ok(level_entity) = level_query.get_single() {
            commands.entity(level_entity).despawn_recursive();
        }
        if let Ok((mut transform, mut speed)) = player_query.get_single_mut() {
            load_level("./assets/levels/og4/1".to_string(),commands,&mut meshes,&mut materials, &mut transform, &mut speed);
        }else{
            println!("  Couldn't load a level: No player entity");
        }
    }
}


// ==== FUNCTIONS ====
/// struct responsible for holding temporary data
pub enum LevelObject {
    /// pos and size
    Obstacle((Vec2, Vec2)),
    /// pos
    Coin(Vec2),
    /// pos
    Door(Vec2),
    /// pos
    PlayerPos(Vec2)
}

impl Default for LevelObject {
fn default() -> Self {
    LevelObject::Obstacle((Vec2::new(0.0,0.0),Vec2::new(0.0,0.0)))
}
}

/// function responsible for loading given file into a deployable data structure.
pub fn load_level_data(path: String) -> Vec<LevelObject> {
    let mut buff: String = String::new();
    let mut file = fs::File::open(&path);
    match &mut file {
        Ok(f) => {
            match f.read_to_string(&mut buff) {
                Ok(_o) => {
                    println!("Loaded data from {}", &path);
                }
                Err(e) => {
                    println!("Error loading data from {} due {}", &path, e);
                }
            };
        }
        Err(e) => {
            println!("Error opening file {} due {:?}", &path, e);
        }
    }
    let mut ret = Vec::new();
    if buff.lines().count()>1 {
        println!("Loading data into vector");
        for (n,l) in buff.lines().enumerate() {
            println!("{}",l);
            match parse_line(l,n) {
                None => {},
                Some(o) => ret.push(o)
            };
        }
    } else {
        println!("Cannot load data");
    }
    println!("Loaded {} objects",ret.len());
    ret
}

pub fn parse_line(text:&str,number:usize) -> Option<LevelObject> {
    let segs:Vec<&str> = text.split_ascii_whitespace().collect();
    if segs.len()==0 {return None;}
    match segs[0] {
        "BARRIER" | "OBSTACLE" => {
            let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(),segs[2].parse().unwrap());
            let size: Vec2 = Vec2::new(segs[3].parse().unwrap(),segs[4].parse().unwrap());
            Some(LevelObject::Obstacle((fix_aligment(pos,size),size)))
        },
        "COIN" => {
            let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(),segs[2].parse().unwrap());
            Some(LevelObject::Coin(fix_aligment(pos,Vec2::new(COIN_SIZE*2.0,COIN_SIZE*2.0))))
        },
        "DOOR" => {
            let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(),segs[2].parse().unwrap());
            Some(LevelObject::Door(fix_aligment(pos,Vec2::new(DOOR_WIDTH,DOOR_HEIGHT))))
        },
        "PLAYER_POS" => {
            let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(),segs[2].parse().unwrap());
            Some(LevelObject::PlayerPos(fix_aligment(pos,Vec2::new(PLAYER_SIZE,PLAYER_SIZE))))
        },
        "PLAYER_SIZE" => {
            println!("  > Identifier 'PLAYER_SIZE' is redundant for new version");
            None
        },
        "COIN_SIZE" => {
            println!("  > Identifier 'COIN_SIZE' is redundant for new version");
            None
        }
        _ => {
            println!("  > Error at line number {}: Wrong object identifier",number);
            None
        }
    }
    
}

#[derive(Component)]
pub struct Level;



/// spawn level object with objects from a given file
pub fn load_level(
    path: String, 
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_transform: &mut Transform,
    player_speed: &mut Speed
) {
    commands.spawn(
        (MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0,0.0)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },Level)
    ).with_children(|parent| {
        let data = load_level_data(path);
        for o in data {
            match o {
                LevelObject::Obstacle((pos,size)) => {
                    parent.spawn(
                        (MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
                            material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
                            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                            ..default()
                        },
                        ObstacleComponent::default(),
                        Size{0:size}
                        ));
                }
                LevelObject::Coin(pos) => {
                    (MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Circle::new(COIN_SIZE))),
                        material: materials.add(Color::YELLOW),
                        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                        ..default()
                    },
                    ObstacleComponent::default(),
                    Size{0:Vec2::new(COIN_SIZE/2.0,COIN_SIZE/2.0)});
                }
                LevelObject::Door(pos) => {
                    (MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(DOOR_WIDTH, DOOR_HEIGHT))),
                        material: materials.add(Color::ORANGE),
                        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                        ..default()
                    },
                    ObstacleComponent::default(),
                    Size{0:Vec2::new(DOOR_WIDTH,DOOR_HEIGHT)});
                }
                LevelObject::PlayerPos(pos) => {
                    player_transform.translation.x = pos.x;
                    player_transform.translation.y = pos.y;
                    player_speed.0.x = 0.0;
                    player_speed.0.y = 0.0;
                }
            }
        }
    });
}

/// fixes alignment issues caused by centering of pos vec by bevy
pub fn fix_aligment (
    pos: Vec2,
    size: Vec2
) -> Vec2 {
    Vec2::new(
        pos.x-size.x/2.0,
        pos.y-size.y/2.0
    )
}