use bevy::prelude::*;
mod systems;

use systems::layout::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    LevelEditor,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelEditor), spawn_menu)
            .add_systems(OnExit(GameState::LevelEditor), despawn_menu);
    }
}

pub fn menu() {
    todo!();
}
