use std::default;

use bevy::prelude::*;

use crate::{game::{collide, PlayerComponent, Size}, AppState, SimulationState};


#[derive(Component)]
pub struct CoinComponent;

pub struct CoinPLugin;

impl Plugin for CoinPLugin {
    fn build(&self, app: &mut App) {
        app
        //.
        .add_systems(OnEnter(AppState::Game), insert_coin_resource)  
        .add_systems(Update, 
            coin_player_collide
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
        )    
        .add_systems(OnExit(AppState::Game), remove_coin_resource)
        //.
        ;
    }
}

#[derive(Resource,Default)]
pub struct Score {
    pub current: usize,
    pub needed: usize,
}

fn insert_coin_resource(
    mut commands: Commands
) {
    commands.init_resource::<Score>();
}

fn remove_coin_resource(
    mut commands: Commands
) {
    commands.remove_resource::<Score>();
}

fn coin_player_collide(
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, & Size), With<PlayerComponent>>,
    coin_query: Query<(Entity, &Transform, &Size), (With<CoinComponent>, Without<PlayerComponent>)>,
    mut score_resource: ResMut<Score>,
) {
    if let Ok((pt, ps)) = player_query.get_single_mut() {
    for (oe,ot,os) in coin_query.iter() {
        if collide(pt.translation.xy(), ps.0, ot.translation.xy(), os.0) {
            //TODO play_coin_sound();
            score_resource.current+=1;
            commands.get_entity(oe).unwrap().despawn();
        }
    }
}
}
