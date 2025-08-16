use bevy::prelude::*;

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_menu(&mut commands, &asset_server);
}
pub fn despawn_menu() {
    todo!();
}

pub fn build_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let menu_entity = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default
        })
        .id();

    menu_entity
}
