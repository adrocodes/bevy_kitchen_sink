use bevy::prelude::*;
use rand::{thread_rng, Rng};

const WINDOW_TITLE: &str = "Random Squares";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BLACK;

const TILE_COUNT: i32 = 20;
const TILE_SIZE: f32 = 64.0;
const TILE_OFFSET: f32 = (TILE_SIZE * (TILE_COUNT / 2) as f32) - (TILE_SIZE / 2.0);

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
        .add_plugin(RandomSquaresPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

struct RandomSquaresPlugin;

#[derive(Resource, Default, Clone)]
struct MapAtlas(Vec<Vec<f32>>);

#[derive(Component)]
struct Tile;

impl Plugin for RandomSquaresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapAtlas::default())
            .add_startup_system(generate_map_grid.label("generate"))
            .add_startup_system(draw_map_grid.after("generate"));
    }
}

fn generate_map_grid(mut map_atlas: ResMut<MapAtlas>) {
    let mut rng = thread_rng();
    let new_atlas: Vec<Vec<f32>> = (0..TILE_COUNT)
        .map(|_| {
            let row: Vec<f32> = (0..TILE_COUNT).map(|_| rng.gen::<f32>()).collect();

            row
        })
        .collect();

    map_atlas.0 = new_atlas;
}

fn build_tile(pos: Vec2, asset_server: &Res<AssetServer>, filename: &str) -> SpriteBundle {
    let path = format!("random_squares/{}.png", filename);
    SpriteBundle {
        texture: asset_server.load(path),
        transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        ..default()
    }
}

fn determine_tile_type(value: &f32) -> &str {
    let as_percentage: i32 = (value * 100.) as i32;
    match as_percentage {
        0..=15 => "water",
        16..=20 => "sand",
        21..=30 => "dirt",
        31..=75 => "grass",
        76..=100 => "stone",
        _ => "grass",
    }
}

fn draw_map_grid(mut commands: Commands, asset_server: Res<AssetServer>, atlas: Res<MapAtlas>) {
    let mut x = 0;

    for row in &atlas.0 {
        let mut y = 0;

        for col in row {
            commands.spawn((
                build_tile(
                    Vec2::new(
                        TILE_SIZE * x as f32 - TILE_OFFSET,
                        TILE_SIZE * y as f32 - TILE_OFFSET,
                    ),
                    &asset_server,
                    determine_tile_type(&col),
                ),
                Tile,
            ));

            y += 1;
        }
        x += 1;
    }
}
