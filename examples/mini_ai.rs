use bevy::prelude::*;

const WINDOW_TITLE: &str = "Mini AI";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn spawn_camera(mut commands: Commands) {
    // Without Pan Cam
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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(spawn_camera)
        .add_plugin(MiniAiPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

struct MiniAiPlugin;

impl Plugin for MiniAiPlugin {
    fn build(&self, app: &mut App) {}
}
