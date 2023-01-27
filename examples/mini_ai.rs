use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::seq::SliceRandom;

const WINDOW_TITLE: &str = "Mini AI";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Component)]
struct MiningSpot {
    occupied: bool,
}

#[derive(Component)]
struct Mine;

impl Mine {
    const SIZE: f32 = 24.0;
    const SIDES: usize = 6;
    const BORDER_WIDTH: f32 = 5.0;
    const RADII: f32 = 6.0;
    const SPOT_RADII: f32 = 4.0;
    const SPOT_BORDER_WIDTH: f32 = 2.0;
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

    fn mine_spot() -> shapes::Circle {
        shapes::Circle {
            radius: Mine::SPOT_RADII,
            ..default()
        }
    }

    fn spawn(commands: &mut Commands, parent_transform: Transform) -> Entity {
        commands
            .spawn((
                Mine,
                GeometryBuilder::build_as(
                    &Mine::outline(),
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::NONE),
                        outline_mode: StrokeMode::new(Mine::COLOR, Mine::BORDER_WIDTH),
                    },
                    parent_transform,
                ),
            ))
            .with_children(|parent| {
                parent.spawn(GeometryBuilder::build_as(
                    &Mine::inner_circle(),
                    DrawMode::Fill(FillMode::color(Mine::COLOR)),
                    Transform::default(),
                ));

                (0..Mine::SIDES).for_each(|index| {
                    let mut transform = Transform::default();

                    transform.translation.y += Mine::SIZE + (Mine::SPOT_RADII * 2.5);
                    transform.rotate_around(
                        Vec3::default(),
                        Quat::from_rotation_z(f32::to_radians(60.0 * index as f32)),
                    );

                    parent.spawn((
                        MiningSpot { occupied: false },
                        GeometryBuilder::build_as(
                            &Mine::mine_spot(),
                            DrawMode::Outlined {
                                fill_mode: FillMode::color(Color::NONE),
                                outline_mode: StrokeMode::new(Color::GRAY, Mine::SPOT_BORDER_WIDTH),
                            },
                            transform,
                        ),
                    ));
                });
            })
            .id()
    }

    fn spawn_initial_mines(mut commands: Commands) {
        Mine::spawn(&mut commands, Transform::from_xyz(-60., -150., 0.));
        Mine::spawn(&mut commands, Transform::from_xyz(60., -150., 0.));
    }

    fn debug_occupied_status(mut query: Query<(&mut DrawMode, &MiningSpot)>) {
        for (mut mode, spot) in query.iter_mut() {
            if let DrawMode::Outlined {
                fill_mode: _,
                ref mut outline_mode,
            } = *mode
            {
                outline_mode.color = match spot.occupied {
                    true => Color::GREEN,
                    false => Color::RED,
                };
            }
        }
    }
}

#[derive(Component)]
struct Worker;

#[derive(Component)]
struct WorkingZone(Entity);

impl Worker {
    fn body() -> RegularPolygon {
        shapes::RegularPolygon {
            sides: 5,
            feature: shapes::RegularPolygonFeature::Radius(10.0),
            ..default()
        }
    }

    fn spawn(commands: &mut Commands, transform: Transform) -> Entity {
        commands
            .spawn((
                Worker,
                GeometryBuilder::build_as(
                    &Worker::body(),
                    DrawMode::Fill(FillMode::color(Color::BEIGE)),
                    transform,
                ),
            ))
            .id()
    }

    fn spawn_initial_worker(mut commands: Commands) {
        Worker::spawn(&mut commands, Transform::default());
        Worker::spawn(&mut commands, Transform::from_xyz(30., 0., 0.));
        Worker::spawn(&mut commands, Transform::from_xyz(-30., 0., 0.));
        Worker::spawn(&mut commands, Transform::from_xyz(60., 0., 0.));
        Worker::spawn(&mut commands, Transform::from_xyz(-60., 0., 0.));
    }

    fn locate_mining_spot(
        mut commands: Commands,
        workers: Query<Entity, (Without<WorkingZone>, With<Worker>)>,
        mut spots: Query<(Entity, &mut MiningSpot)>,
    ) {
        let mut filtered_spots = spots
            .iter_mut()
            .filter(|(_, spot)| !spot.occupied)
            .collect::<Vec<_>>();

        if filtered_spots.len() == 0 {
            return;
        }

        filtered_spots.shuffle(&mut rand::thread_rng());

        for worker_id in workers.iter() {
            let chosen_spot = filtered_spots.pop();

            match chosen_spot {
                Some((spot_id, mut spot)) => {
                    spot.occupied = true;
                    commands.entity(worker_id).insert(WorkingZone(spot_id));
                }
                None => println!("Invalid Spot"),
            }
        }
    }
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
        app.add_startup_system(Mine::spawn_initial_mines)
            .add_system(Mine::debug_occupied_status)
            .add_startup_system(Worker::spawn_initial_worker)
            .add_system(Worker::locate_mining_spot);
    }
}
