use bevy::prelude::*;

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_menu(&mut commands, &asset_server);
}
pub fn despawn_menu() {
    todo!();
}

pub fn build_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Main Menu Title
            parent.spawn((
                Text::new("Main Menu"),
                TextFont {
                    font_size: 48.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..Default::default()
                },
            ));
        })
        .id();

    menu_entity
}
