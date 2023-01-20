use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle},
};
use bevy_asset_loader::prelude::*;
use bevy_mouse_position::{MousePositionPlugin, WorldPosition};

const WINDOW_TITLE: &str = "Doodle Demigod";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const BACKGROUND_COLOR: Color = Color::BEIGE;

const SLOT_SIZE: f32 = 85.0;
const TILE_GAP: f32 = 20.0;
const TILE_WIDTH: f32 = 120.0;
const TILE_HEIGHT: f32 = 140.0;
const TILE_ROW_COUNT: f32 = 3.0;

#[derive(Clone, Eq, PartialEq, Copy)]
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
        .add_plugin(MousePositionPlugin)
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
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(reposition_tile_choices.label("tile_reposition"))
                .with_system(update_bounds_position.after("tile_reposition"))
                .with_system(hover_square)
                .with_system(select_tile)
                .with_system(move_to_goal_translation),
        );
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
struct BelongsTo(Entity);

#[derive(Component)]
struct GoalTranslation(Vec3);

#[derive(Component)]
struct Tile(TileType);

#[derive(Component)]
struct TileContainer(Vec2);

#[derive(Component)]
struct Hovered;

#[derive(Debug, Clone, Copy, Component)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }

    pub fn in_bounds_centered(&self, coords: Vec2) -> bool {
        let half_size = self.size * Vec2::new(0.5, 0.5);
        let new_position = self.position - half_size;
        let bounds = Bounds2 {
            position: new_position,
            size: self.size,
        };

        return bounds.in_bounds(coords);
    }
}

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
            vec![TileType::Trees, TileType::Rocks]
                .iter()
                .for_each(|tile| {
                    parent.spawn((
                        SpriteBundle {
                            texture: tile.asset(&tile_assets),
                            sprite: Sprite { ..default() },
                            ..default()
                        },
                        Tile(tile.clone()),
                        Name::new(tile.name()),
                        Bounds2 {
                            position: Vec2::default(),
                            size: Vec2::new(TILE_WIDTH, TILE_HEIGHT),
                        },
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
            let x = ((TILE_WIDTH + gap) * x_offset) + offset_row_x + (TILE_WIDTH / 2.0);
            let y = (TILE_HEIGHT - gap) * y_offset + (TILE_HEIGHT / 2.0);

            transform.translation = Vec3::new(x, -y, 0.0);

            x_offset += 1.0;
        };
    }
}

fn update_bounds_position(
    mut tiles: Query<(&GlobalTransform, &mut Bounds2), Or<(With<Tile>, With<GoalTranslation>)>>,
) {
    for (transform, mut bounds) in tiles.iter_mut() {
        let t = transform.translation();
        bounds.position = Vec2::new(t.x, t.y);
    }
}

fn hover_square(mut query: Query<(&mut Transform, &Bounds2)>, mouse_position: Res<WorldPosition>) {
    for (mut transform, bounds) in query.iter_mut() {
        if bounds.in_bounds_centered(mouse_position.0) {
            transform.scale = transform.scale.lerp(Vec3::new(1.15, 1.15, 1.1), 0.2);
        } else {
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }
}

fn select_tile(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    tile_query: Query<(&Tile, &Bounds2, &GlobalTransform)>,
    mut slots_query: Query<(Entity, &mut Slot, &GlobalTransform)>,
    mouse_position: Res<WorldPosition>,
    tile_assets: Res<TileAssets>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let tile = tile_query
            .iter()
            .filter(|(_, bounds, _)| bounds.in_bounds_centered(mouse_position.0))
            .collect::<Vec<_>>();

        if let Some(tile) = tile.get(0) {
            let c_tile = tile.0;
            let tile_transform = tile.2;

            for (entity, mut slot, transform) in slots_query.iter_mut() {
                if slot.0 != None {
                    continue;
                }

                slot.0 = Some(c_tile.0);

                let mut goal: Vec3 = Vec3::from(transform.translation());
                goal.z = 2.0;
                let sprite_transform = Transform::from_xyz(
                    tile_transform.translation().x,
                    tile_transform.translation().y,
                    2.0,
                );

                commands.spawn((
                    SpriteBundle {
                        texture: c_tile.0.asset(&tile_assets),
                        transform: sprite_transform,
                        ..default()
                    },
                    BelongsTo(entity),
                    GoalTranslation(goal),
                    Bounds2 {
                        position: Vec2::default(),
                        size: Vec2::new(TILE_WIDTH, TILE_HEIGHT),
                    },
                ));

                break;
            }
        }
    }
}

fn move_to_goal_translation(mut query: Query<(&mut Transform, &GoalTranslation)>) {
    for (mut transform, goal) in query.iter_mut() {
        transform.translation = transform.translation.lerp(goal.0, 0.5);
    }
}
