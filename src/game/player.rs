use super::{collide, Acceleration, CollisionSides, GravityCounter, JumpLock, ObstacleComponent, Size, Speed};
use crate::{data::mymath::reduction, AppState, SimulationState};
use bevy::{
    audio::Volume, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use rand::prelude::*;



// ==== Constants ====
pub const PLAYER_DECELERATION_RATE: f32 = 0.8;
pub const PLAYER_FRICTION_RATE: f32 = 0.8;
pub const PLAYER_ACCELERATION: f32 = 1.0;
pub const PLAYER_JUMP_STRENGTH: f32 = 7.0;
pub const PLAYER_JUMP_TIME: u32 = 20;
pub const PLAYER_MASS: f32 = 1.0;
pub const VOLUME_DETERMINATION_BASE: f32 = 20.0;

// ==== PLUGIN ====

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        // Event
        .add_event::<PlayerInput>()
            // On Enter
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Update
            .add_systems(
                Update,
                (handle_player_keyboard)
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
            )
            // FixedUpdate
            .add_systems(
                FixedUpdate,
                (handle_player_input,(update_player_physics,handle_player_obstacle_collision).chain())
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // On Exit
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
// ==== SYSTEMS =====
// === SPAWNING AND BASE ====

#[derive(Component, Default)]
pub struct PlayerComponent;

#[derive(Event)]
pub struct PlayerInput(PlayerAction);

pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    JumpStart,
    JumpEnd
}

/// Spawn player
pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0))),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0)),
            transform: Transform::from_xyz(600.0, 300.0, 0.0),
            ..default()
        },
        PlayerComponent::default(),
        Speed::default(),
        Acceleration::default(),
        GravityCounter::default(),
        Size{0:Vec2::new(50.0,50.0)},
        CollisionSides::default(),
        JumpLock::default()
    ));
}

/// Despawn player
pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<PlayerComponent>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

// ==== Movement ====

/// Perform all physics calculations
pub fn update_player_physics(
    mut player_query: Query<(&mut Transform, Entity), With<PlayerComponent>>,
    mut player_spd: Query<(&mut Speed,&mut CollisionSides,&mut GravityCounter), With<PlayerComponent>>,
    mut player_acc: Query<&mut Acceleration, With<PlayerComponent>>,
) {
    if let Ok((mut player_speed, mut player_sides, mut player_gravity)) = player_spd.get_single_mut() {
        // 1. apply gravity unless on the ground (collision from below) or counteracted (gravity timer for jumping)
        if !(player_sides.0[0] || player_gravity.0>0){
            player_speed.0.y -= PLAYER_MASS;
        }
        // Step 2 is handled withing system handle_player_input
        // Step 3 is handled in fn handle_player_input
        // 4. decrease gravty counteraction time by 1 if >0
        if player_gravity.0 > 0 {
            player_gravity.0 -= 1;
        }
        //
        if let Ok(mut player_acceleration) = player_acc.get_single_mut() {
            // 5. apply acceleration to speed on x axis while limiting value withing borders
            if (player_acceleration.0.x<0.0 && !player_sides.0[2]) || (player_acceleration.0.x>0.0 && !player_sides.0[3]) {
                player_speed.0.x += player_acceleration.0.x;
            }
            // 6. apply acceleration to speed on y axis
            player_speed.0.y += player_acceleration.0.y;
            // 7. reset acceleration
            player_acceleration.0.x = 0.0;
            player_acceleration.0.y = 0.0;
        }
        // 8. clamp speed & reduce speed on x axis
        match player_sides.0[0] { // further reduction if on the ground
            true => {player_speed.0.x = reduction(player_speed.0.x, PLAYER_DECELERATION_RATE*PLAYER_FRICTION_RATE, 0.5);},
            false => {player_speed.0.x = reduction(player_speed.0.x, PLAYER_DECELERATION_RATE, 0.5);}
        };
        //player_speed.0.y = reduction(player_speed.0.y, PLAYER_DECELERATION_RATE, 0.5);
        
        // 9. restart gravity (set counteraction time to 0) if jump is cancelled 

        // in the old code, applying speed to position was in the general update method for player struct
        if let Ok((mut player_transform, _player_entity)) = player_query.get_single_mut() {
            player_transform.translation.x += player_speed.0.x;
            player_transform.translation.y += player_speed.0.y;
        }
        // reset collision sides
        player_sides.0[0]=false;
        player_sides.0[1]=false;
        player_sides.0[2]=false;
        player_sides.0[3]=false;
    }
}


/// Apply player input to acceleration
pub fn handle_player_keyboard(
    mut event_writer: EventWriter<PlayerInput>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    // 2. apply acceleration on x axis by controller
    // Accelerate left
    if keyboard_input.pressed(KeyCode::KeyA) {
        event_writer.send(PlayerInput(PlayerAction::MoveLeft));
    }
    // Accelerate right
    if keyboard_input.pressed(KeyCode::KeyD) {
        event_writer.send(PlayerInput(PlayerAction::MoveRight));
    }
    // 3. apply jump-related acceleration on y axis & grant counteraction time
    if keyboard_input.just_pressed(KeyCode::Space) {
        event_writer.send(PlayerInput(PlayerAction::JumpStart));
    }
    if keyboard_input.just_released(KeyCode::Space) {
        event_writer.send(PlayerInput(PlayerAction::JumpEnd));
    }
}

// pub fn no_grav(
//     mut player_gravity: Query<&mut GravityCounter, With<PlayerComponent>>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {

// }

pub fn handle_player_input (
    mut event_reader: EventReader<PlayerInput>,
    mut player_attrib: Query<(&mut Acceleration,&mut JumpLock,&mut GravityCounter), With<PlayerComponent>>,
) {
    if let Ok((mut player_acc,mut player_jumplock,mut player_grav)) = player_attrib.get_single_mut() {
        for event in event_reader.read() {
            match event.0 {
                PlayerAction::MoveLeft => {
                    player_acc.0.x -= PLAYER_ACCELERATION;
                },
                PlayerAction::MoveRight => {
                    player_acc.0.x += PLAYER_ACCELERATION;
                },
                PlayerAction::JumpStart => {
                if !player_jumplock.0 {
                    // apply jump acceleration
                    player_acc.0.y += PLAYER_JUMP_STRENGTH;
                    // lock jump (it gets unlocked at collision with ground, within fn player_obstacle_collision)
                    player_jumplock.0 = true;
                    // set gravity to jump time
                    player_grav.0 = PLAYER_JUMP_TIME;
                };
                },
                PlayerAction::JumpEnd => {
                    player_grav.0 = 0;
                },
            }
        }   
    }

}

/// 
pub fn handle_player_obstacle_collision(
    mut player_query: Query<(&mut Transform, &mut Speed,& Size,&mut CollisionSides,
         &mut JumpLock, &mut GravityCounter), With<PlayerComponent>>,
    obstacle_query: Query<(&Transform, &Size), (With<ObstacleComponent>, Without<PlayerComponent>)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    if let Ok((mut pt,mut psd, ps,mut cs,
         mut pj,mut gc)) = player_query.get_single_mut() {
    for (ot,os) in obstacle_query.iter() {
        if collide(pt.translation.xy(), ps.0, ot.translation.xy(), os.0) {
            player_obstacle_collision(
                &mut pt,&mut psd, ps, 
                &mut cs, 
                &mut pj, &mut gc, ot, os, &mut commands, &asset_server
            )
        }
    }
}
}

///
pub fn player_obstacle_collision(
    player_transform: &mut Transform,
    player_speed: &mut Speed,
    player_size: &Size,
    collision_sides: &mut CollisionSides,
    jump_lock: &mut JumpLock,
    gravity_counter: &mut GravityCounter,
    obstacle_transform: &Transform,
    obstacle_size: &Size,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>
) {
    let mut mk = 32;
    let mut _mv = f32::MAX;
    let mut d = (0.0,0.0,0.0,0.0);
    // distance between player's bottom and obstacle's top
    d.0 = (player_transform.translation.y-player_size.0.y/2.0-(obstacle_transform.translation.y+obstacle_size.0.y/2.0)).abs();
    // distance between player's top and obstacle's bottom
    d.1 = (player_transform.translation.y+player_size.0.y/2.0-(obstacle_transform.translation.y-obstacle_size.0.y/2.0)).abs();
    // distance between player's left and obstacle's right
    d.2 = (player_transform.translation.x-player_size.0.x/2.0-(obstacle_transform.translation.x+obstacle_size.0.x/2.0)).abs();
    // distance between player's right and obstacle's left
    d.3 = (player_transform.translation.x+player_size.0.x/2.0-(obstacle_transform.translation.x-obstacle_size.0.x/2.0)).abs();
    //println!("{:?}",d);
    if d.0<_mv {mk=0;_mv=d.0};
    if d.1<_mv {mk=1;_mv=d.1};
    if d.2<_mv {mk=2;_mv=d.2};
    if d.3<_mv {mk=3;_mv=d.3};
    match mk {
        0 => {
            // Collision with a floor
            //println!("floor");
            player_transform.translation.y = obstacle_transform.translation.y+player_size.0.y/2.0+obstacle_size.0.y/2.0;
            play_impact(&mut commands,&asset_server,Volume::new(player_speed.0.y.abs()/VOLUME_DETERMINATION_BASE));
            jump_lock.0 = false;
            if player_speed.0.y<0.0 {player_speed.0.y=0.0};
        },
        1 => {
            // Collision with the ceiling
            //println!("ceiling");
            player_transform.translation.y = obstacle_transform.translation.y-player_size.0.y/2.0-obstacle_size.0.y/2.0;
            play_impact(&mut commands,&asset_server,Volume::new( player_speed.0.y.abs()/VOLUME_DETERMINATION_BASE));
            if player_speed.0.y>0.0 {player_speed.0.y=0.0};
            gravity_counter.0 = 0;
        },
        2 => {
            // Collision on the left (player on the right of obstacle)
            //println!("left");
            player_transform.translation.x = obstacle_transform.translation.x+player_size.0.x/2.0+obstacle_size.0.x/2.0;
            //play_impact(&mut commands,&asset_server,Volume::new(-player_speed.0.x/VOLUME_DETERMINATION_BASE));
            if player_speed.0.x<0.0 {player_speed.0.x=0.0};
        },
        3 => {
            // Collision on the right (player on the left of obstacle)
            //println!("right");
            player_transform.translation.x = obstacle_transform.translation.x-player_size.0.x/2.0-obstacle_size.0.x/2.0;
            //play_impact(&mut commands,&asset_server,Volume::new( player_speed.0.x/VOLUME_DETERMINATION_BASE));
            if player_speed.0.x>0.0 {player_speed.0.x=0.0};
        },
        _ => {println!("At player_obstacle_collision, somehow an impossible collision side key was matched...");}
    }
    collision_sides.0[mk]=true;
    //println!("{:?}",collision_sides.0);
}

pub fn play_impact(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    volume: Volume
) {
        // let sound = if random::<f32>() > 0.5 {
        //     asset_server.load("audio/impactGeneric_light_000.ogg")
        // } else {
        //     asset_server.load("audio/impactGeneric_light_001.ogg")
        // };
        let sound: [&str;5] = [
            "audio/impactGeneric_light_000.ogg",
            "audio/impactGeneric_light_001.ogg",
            "audio/impactGeneric_light_002.ogg",
            "audio/impactGeneric_light_003.ogg",
            "audio/impactGeneric_light_004.ogg"
        ];
        let sound = sound.choose(&mut rand::thread_rng()).unwrap().to_owned();
        commands.spawn(AudioBundle {
            source: asset_server.load(sound),
            settings: PlaybackSettings::DESPAWN.with_volume(volume),
        });
}