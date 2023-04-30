use bevy::prelude::*;

// Make the 'spawn_title_text' function into a plugin for cleaner code
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_title_text);
    }
}

#[derive(Component)]
struct SpawnTitleText;

// This just creates text in the bottom right corner displaying the name of the application
fn spawn_title_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "Bevy Renderer Plus (WIP)",
                TextStyle {
                    font: asset_server.load("fonts/IBMPlexSans-Regular.ttf"),
                    font_size: 42.0,
                    color: Color::BLACK,
                },
            ),
            ..Default::default()
        })
        .insert(SpawnTitleText);
}
