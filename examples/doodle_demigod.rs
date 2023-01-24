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

const NORMAL_BUTTON: Color = Color::BLACK;
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::DARK_GREEN;

#[derive(Clone, Eq, PartialEq, Copy, PartialOrd, Ord)]
enum TileType {
    Trees,
    Rocks,
    TreesRocks,
    Stone,
    Fountain,
    FountainStoneTrees,
    StoneTrees,
}

impl TileType {
    fn name(&self) -> String {
        let name = match self {
            TileType::Trees => "Trees",
            TileType::Rocks => "Rocks",
            TileType::TreesRocks => "Trees amoung Rocks",
            TileType::Fountain => "Fountain",
            TileType::Stone => "Stone",
            TileType::FountainStoneTrees => "Fountain among trees",
            TileType::StoneTrees => "Stone Trees",
        };

        name.to_string()
    }

    fn asset(&self, assets: &TileAssets) -> Handle<Image> {
        match self {
            TileType::Trees => assets.trees.clone(),
            TileType::Rocks => assets.rocks.clone(),
            TileType::TreesRocks => assets.trees_rocks.clone(),
            TileType::Fountain => assets.fountain.clone(),
            TileType::Stone => assets.stone.clone(),
            TileType::FountainStoneTrees => assets.fountain_stone_trees.clone(),
            TileType::StoneTrees => assets.stone_trees.clone(),
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

struct TileSelectEvent;
struct ClearSlotsEvent;
struct SpawnRecipeTileEvent(TileType);

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
        .add_event::<TileSelectEvent>()
        .add_event::<ClearSlotsEvent>()
        .add_event::<SpawnRecipeTileEvent>()
        .insert_resource(Recipes::default())
        .insert_resource(SlotBorderColor(Color::BLACK))
        .add_system_set(
            SystemSet::on_enter(GameState::Next)
                .with_system(spawn_slots)
                .with_system(spawn_initial_tiles)
                .with_system(spawn_merge_button),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(reposition_tile_choices.label("tile_reposition"))
                .with_system(update_bounds_position.after("tile_reposition"))
                .with_system(hover_square)
                .with_system(select_tile.label("select_title"))
                .with_system(deselect_tile)
                .with_system(
                    move_to_goal_translation
                        .after("select_tile")
                        .label("goal_transition"),
                )
                .with_system(check_merge)
                .with_system(clear_slots)
                .with_system(spawn_recipe_tile),
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
    #[asset(path = "doodle_demigod/stone.png")]
    stone: Handle<Image>,
    #[asset(path = "doodle_demigod/fountain+stone_trees.png")]
    fountain_stone_trees: Handle<Image>,
    #[asset(path = "doodle_demigod/fountain.png")]
    fountain: Handle<Image>,
    #[asset(path = "doodle_demigod/stone+trees.png")]
    stone_trees: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}

#[derive(Resource)]
struct Recipes(Vec<Recipe>);

impl Recipes {
    fn find_by_tiles(&self, mut tiles: [Option<TileType>; 2]) -> Option<TileType> {
        let recipe = self.0.iter().find(|r| {
            let mut ingredients = r.ingredients;

            ingredients.sort() == tiles.sort()
        });

        match recipe {
            Some(r) => Some(r.result),
            _ => None,
        }
    }
}

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

#[derive(Resource)]
struct SlotBorderColor(Color);

#[derive(Component, PartialEq)]
struct Slot(Option<TileType>);

impl Slot {
    fn all_slots_occupied(slots: &Vec<&Slot>) -> bool {
        let empty_slot = slots.iter().find(|s| s.0 == None);
        return empty_slot == None;
    }
}

#[derive(Component)]
struct BelongsTo(Entity);

#[derive(Component)]
struct GoalTranslation(Vec3);

#[derive(Component, PartialEq)]
struct Tile(TileType);

impl Tile {
    fn existing_tile(tiles: &Vec<&Tile>, search: &Tile) -> bool {
        tiles.contains(&search)
    }
}

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
    border: Res<SlotBorderColor>,
) {
    let size: f32 = SLOT_SIZE;
    let child_size: f32 = size - 5.;
    let gap: f32 = 80.;

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
                material: materials.add(ColorMaterial::from(border.0)),
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
                material: materials.add(ColorMaterial::from(border.0)),
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

fn spawn_merge_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(175.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "COMBINE",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
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
    mut ev_tile_selected: EventWriter<TileSelectEvent>,
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

                ev_tile_selected.send(TileSelectEvent);

                break;
            }
        }
    }
}

fn deselect_tile(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut slot_query: Query<(Entity, &mut Slot)>,
    tile_query: Query<(Entity, &Bounds2, &BelongsTo)>,
    mouse_position: Res<WorldPosition>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let tile = tile_query
            .iter()
            .filter(|(_, bounds, _)| bounds.in_bounds_centered(mouse_position.0))
            .collect::<Vec<_>>();

        if let Some(tile) = tile.get(0) {
            let tile_entity = tile.0;
            let tile_belongs_to = tile.2;

            for (slot_entity, mut slot) in slot_query.iter_mut() {
                if slot_entity != tile_belongs_to.0 {
                    continue;
                }

                slot.0 = None;

                commands.entity(tile_entity).despawn();
            }
        }
    }
}

fn move_to_goal_translation(mut query: Query<(&mut Transform, &GoalTranslation)>) {
    for (mut transform, goal) in query.iter_mut() {
        transform.translation = transform.translation.lerp(goal.0, 0.5);
    }
}

fn clear_slots(
    mut commands: Commands,
    ev_clear_slots: EventReader<ClearSlotsEvent>,
    mut slots: Query<&mut Slot>,
    selected_tiles: Query<Entity, With<BelongsTo>>,
) {
    if ev_clear_slots.is_empty() {
        return;
    }

    for mut slot in slots.iter_mut() {
        slot.0 = None;
    }

    for tile in selected_tiles.iter() {
        commands.entity(tile).despawn();
    }

    ev_clear_slots.clear();
}

fn spawn_recipe_tile(
    mut commands: Commands,
    mut ev_recipe_tile: EventReader<SpawnRecipeTileEvent>,
    tile_assets: Res<TileAssets>,
    tile_container: Query<Entity, With<TileContainer>>,
) {
    if ev_recipe_tile.is_empty() {
        return;
    }

    let tile = ev_recipe_tile.iter().collect::<Vec<_>>();

    if let Some(tile) = tile.get(0) {
        let tile = tile.0;
        let parent = tile_container.single();
        let child = commands
            .spawn((
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
            ))
            .id();

        commands.entity(parent).push_children(&[child]);
    }

    ev_recipe_tile.clear()
}

fn check_merge(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_clear_slots: EventWriter<ClearSlotsEvent>,
    mut ev_spawn_recipe_tile: EventWriter<SpawnRecipeTileEvent>,
    slots: Query<&mut Slot>,
    recipes: Res<Recipes>,
    tiles: Query<&Tile>,
) {
    for (interaction, mut color, mut transform) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                transform.scale = Vec3::new(1.0, 1.0, 1.0);

                let all_slots = &slots.iter().collect::<Vec<_>>();
                let all_tiles = tiles.iter().collect::<Vec<_>>();

                if Slot::all_slots_occupied(&all_slots) {
                    let s1 = all_slots.get(0).unwrap();
                    let s2 = all_slots.get(1).unwrap();
                    let tiles: [Option<TileType>; 2] = [s1.0, s2.0];
                    let recipe = recipes.find_by_tiles(tiles);

                    if let Some(recipe) = recipe {
                        let tile = Tile(recipe);

                        if !Tile::existing_tile(&all_tiles, &tile) {
                            println!("New recipe found, need to spawn shit");
                            ev_spawn_recipe_tile.send(SpawnRecipeTileEvent(recipe));
                            ev_clear_slots.send(ClearSlotsEvent);
                        }
                    }
                }
            }
            Interaction::Hovered => {
                transform.scale = transform.scale.lerp(Vec3::new(1.1, 1.1, 1.1), 0.9);
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                transform.scale = Vec3::new(1.0, 1.0, 1.0);
            }
        }
    }
}
