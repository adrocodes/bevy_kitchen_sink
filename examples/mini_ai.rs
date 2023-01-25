use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const WINDOW_TITLE: &str = "Mini AI";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Component)]
struct Mine;

impl Mine {
    const SIZE: f32 = 24.0;
    const SIDES: usize = 7;
    const BORDER_WIDTH: f32 = 3.0;
    const RADII: f32 = 6.0;

    fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        let child_size: f32 = Mine::SIZE - Mine::BORDER_WIDTH;

        let inner_hexagon = MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(child_size, Mine::SIDES).into())
                .into(),
            material: materials.add(ColorMaterial::from(BACKGROUND_COLOR)),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        };

        let inner_circle = MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(Mine::RADII).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
            ..default()
        };

        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(Mine::SIZE, Mine::SIDES).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    ..default()
                },
                Mine,
            ))
            .with_children(|parent| {
                parent.spawn(inner_hexagon);
                parent.spawn(inner_circle);
            });
    }
}

fn spawn_initial_mines(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    Mine::spawn(&mut commands, &mut meshes, &mut materials);
    // Mine::spawn(&commands, &meshes, &materials);
}

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
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_initial_mines);
    }
}
