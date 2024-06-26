use std::{
    fmt::Debug,
    fs::{self, read_dir, DirEntry, File},
    io::Read,

};

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    game::{
        coin::{CoinComponent, Score}, door::DoorComponent, obstacle::ObstacleComponent, spawn_player,
        PlayerComponent, Size, Speed,
    },
    AppState,
};

#[allow(unused)]
pub const COIN_SIZE: f32 = 20.0;

#[allow(unused)]
pub const PLAYER_SIZE: f32 = 50.0;

#[allow(unused)]
pub const LEGACY_SCALE: f32 = 2.0;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<LoadLevelEvent>()
            // systems
            .add_systems(OnEnter(AppState::Game), test_loading.after(spawn_player))
            .add_systems(Update, handle_loadlevelevent.run_if(in_state(AppState::Game)))
            .add_systems( OnExit(AppState::Game), despawn_level)
            //.
            ;
    }
}

// ==== EVENT & EVENT HANDLING HANDLING ====


#[derive(Event)]
pub struct LoadLevelEvent {
    path: String,
}

pub fn handle_loadlevelevent (
    mut event_read: EventReader<LoadLevelEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&mut Transform, &mut Speed), With<PlayerComponent>>,
    level_query: Query<Entity, With<Level>>,
    mut score_resource: ResMut<Score>
) {
    if let Some(e) = event_read.read().last() {
        if let Ok(level_entity) = level_query.get_single() {
            println!("[LOADER] Despawning prior level");
            commands.entity(level_entity).despawn_recursive();
        }
        if let Ok((mut transform, mut speed)) = player_query.get_single_mut() {
            println!("[LOADER] Starting loading of test level ./assets/levels/og4/0");
            let coin_count = spawn_level(
                e.path.clone(),
                commands,
                &mut meshes,
                &mut materials,
                &mut transform,
                &mut speed,
            );
            score_resource.needed = coin_count;
            println!("Level score requirement (score.needed) set to {}",score_resource.needed);
        } else {
            println!("[LOADER] Couldn't load a level: No player entity");
        };
    }
}

// ==== SYSTEMS ====

pub fn test_loading(
    mut event_writer: EventWriter<LoadLevelEvent>,
) {
    event_writer.send(LoadLevelEvent {
        path: "./assets/levels/og4/0".to_string()
    });
}

// pub fn load_level(
//     path: String,
//     mut commands: &mut Commands,
//     mut meshes: &mut ResMut<Assets<Mesh>>,
//     mut materials: &mut ResMut<Assets<ColorMaterial>>,
//     mut player_query: &mut Query<(&mut Transform, &mut Speed), With<PlayerComponent>>,
//     level_query: & Query<Entity, With<Level>>,
//     mut score_resource: &mut ResMut<Score>
//     //keyboard: Res<ButtonInput<KeyCode>>,
// ) {

// }

// ==== FUNCTIONS ====
/// struct responsible for holding temporary data
pub enum LevelObject {
    /// pos and size
    Obstacle((Vec2, Vec2)),
    /// pos
    Coin(Vec2),
    /// pos and size
    Door(Vec2,Vec2),
    /// pos
    PlayerPos(Vec2),
}

impl Default for LevelObject {
    fn default() -> Self {
        LevelObject::Obstacle((Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)))
    }
}

/// function responsible for loading given file into a deployable data structure.
pub fn load_level_data(
    path: String,
    parser: &dyn Fn(&str, usize) -> Option<LevelObject>,
) -> Vec<LevelObject> {
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
    if buff.lines().count() > 1 {
        println!("Loading data into vector");
        for (n, l) in buff.lines().enumerate() {
            println!("{}", l);
            match parser(l, n) {
                None => {}
                Some(o) => ret.push(o),
            };
        }
    } else {
        println!("Cannot load data");
    }
    println!("Loaded {} objects", ret.len());
    ret
}

#[derive(Component)]
pub struct Level;

/// spawn level object with objects from a given file
pub fn spawn_level(
    path: String,
    mut commands: Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_transform: &mut Transform,
    player_speed: &mut Speed,
) -> usize {
    let mut coin_count = 0;
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
                material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.0)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Level
        ))
        .with_children(|parent| {
            let data = load_level_data(path, &legacy_loading::parse_line);
            for o in data {
                match o {
                    LevelObject::Obstacle((pos, size)) => {
                        parent.spawn((
                            MaterialMesh2dBundle {
                                mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
                                material: materials.add(Color::WHITE),
                                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                                ..default()
                            },
                            ObstacleComponent::default(),
                            Size { 0: size },
                        ));
                    }
                    LevelObject::Coin(pos) => {
                        parent.spawn((
                            MaterialMesh2dBundle {
                                mesh: Mesh2dHandle(meshes.add(Circle::new(COIN_SIZE))),
                                material: materials.add(Color::YELLOW),
                                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                                ..default()
                            },
                            CoinComponent,
                            Size {
                                0: Vec2::new(COIN_SIZE * 2.0, COIN_SIZE * 2.0),
                            },
                        ));
                        coin_count+=1;
                    }
                    LevelObject::Door(pos, size) => {
                        parent.spawn((
                            MaterialMesh2dBundle {
                                mesh: Mesh2dHandle(
                                    meshes.add(Rectangle::new(size.x, size.y)),
                                ),
                                material: materials.add(Color::ORANGE),
                                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                                ..default()
                            },
                            DoorComponent,
                            Size {
                                0: size,
                            },
                        ));
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
        coin_count
}

pub struct LevelData {
    name: String,
    author: u64,
    amount: u8,
}

impl LevelData {
    pub fn read_dir(path: &DirEntry) -> Option<LevelData> {
        let mut fs = File::open(format!("{}/info", path.path().display())).unwrap();
        let mut buff = "".to_string();
        fs.read_to_string(&mut buff).expect("FU2");
        match buff.lines().into_iter().next() {
            Some(o) => {
                let mut oo = o.split_whitespace().into_iter();
                oo.next().unwrap();
                return Some(LevelData {
                    amount: 0,
                    author: 0,
                    name: oo.next().expect("F").to_string(),
                });
            }
            None => {
                return None;
            }
        }
    }
}

impl Debug for LevelData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.author)
            .field(&self.name)
            .field(&self.amount)
            .finish()
    }
}

pub const PATH_LEVELS: &str = "./assets/levels/";

pub fn get_levels_data() -> Vec<LevelData> {
    let mut n = Vec::new();
    read_dir(PATH_LEVELS).expect("FU1").for_each(|x| {
        match LevelData::read_dir(&x.as_ref().unwrap()) {
            Some(o) => n.push(o),
            None => {
                println!("Failed to load {}", &x.unwrap().path().display())
            }
        };
    });
    n
}

mod legacy_loading {
    use super::{LevelObject, COIN_SIZE, LEGACY_SCALE, PLAYER_SIZE};
    use bevy::prelude::*;

    pub fn parse_line(text: &str, number: usize) -> Option<LevelObject> {
        let segs: Vec<&str> = text.split_ascii_whitespace().collect();
        if segs.len() == 0 {
            return None;
        }
        match segs[0] {
            "BARRIER" | "OBSTACLE" => {
                let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(), segs[2].parse().unwrap());
                let size: Vec2 = Vec2::new(segs[3].parse().unwrap(), segs[4].parse().unwrap());
                //println!("Obstacle -> pos:{}, size:{}",fix_aligment(pos, size),size);
                Some(LevelObject::Obstacle((fix_aligment(pos, size), 2.0 * size)))
            }
            "COIN" => {
                let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(), segs[2].parse().unwrap());
                Some(LevelObject::Coin(fix_aligment(
                    pos,
                    Vec2::new(20.0,20.0),
                )))
            }
            "DOOR" => {
                let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(), segs[2].parse().unwrap());
                let size: Vec2 = Vec2::new(segs[3].parse().unwrap(), segs[4].parse().unwrap());
                Some(LevelObject::Door(fix_aligment(
                    pos,
                    size
                ),
                    size*2.0
                ))
            }
            "PLAYER_POS" => {
                let pos: Vec2 = Vec2::new(segs[1].parse().unwrap(), segs[2].parse().unwrap());
                Some(LevelObject::PlayerPos(fix_aligment(
                    pos,
                    Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
                )))
            }
            "PLAYER_SIZE" => {
                println!("  > Identifier 'PLAYER_SIZE' is redundant for new version");
                None
            }
            "COIN_SIZE" => {
                println!("  > Identifier 'COIN_SIZE' is redundant for new version");
                None
            }
            _ => {
                println!(
                    "  > Error at line number {}: Wrong object identifier",
                    number
                );
                None
            }
        }
    }

    /// fixes alignment issues caused by centering of pos vec by bevy<br>
    /// also transforms coords to fit new window size and coord system (flipped OY)
    pub fn fix_aligment(pos: Vec2, size: Vec2) -> Vec2 {
        Vec2::new(
            LEGACY_SCALE * (pos.x + size.x / 2.0),
            -LEGACY_SCALE * (pos.y + size.y / 2.0) + 960.0,
        )
    }
}

pub fn despawn_level(mut commands: Commands, objects_query: Query<Entity, With<Level>>) {
    for obj in objects_query.iter() {
        commands.entity(obj).despawn_recursive()
    }
}
