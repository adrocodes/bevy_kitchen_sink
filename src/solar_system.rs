use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct SolarSystemPlugin;

#[derive(Component)]
struct Planet {
    speed: f32,
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Sun;

#[derive(Resource, Default)]
struct TimeScale(f32);

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeScale(5.0))
            .add_startup_system(spawn_sun)
            .add_startup_system(spawn_planets)
            .add_system(rotate_planets_around_sun);
    }
}

fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Sun,
        Name("Sun".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::YELLOW)),
            ..default()
        },
    ));
}

fn spawn_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let earth_radii: f32 = 20.0;
    let au: f32 = -250.0;
    let earth_speed: f32 = f32::to_radians(10.);

    let mercury_radii: f32 = earth_radii * 0.38;
    let mercury_au: f32 = au * 0.4;
    let mercury_speed: f32 = earth_speed * (365.0 / 88.0);

    let mars_radii: f32 = earth_radii * 0.53;
    let mars_au: f32 = au * 1.5;
    let mars_speed: f32 = earth_speed / 1.88;

    let venus_radii: f32 = earth_radii * 0.95;
    let venus_au: f32 = au * 0.7;
    let venus_speed: f32 = earth_speed * (365.0 / 225.0);

    let neptune_radii: f32 = earth_radii * 3.88;
    let neptune_au: f32 = au * 30.1;
    let neptune_speed: f32 = earth_speed / 164.81;

    let uranus_radii: f32 = earth_radii * 4.0;
    let uranus_au: f32 = au * 19.8;
    let uranus_speed: f32 = earth_speed / 84.0;

    let saturn_radii: f32 = earth_radii * 9.45;
    let saturn_au: f32 = au * 9.5;
    let saturn_speed: f32 = earth_speed / 29.45;

    let jupiter_radii: f32 = earth_radii * 11.20;
    let jupiter_au: f32 = au * 5.2;
    let jupiter_speed: f32 = earth_speed / 11.86;

    // Mercury
    commands.spawn((
        Name("Mercury".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(mercury_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_xyz(mercury_au, 0.0, 0.0),
            ..default()
        },
        Planet {
            speed: mercury_speed,
        },
    ));

    // Venus
    commands.spawn((
        Name("Venus".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(venus_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PINK)),
            transform: Transform::from_xyz(venus_au, 0.0, 0.0),
            ..default()
        },
        Planet { speed: venus_speed },
    ));

    // Earth
    commands.spawn((
        Name("Earth".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(earth_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(au, 0.0, 0.0),
            ..default()
        },
        Planet { speed: earth_speed },
    ));

    // Mars
    commands.spawn((
        Name("Mars".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(mars_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_xyz(mars_au, 0.0, 0.0),
            ..default()
        },
        Planet { speed: mars_speed },
    ));

    // Jupiter
    commands.spawn((
        Name("Jupiter".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(jupiter_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_xyz(jupiter_au, 0.0, 0.0),
            ..default()
        },
        Planet {
            speed: jupiter_speed,
        },
    ));

    // Saturn
    commands.spawn((
        Name("Saturn".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(saturn_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_xyz(saturn_au, 0.0, 0.0),
            ..default()
        },
        Planet {
            speed: saturn_speed,
        },
    ));

    // Uranus
    commands.spawn((
        Name("Uranus".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(uranus_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_xyz(uranus_au, 0.0, 0.0),
            ..default()
        },
        Planet {
            speed: uranus_speed,
        },
    ));

    // Neptune
    commands.spawn((
        Name("Neptune".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(neptune_radii).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BEIGE)),
            transform: Transform::from_xyz(neptune_au, 0.0, 0.0),
            ..default()
        },
        Planet {
            speed: neptune_speed,
        },
    ));
}

fn rotate_planets_around_sun(
    sun_query: Query<&Transform, (With<Sun>, Without<Planet>)>,
    mut planets_query: Query<(&mut Transform, &Planet), (Without<Sun>, With<Planet>)>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    let sun = sun_query.single();

    for (mut planet_transform, planet) in planets_query.iter_mut() {
        planet_transform.rotate_around(
            Vec3::new(sun.translation.x, sun.translation.y, 0.0),
            Quat::from_rotation_z(planet.speed * time_scale.0 * time.delta_seconds()),
        );
    }
}
