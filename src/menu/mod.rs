use bevy::prelude::*;
mod systems;

use systems::layout::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(State::new(state))), spawn_menu)
            .add_systems(OnExit(State::new(state)), despawn_menu);
    }
}

pub fn menu() {
    todo!();
}
