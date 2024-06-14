use bevy::prelude::*;

mod player;
pub use player::*;

mod obstacle;
pub use obstacle::*;


mod hud;
pub use hud::*;

mod pause_menu;
pub use pause_menu::*;

use crate::{AppState, SimulationState};

// ==== PLUGIN ====

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        //
        .add_systems(OnEnter(AppState::Game), start_running)
        .add_plugins((PlayerPlugin,ObstaclePlugin,HudPlugin,PauseMenuPlugin))
        //
        //.add_systems(Update, test_collision)
        //
        ;
    }
}

// ==== COMPONENTS ====
#[derive(Component, Default)]
pub struct Speed(pub Vec3);

#[derive(Component, Default)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Default)]
pub struct GravityCounter(pub u32);

#[derive(Component, Default)]
pub struct Size(pub Vec2);

#[derive(Component, Default)]
pub struct CollisionSides(pub [bool;4]);

#[derive(Component, Default)]
/// JumpLock set to true disallows jumping
pub struct JumpLock(pub bool);

// ==== FUNCTIONS ====

pub fn collide(p1: Vec2, s1: Vec2, p2: Vec2, s2: Vec2) -> bool {
    // move center to top left to simplify overlap check
    let p1 = Vec2::new(p1.x-s1.x/2.0,p1.y-s1.y/2.0);
    let p2 = Vec2::new(p2.x-s2.x/2.0,p2.y-s2.y/2.0);
    // simple overlap check
    ((p2.x-1.0<=p1.x && p1.x<=p2.x+s2.x+1.0) || 
     (p1.x-1.0<=p2.x && p2.x<=p1.x+s1.x+1.0)) 
   && 
   ((p2.y-1.0<=p1.y && p1.y<=p2.y+s2.y+1.0) || 
    (p1.y-1.0<=p2.y && p2.y<=p1.y+s1.y+1.0)) 
}

// ==== SYSTEMS =====

// pub fn test_collision(
//     player_query: Query<(&Transform,&Size), With<PlayerComponent>>,
//     obstacle_query: Query<(&Transform,&Size), With<ObstacleComponent>>
// ) {
//     if let Ok((pt,ps)) = player_query.get_single() {
//         if let Ok((ot,os)) = obstacle_query.get_single() {
//             println!("Collision: {}",collide(
//                 pt.translation.xy(),ps.0.xy(),ot.translation.xy(),os.0.xy(),
//             ));
//         }
//     }
// }

pub fn start_running(
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    simulation_state_next_state.set(SimulationState::Running)
}