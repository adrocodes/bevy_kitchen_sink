use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle},
};
use bevy_asset_loader::prelude::*;

const WINDOW_TITLE: &str = "Doodle Demigod";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BEIGE;

const SLOT_SIZE: f32 = 85.0;
const TILE_GAP: f32 = 20.0;
const TILE_WIDTH: f32 = 120.0;
const TILE_HEIGHT: f32 = 140.0;
const TILE_ROW_COUNT: f32 = 3.0;

#[derive(Clone)]
enum TileType {
    Trees,
    Rocks,
    TreesRocks,
}

impl TileType {
    fn name(&self) -> String {
        let name = match self {
            TileType::Trees => "Trees",
            TileType::Rocks => "Rocks",
            TileType::TreesRocks => "Trees amoung Rocks",
        };

        name.to_string()
    }

    fn asset(&self, assets: &TileAssets) -> Handle<Image> {
        match self {
            TileType::Trees => assets.trees.clone(),
            TileType::Rocks => assets.rocks.clone(),
            TileType::TreesRocks => assets.trees_rocks.clone(),
        }
    }
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
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Next)
                .with_collection::<TileAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .insert_resource(Recipes::default())
        .add_system_set(
            SystemSet::on_enter(GameState::Next)
                .with_system(spawn_slots)
                .with_system(spawn_initial_tiles),
        )
        .add_system_set(SystemSet::on_update(GameState::Next).with_system(reposition_tile_choices));
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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
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
struct Slot(Option<TileType>);

#[derive(Component)]
struct Tile(TileType);

#[derive(Component)]
struct TileContainer(Vec2);

fn spawn_slots(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size: f32 = SLOT_SIZE;
    let child_size: f32 = size - 5.;
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
            Slot(None),
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
            Slot(None),
        ))
        .with_children(|parent| {
            parent.spawn(inner_child.clone());
        });
}

fn spawn_initial_tiles(
    mut commands: Commands,
    windows: Res<Windows>,
    tile_assets: Res<TileAssets>,
) {
    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    let half_width = width / 2.0;
    let half_height = height / 2.0;

    let parent_zone_width = (TILE_WIDTH * TILE_ROW_COUNT) + (TILE_GAP * (TILE_ROW_COUNT - 1.0));
    let parent_zone_height = height - 40.0;

    let size = Vec2::new(parent_zone_width, parent_zone_height);

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    custom_size: Some(size),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_xyz(-half_width + 20.0, half_height - 20.0, 0.0),
                ..default()
            },
            TileContainer(size),
        ))
        .with_children(|parent| {
            vec![
                TileType::Trees,
                TileType::Rocks,
                TileType::Trees,
                TileType::Rocks,
                TileType::TreesRocks,
                TileType::Rocks,
                TileType::TreesRocks,
                TileType::Trees,
                TileType::Rocks,
                TileType::TreesRocks,
            ]
            .iter()
            .for_each(|tile| {
                parent.spawn((
                    SpriteBundle {
                        texture: tile.asset(&tile_assets),
                        sprite: Sprite {
                            anchor: Anchor::TopLeft,
                            ..default()
                        },
                        ..default()
                    },
                    Tile(tile.clone()),
                    Name::new(tile.name()),
                ));
            });
        });
}

fn reposition_tile_choices(
    q_parent: Query<&Children, With<TileContainer>>,
    mut q_tiles: Query<&mut Transform, With<Tile>>,
) {
    let children = q_parent.single();
    let mut x_offset: f32 = 0.0;
    let mut y_offset: f32 = 0.0;
    let mut offset_row: bool = false;

    let gap: f32 = TILE_GAP;

    for &child in children.iter() {
        if let Ok(mut transform) = q_tiles.get_mut(child) {
            if (x_offset == TILE_ROW_COUNT && !offset_row)
                || (x_offset == (TILE_ROW_COUNT - 1.0) && offset_row)
            {
                y_offset += 1.0;
                x_offset = 0.0;
                offset_row = !offset_row;
            }

            let offset_row_x: f32 = match offset_row {
                true => (TILE_WIDTH / 2.0) + (gap / 2.0),
                _ => 0.0,
            };
            let x = ((TILE_WIDTH + gap) * x_offset) + offset_row_x;
            let y = (TILE_HEIGHT - gap) * y_offset;

            transform.translation = Vec3::new(x, -y, 0.0);

            x_offset += 1.0;
        };
    }
}
