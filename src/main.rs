use bevy::prelude::*;
use bevy_mouse_position::MousePositionPlugin;

use pancam::{PanCam, PanCamPlugin};
// use command_defender::CommandDefenderPlugin;
use select_area::SelectAreaPlugin;
use solar_system::SolarSystemPlugin;

// mod command_defender;
mod pancam;
mod select_area;
mod solar_system;

const WINDOW_TITLE: &str = "Commander Defender";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        grab_buttons: vec![MouseButton::Middle],
        min_scale: 1.,
        max_scale: Some(10.),
        ..default()
    });
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
        .add_plugin(PanCamPlugin::default())
        .add_plugin(MousePositionPlugin)
        // .add_plugin(CommandDefenderPlugin)
        .add_plugin(SelectAreaPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(spawn_camera)
        .add_plugin(SolarSystemPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
