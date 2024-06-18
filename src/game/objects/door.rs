use bevy::prelude::*;

use crate::{game::{collide, PlayerComponent, Size}, AppState, SimulationState};

use super::coin::Score;

#[derive(Component)]
pub struct DoorComponent;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app
        //.
        .add_systems(Update, 
            door_player_collide
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        //.
        ;
    }
}

fn door_player_collide(
    mut player_query: Query<(&mut Transform, &Size), With<PlayerComponent>>,
    coin_query: Query<(&Transform, &Size), (With<DoorComponent>, Without<PlayerComponent>)>,
    score_resource: Res<Score>,
) {
    if let Ok((pt, ps)) = player_query.get_single_mut() {
        for (ot, os) in coin_query.iter() {
            if collide(pt.translation.xy(), ps.0, ot.translation.xy(), os.0)
                && score_resource.current >= score_resource.needed // greater or equal allows for levels with variable paths
            {
                println!("Level Won");
                // TODO: Implement proper level switching once loading is done
            }
        }
    }
}
