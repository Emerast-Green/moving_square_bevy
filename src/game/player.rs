use super::{collide, Acceleration, CollisionSides, GravityCounter, JumpLock, ObstacleComponent, Size, Speed};
use crate::{data::mymath::reduction, AppState, SimulationState};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

// ==== Constants ====
pub const PLAYER_DECELERATION_RATE: f32 = 0.8;
pub const PLAYER_ACCELERATION: f32 = 1.5;
pub const PLAYER_JUMP_STRENGTH: f32 = 10.0;
pub const PLAYER_JUMP_TIME: u32 = 25;
pub const PLAYER_MASS: f32 = 2.5;

// ==== PLUGIN ====

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::Game), spawn_player)
            // Update
            .add_systems(
                Update,
                (handle_player_input)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // FixedUpdate
            .add_systems(
                FixedUpdate,
                (handle_player_obstacle_collision,update_player_physics).chain()
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
    mut player_spd: Query<(&mut Speed,&mut CollisionSides), With<PlayerComponent>>,
    mut player_acc: Query<&mut Acceleration, With<PlayerComponent>>,
    mut player_grv: Query<&mut GravityCounter, With<PlayerComponent>>,
) {
    if let Ok((mut player_speed, mut player_sides)) = player_spd.get_single_mut() {
        // 1. apply gravity unless on the ground (collision from below) or counteracted (gravity timer for jumping)
        if !player_sides.0[0] {
            player_speed.0.y -= PLAYER_MASS;
        }
        // Step 2 is handled withing system handle_player_input
        // Step 3 is handled in fn handle_player_input
        // 4. decrease gravty counteraction time by 1 if >0
        if let Ok(mut player_gravity) = player_grv.get_single_mut() {
            if player_gravity.0 > 0 {
                player_gravity.0 -= 1;
            }
        }
        //
        if let Ok(mut player_acceleration) = player_acc.get_single_mut() {
            // 5. apply acceleration to speed on x axis while limiting value withing borders
            player_speed.0.x += player_acceleration.0.x;
            // 6. apply acceleration to speed on y axis
            player_speed.0.y += player_acceleration.0.y;
            // 7. reset acceleration
            player_acceleration.0.x = 0.0;
            player_acceleration.0.y = 0.0;
        }
        // 8. clamp speed & reduce speed on x axis
        player_speed.0.x = reduction(player_speed.0.x, PLAYER_DECELERATION_RATE, 0.5);
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
pub fn handle_player_input(
    mut player_acc: Query<&mut Acceleration, With<PlayerComponent>>,
    mut player_jump: Query<&mut JumpLock, With<PlayerComponent>>,
    mut player_gravity: Query<&mut GravityCounter, With<PlayerComponent>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // 2. apply acceleration on x axis by controller
    if let Ok(mut player_acceleration) = player_acc.get_single_mut() {
        // Accelerate left
        if keyboard_input.pressed(KeyCode::KeyA) {
            player_acceleration.0.x -= PLAYER_ACCELERATION;
        }
        // Accelerate right
        if keyboard_input.pressed(KeyCode::KeyD) {
            player_acceleration.0.x += PLAYER_ACCELERATION;
        }
        // 3. apply jump-related acceleration on y axis & grant counteraction time
        if keyboard_input.pressed(KeyCode::Space) && !player_jump.get_single().unwrap().0{
            // apply jump acceleration
            player_acceleration.0.y += PLAYER_JUMP_STRENGTH;
            // lock jump (it gets unlocked at collision with ground, within fn player_obstacle_collision)
            player_jump.get_single_mut().unwrap().0 = true;
            // set gravity to jump time
            player_gravity.get_single_mut().unwrap().0 = PLAYER_JUMP_TIME;
        }
    }
}

/// 
pub fn handle_player_obstacle_collision(
    mut player_query: Query<(&mut Transform, &mut Speed,& Size,&mut CollisionSides,
         &mut JumpLock, &mut GravityCounter), With<PlayerComponent>>,
    obstacle_query: Query<(&Transform, &Size), (With<ObstacleComponent>, Without<PlayerComponent>)>
) {
    if let Ok((mut pt,mut psd, ps,mut cs,
         mut pj,mut gc)) = player_query.get_single_mut() {
    for (ot,os) in obstacle_query.iter() {
        if collide(pt.translation.xy(), ps.0, ot.translation.xy(), os.0) {
            player_obstacle_collision(
                &mut pt,&mut psd, ps, 
                &mut cs, 
                &mut pj, &mut gc, ot, os 
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
) {
    let mut mk = 32;
    let mut mv = f32::MAX;
    let mut d = (0.0,0.0,0.0,0.0);
    // distance between player's bottom and obstacle's top
    d.0 = (player_transform.translation.y-player_size.0.y/2.0-(obstacle_transform.translation.y+obstacle_size.0.y/2.0)).abs();
    // distance between player's top and obstacle's bottom
    d.1 = (player_transform.translation.y+player_size.0.y/2.0-(obstacle_transform.translation.y-obstacle_size.0.y/2.0)).abs();
    // distance between player's left and obstacle's right
    d.2 = (player_transform.translation.x-player_size.0.x/2.0-(obstacle_transform.translation.x+obstacle_size.0.x/2.0)).abs();
    // distance between player's right and obstacle's left
    d.3 = (player_transform.translation.x+player_size.0.x/2.0-(obstacle_transform.translation.x-obstacle_size.0.x/2.0)).abs();
    println!("{:?}",d);
    if d.0<mv {mk=0;mv=d.0};
    if d.1<mv {mk=1;mv=d.1};
    if d.2<mv {mk=2;mv=d.2};
    if d.3<mv {mk=3;mv=d.3};
    match mk {
        0 => {
            // Collision with a floor
            println!("floor");
            player_transform.translation.y = obstacle_transform.translation.y+player_size.0.y/2.0+obstacle_size.0.y/2.0;
            if player_speed.0.y<0.0 {player_speed.0.y=0.0};
            jump_lock.0 = false;
        },
        1 => {
            // Collision with the ceiling
            println!("ceiling");
            player_transform.translation.y = obstacle_transform.translation.y-player_size.0.y/2.0-obstacle_size.0.y/2.0;
            if player_speed.0.y>0.0 {player_speed.0.y=0.0};
            gravity_counter.0 = 0;
        },
        2 => {
            // Collision on the left (player on the right of obstacle)
            println!("left");
            player_transform.translation.x = obstacle_transform.translation.x+player_size.0.x/2.0+obstacle_size.0.x/2.0;
            if player_speed.0.x<0.0 {player_speed.0.x=0.0};
        },
        3 => {
            // Collision on the right (player on the left of obstacle)
            println!("right");
            player_transform.translation.x = obstacle_transform.translation.x-player_size.0.x/2.0-obstacle_size.0.x/2.0;
            if player_speed.0.x>0.0 {player_speed.0.x=0.0};
        },
        _ => {println!("At player_obstacle_collision, somehow an impossible collision side key was matched...");}
    }
    collision_sides.0[mk]=true;
    println!("{:?}",collision_sides.0);
}