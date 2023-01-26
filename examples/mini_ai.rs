use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

const WINDOW_TITLE: &str = "Mini AI";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Component)]
struct Mine;

impl Mine {
    const SIZE: f32 = 24.0;
    const SIDES: usize = 6;
    const BORDER_WIDTH: f32 = 5.0;
    const RADII: f32 = 6.0;
    const COLOR: Color = Color::WHITE;

    fn outline() -> RegularPolygon {
        shapes::RegularPolygon {
            sides: Mine::SIDES,
            feature: shapes::RegularPolygonFeature::Radius(Mine::SIZE),
            ..default()
        }
    }

    fn inner_circle() -> shapes::Circle {
        shapes::Circle {
            radius: Mine::RADII,
            ..default()
        }
    }

    fn spawn(commands: &mut Commands, transform: Transform) -> Entity {
        commands
            .spawn((
                Mine,
                GeometryBuilder::build_as(
                    &Mine::outline(),
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::NONE),
                        outline_mode: StrokeMode::new(Mine::COLOR, Mine::BORDER_WIDTH),
                    },
                    transform,
                ),
            ))
            .with_children(|parent| {
                parent.spawn(GeometryBuilder::build_as(
                    &Mine::inner_circle(),
                    DrawMode::Fill(FillMode::color(Mine::COLOR)),
                    Transform::default(),
                ));
            })
            .id()
    }
}

fn spawn_initial_mines(mut commands: Commands) {
    Mine::spawn(&mut commands, Transform::from_xyz(0., 0., 0.));
    Mine::spawn(&mut commands, Transform::from_xyz(60., -150., 0.));
}

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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugin(ShapePlugin)
        .add_startup_system(spawn_camera)
        .add_plugin(MiniAiPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

struct MiniAiPlugin;

impl Plugin for MiniAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_initial_mines);
    }
}
