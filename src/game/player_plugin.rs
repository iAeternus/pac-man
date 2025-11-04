use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::state::OnEnter,
};

use crate::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (handle_player_input, update_player_position).run_if(GameState::Playing),
            );
    }
}

fn spawn_player() {}

fn handle_player_input() {}

fn update_player_position() {}
