use bevy::{prelude::*, winit::WinitSettings};
use bevy_mouse_position::MousePositionPlugin;

use command_defender::CommandDefenderPlugin;
use select_area::SelectAreaPlugin;

mod command_defender;
mod select_area;

const WINDOW_TITLE: &str = "Commander Defender";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                title: WINDOW_TITLE.to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(MousePositionPlugin)
        .add_plugin(CommandDefenderPlugin)
        .add_plugin(SelectAreaPlugin)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(spawn_camera)
        .add_system(bevy::window::close_on_esc)
        .run();
}
