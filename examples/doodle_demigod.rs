use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_asset_loader::prelude::*;

const WINDOW_TITLE: &str = "Doodle Demigod";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BEIGE;

enum TileType {
    Trees,
    Rocks,
    TreesRocks,
}

fn tile_type_name(tile: TileType) -> String {
    let name = match tile {
        TileType::Trees => "Trees",
        TileType::Rocks => "Rocks",
        TileType::TreesRocks => "Trees amoung Rocks",
        _ => "No idea m80",
    };

    name.to_string()
}

struct Recipe {
    ingredients: [TileType; 2],
    result: TileType,
}

impl Recipe {
    fn new(ingredients: [TileType; 2], result: TileType) -> Recipe {
        Recipe {
            ingredients,
            result,
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
        .add_startup_system(spawn_camera)
        .add_plugin(DoodleDemiGodPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

struct DoodleDemiGodPlugin;

impl Plugin for DoodleDemiGodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Recipes::default())
            .add_startup_system(spawn_slots);
    }
}

#[derive(AssetCollection, Resource)]
struct TileAssets {
    #[asset(path = "doodle_demigod/trees.png")]
    trees: Handle<Image>,
    #[asset(path = "doodle_demigod/rocks.png")]
    rocks: Handle<Image>,
    #[asset(path = "doodle_demigod/trees+rocks.png")]
    trees_rocks: Handle<Image>,
}

#[derive(Resource)]
struct Recipes(Vec<Recipe>);

impl Default for Recipes {
    fn default() -> Self {
        let mut recipes: Vec<Recipe> = vec![];

        recipes.push(Recipe::new(
            [TileType::Trees, TileType::Rocks],
            TileType::TreesRocks,
        ));

        Recipes(recipes)
    }
}

#[derive(Component)]
struct Slot;

#[derive(Component)]
struct Tile(TileType);

fn spawn_slots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size: f32 = 80.;
    let child_size: f32 = 75.;
    let gap: f32 = 40.;

    let offset = 0. + size + (gap / 2.0);

    let inner_child = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(child_size, 6).into())
            .into(),
        material: materials.add(ColorMaterial::from(BACKGROUND_COLOR)),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    };

    // Top Slot
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(size, 6).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(0., -offset, 0.)),
                ..default()
            },
            Slot,
        ))
        .with_children(|parent| {
            parent.spawn(inner_child.clone());
        });

    // Bottom Slot
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::RegularPolygon::new(size, 6).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(0., offset, 0.)),
                ..default()
            },
            Slot,
        ))
        .with_children(|parent| {
            parent.spawn(inner_child.clone());
        });
}
