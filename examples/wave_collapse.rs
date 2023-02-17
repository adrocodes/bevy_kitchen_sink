use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::seq::SliceRandom;

const WINDOW_TITLE: &str = "Wave Collapse";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

const TILE_COUNT: usize = 10;
const TILE_SIZE: f32 = 64.0;
const TILE_OFFSET: f32 = (TILE_SIZE * (TILE_COUNT / 2) as f32) - (TILE_SIZE / 2.0);

const BACKGROUND_COLOR: Color = Color::BEIGE;

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
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(spawn_camera)
        .add_plugin(WaveCollapseGamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileType {
    Cross,
    CurveBlc,
    CurveBrc,
    CurveTlc,
    CurveTrc,
    EndT,
    EndR,
    EndL,
    EndB,
    LeftToRight,
    TopToBottom,
    StraightBlc,
    StraightBrc,
    StraightTlc,
    StraightTrc,
    TeeB,
    TeeL,
    TeeR,
    TeeT,
    Empty,
}

impl TileType {
    fn asset(&self, assets: &TileAssets) -> Option<Handle<Image>> {
        match self {
            TileType::Cross => Some(assets.cross.clone()),
            TileType::CurveBlc => Some(assets.curve_blc.clone()),
            TileType::CurveBrc => Some(assets.curve_brc.clone()),
            TileType::CurveTlc => Some(assets.curve_tlc.clone()),
            TileType::CurveTrc => Some(assets.curve_trc.clone()),
            TileType::EndT => Some(assets.end_t.clone()),
            TileType::EndR => Some(assets.end_r.clone()),
            TileType::EndL => Some(assets.end_l.clone()),
            TileType::EndB => Some(assets.end_b.clone()),
            TileType::LeftToRight => Some(assets.l_to_r.clone()),
            TileType::TopToBottom => Some(assets.t_to_b.clone()),
            TileType::StraightBlc => Some(assets.straight_blc.clone()),
            TileType::StraightBrc => Some(assets.straight_brc.clone()),
            TileType::StraightTlc => Some(assets.straight_tlc.clone()),
            TileType::StraightTrc => Some(assets.straight_trc.clone()),
            TileType::TeeB => Some(assets.tee_b.clone()),
            TileType::TeeL => Some(assets.tee_l.clone()),
            TileType::TeeR => Some(assets.tee_r.clone()),
            TileType::TeeT => Some(assets.tee_t.clone()),
            TileType::Empty => None,
        }
    }
}

#[derive(Debug, Clone, Component)]
struct Tile {
    of_type: TileType,
    top: Vec<TileType>,
    right: Vec<TileType>,
    bottom: Vec<TileType>,
    left: Vec<TileType>,
}

impl Tile {
    fn empty() -> Self {
        TileBuilder::new().build(TileType::Empty)
    }

    /// Tiles that can connection to the top of a tile
    /// These tiles should have a bottom path to connect to
    fn top_connections() -> Vec<TileType> {
        vec![
            TileType::Cross,
            TileType::CurveTlc,
            TileType::CurveTrc,
            TileType::EndT,
            TileType::StraightTlc,
            TileType::StraightTrc,
            TileType::TopToBottom,
            TileType::TeeL,
            TileType::TeeR,
            TileType::TeeT,
            TileType::Empty,
        ]
    }

    /// Tiles that can connection to the right of a tile
    /// These tiles should have a left path to connect to
    fn right_connections() -> Vec<TileType> {
        vec![
            TileType::Cross,
            TileType::CurveBrc,
            TileType::CurveTrc,
            TileType::EndL,
            TileType::LeftToRight,
            TileType::StraightBrc,
            TileType::StraightTrc,
            TileType::TeeB,
            TileType::TeeL,
            TileType::TeeT,
            TileType::Empty,
        ]
    }

    /// Tiles that can connection to the bottom of a tile
    /// These tiles should have a top path to connect to
    fn bottom_connections() -> Vec<TileType> {
        vec![
            TileType::Cross,
            TileType::CurveBlc,
            TileType::CurveBrc,
            TileType::EndB,
            TileType::StraightBlc,
            TileType::StraightBrc,
            TileType::TopToBottom,
            TileType::TeeB,
            TileType::TeeL,
            TileType::TeeR,
            TileType::Empty,
        ]
    }

    /// Tiles that can connection to the left of a tile
    /// These tiles should have a right path to connect to
    fn left_connections() -> Vec<TileType> {
        vec![
            TileType::Cross,
            TileType::CurveBlc,
            TileType::CurveTlc,
            TileType::EndR,
            TileType::LeftToRight,
            TileType::StraightBlc,
            TileType::StraightTlc,
            TileType::TeeB,
            TileType::TeeR,
            TileType::TeeT,
            TileType::Empty,
        ]
    }
}

struct TileBuilder {
    top: Vec<TileType>,
    right: Vec<TileType>,
    bottom: Vec<TileType>,
    left: Vec<TileType>,
}

impl TileBuilder {
    pub fn new() -> TileBuilder {
        TileBuilder {
            top: vec![],
            right: vec![],
            bottom: vec![],
            left: vec![],
        }
    }

    pub fn top(mut self) -> TileBuilder {
        self.top = Tile::top_connections();
        self
    }

    pub fn right(mut self) -> TileBuilder {
        self.right = Tile::right_connections();
        self
    }

    pub fn bottom(mut self) -> TileBuilder {
        self.bottom = Tile::bottom_connections();
        self
    }

    pub fn left(mut self) -> TileBuilder {
        self.left = Tile::left_connections();
        self
    }

    pub fn build(self, of_type: TileType) -> Tile {
        Tile {
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.left,
            of_type,
        }
    }
}

fn gen_tile_list() -> Vec<Tile> {
    let cross = TileBuilder::new()
        .top()
        .right()
        .bottom()
        .left()
        .build(TileType::Cross);

    let curve_blc = TileBuilder::new().top().right().build(TileType::CurveBlc);
    let curve_brc = TileBuilder::new().top().left().build(TileType::CurveBrc);
    let curve_tlc = TileBuilder::new()
        .right()
        .bottom()
        .build(TileType::CurveTlc);
    let curve_trc = TileBuilder::new().left().bottom().build(TileType::CurveTrc);

    // TODO: Confirm end_t and end_b
    let end_t = TileBuilder::new().bottom().build(TileType::EndT);
    let end_b = TileBuilder::new().top().build(TileType::EndB);
    let end_r = TileBuilder::new().right().build(TileType::EndR);
    let end_l = TileBuilder::new().left().build(TileType::EndL);

    let l_to_r = TileBuilder::new()
        .right()
        .left()
        .build(TileType::LeftToRight);
    let t_to_b = TileBuilder::new()
        .top()
        .bottom()
        .build(TileType::TopToBottom);

    let straight_blc = TileBuilder::new()
        .top()
        .right()
        .build(TileType::StraightBlc);
    let straight_brc = TileBuilder::new().top().left().build(TileType::StraightBrc);
    let straight_tlc = TileBuilder::new()
        .right()
        .bottom()
        .build(TileType::StraightTlc);
    let straight_trc = TileBuilder::new()
        .left()
        .bottom()
        .build(TileType::StraightTrc);

    let tee_b = TileBuilder::new()
        .top()
        .right()
        .left()
        .build(TileType::TeeB);
    let tee_l = TileBuilder::new()
        .top()
        .bottom()
        .left()
        .build(TileType::TeeL);
    let tee_r = TileBuilder::new()
        .top()
        .right()
        .bottom()
        .build(TileType::TeeR);
    let tee_t = TileBuilder::new()
        .bottom()
        .right()
        .left()
        .build(TileType::TeeT);

    vec![
        cross,
        curve_blc,
        curve_brc,
        curve_tlc,
        curve_trc,
        // end_b,
        // end_l,
        // end_r,
        // end_t,
        l_to_r,
        t_to_b,
        straight_blc,
        straight_brc,
        straight_tlc,
        straight_trc,
        tee_b,
        tee_l,
        tee_r,
        tee_t,
    ]
}

#[derive(Debug)]
struct WaveCollapse {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Vec<Tile>>>,
    collapsed_grid: Vec<Vec<Option<Tile>>>,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
enum OffsetType {
    Top,
    Right,
    Bottom,
    Left,
}

impl WaveCollapse {
    fn new(rows: usize, cols: usize) -> Self {
        let grid: Vec<Vec<Vec<Tile>>> = (0..rows)
            .map(|_| (0..cols).map(|_| gen_tile_list()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let collapsed_grid: Vec<Vec<Option<Tile>>> = (0..rows)
            .map(|_| (0..cols).map(|_| None).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            rows,
            cols,
            grid,
            collapsed_grid,
        }
    }

    fn get_cell(&mut self, at: &Pos) -> &mut Vec<Tile> {
        &mut self.grid[at.row][at.col]
    }

    fn get_offset_cell(&mut self, at: &Pos, direction: OffsetType) -> Option<&mut Vec<Tile>> {
        let pos = match direction {
            OffsetType::Top => {
                if at.row == 0 {
                    return None;
                }

                Pos {
                    row: at.row - 1,
                    col: at.col,
                }
            }
            OffsetType::Right => {
                if at.col >= self.cols - 1 {
                    return None;
                }

                Pos {
                    row: at.row,
                    col: at.col + 1,
                }
            }
            OffsetType::Bottom => {
                if at.row >= self.rows - 1 {
                    return None;
                }

                Pos {
                    row: at.row + 1,
                    col: at.col,
                }
            }
            OffsetType::Left => {
                if at.col == 0 {
                    return None;
                }

                Pos {
                    row: at.row,
                    col: at.col - 1,
                }
            }
        };

        Some(self.get_cell(&pos))
    }

    fn get_collapsed_cell(&self, at: &Pos) -> Option<Tile> {
        self.collapsed_grid[at.row][at.col].clone()
    }

    fn collapse_cell(&mut self, at: &Pos) {
        let cell = self.get_cell(&at);
        // dbg!(&cell);
        let chosen_tile = cell.choose(&mut rand::thread_rng());

        if let Some(chosen_tile) = chosen_tile {
            self.collapsed_grid[at.row][at.col] = Some(chosen_tile.clone());
        } else {
            self.collapsed_grid[at.row][at.col] = Some(Tile::empty());
        }
    }

    fn get_next_lowest_tile(&self) -> Pos {
        let mut pos = Pos { row: 0, col: 0 };
        let mut count = usize::MAX;

        for (i, row) in self.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if col.len() < count && self.collapsed_grid[i][j].is_none() {
                    count = col.len();
                    pos = Pos { row: i, col: j };
                }
            }
        }

        pos
    }

    fn collapse(&mut self, at: Pos) {
        if self.grid_is_collapsed() {
            return;
        }

        self.collapse_cell(&at);
        self.repopulate_grid();

        self.propogate(&at, OffsetType::Top);
        self.propogate(&at, OffsetType::Right);
        self.propogate(&at, OffsetType::Bottom);
        self.propogate(&at, OffsetType::Left);

        let next_at = self.get_next_lowest_tile();
        self.collapse(next_at);
    }

    fn propogate(&mut self, from: &Pos, direction: OffsetType) {
        let tile = self.get_collapsed_cell(&from);

        if let Some(tile) = tile {
            let offset_tile = self.get_offset_cell(&from, direction);

            if let Some(offset_tile) = offset_tile {
                offset_tile.retain(|t| {
                    let result = match direction {
                        OffsetType::Top => {
                            tile.top.contains(&t.of_type)
                                || (tile.top.len() == 0 && t.bottom.len() == 0)
                        }
                        OffsetType::Right => {
                            tile.right.contains(&t.of_type)
                                || (tile.right.len() == 0 && t.left.len() == 0)
                        }
                        OffsetType::Bottom => {
                            tile.bottom.contains(&t.of_type)
                                || (tile.bottom.len() == 0 && t.top.len() == 0)
                        }
                        OffsetType::Left => {
                            tile.left.contains(&t.of_type)
                                || (tile.left.len() == 0 && t.right.len() == 0)
                        }
                    };

                    result
                });
            }
        }
    }

    fn repopulate_grid(&mut self) {
        let grid: Vec<Vec<Vec<Tile>>> = (0..self.rows)
            .map(|_| (0..self.cols).map(|_| gen_tile_list()).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        self.grid = grid;
    }

    fn grid_is_collapsed(&self) -> bool {
        self.collapsed_grid
            .iter()
            .all(|row| row.iter().all(|col| col.is_some()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_is_not_collapsed() {
        let wave = WaveCollapse::new(1, 1);
        assert_eq!(false, wave.grid_is_collapsed());
    }

    #[test]
    fn grid_is_collapsed() {
        let grid: Vec<Vec<Vec<Tile>>> = Vec::new();
        let wave = WaveCollapse {
            rows: 0,
            cols: 0,
            grid,
            collapsed_grid: Vec::new(),
        };

        assert_eq!(true, wave.grid_is_collapsed());
    }

    #[test]
    fn get_cell() {
        let mut wave = WaveCollapse::new(1, 1);
        let cell = wave.get_cell(&Pos { row: 0, col: 0 });
        assert_eq!(true, cell.len() > 0);
    }

    #[test]
    fn get_collapsed_cell() {
        let wave = WaveCollapse::new(1, 1);
        let cell = wave.get_collapsed_cell(&Pos { row: 0, col: 0 });
        assert!(cell.is_none());
    }

    #[test]
    fn get_offset_cell() {
        let mut wave = WaveCollapse::new(2, 2);
        // row < 0 - can't run because usize
        let cell = wave.get_offset_cell(&Pos { row: 0, col: 0 }, OffsetType::Top);
        assert!(cell.is_none());
        // col < 0 - can't run because usize
        let cell = wave.get_offset_cell(&Pos { row: 0, col: 0 }, OffsetType::Left);
        assert!(cell.is_none());
        // row >= rows
        let cell = wave.get_offset_cell(&Pos { row: 1, col: 0 }, OffsetType::Bottom);
        assert!(cell.is_none());
        // col >= cols
        let cell = wave.get_offset_cell(&Pos { row: 0, col: 1 }, OffsetType::Right);
        assert!(cell.is_none());

        // valid
        let cell = wave.get_offset_cell(&Pos { row: 0, col: 0 }, OffsetType::Right);
        assert!(cell.is_some());
        let cell = wave.get_offset_cell(&Pos { row: 0, col: 0 }, OffsetType::Right);
        assert!(cell.is_some());
    }

    #[test]
    fn collapse_cell() {
        let mut wave = WaveCollapse::new(1, 1);
        let pos = Pos { row: 0, col: 0 };
        wave.collapse_cell(&pos);
        let cell = &wave.collapsed_grid[pos.row][pos.col];

        assert_eq!(true, cell.is_some());
    }

    #[test]
    fn test_propogate() {
        let mut wave = WaveCollapse::new(5, 5);
        let pos = Pos { row: 1, col: 1 };
        wave.collapse_cell(&pos);
        wave.propogate(&pos, OffsetType::Top);
        let top_tile = wave.get_offset_cell(&pos, OffsetType::Top);

        assert_eq!(10, top_tile.unwrap().len());
    }

    #[test]
    fn test_next_lowest_tile() {
        let mut wave = WaveCollapse::new(5, 5);
        let pos = Pos { row: 1, col: 1 };
        wave.collapse_cell(&pos);
        wave.propogate(&pos, OffsetType::Top);

        let next_pos = wave.get_next_lowest_tile();
        assert_eq!(0, next_pos.row);
        assert_eq!(1, next_pos.col);
    }

    #[test]
    fn test_collapse() {
        let mut wave = WaveCollapse::new(5, 5);
        wave.collapse(Pos { row: 0, col: 0 });

        assert!(wave.grid_is_collapsed());
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Playing,
}

#[derive(AssetCollection, Resource)]
struct TileAssets {
    #[asset(path = "wave_collapse/cross.png")]
    cross: Handle<Image>,
    #[asset(path = "wave_collapse/curve_blc.png")]
    curve_blc: Handle<Image>,
    #[asset(path = "wave_collapse/curve_brc.png")]
    curve_brc: Handle<Image>,
    #[asset(path = "wave_collapse/curve_tlc.png")]
    curve_tlc: Handle<Image>,
    #[asset(path = "wave_collapse/curve_trc.png")]
    curve_trc: Handle<Image>,
    #[asset(path = "wave_collapse/end_b.png")]
    end_b: Handle<Image>,
    #[asset(path = "wave_collapse/end_l.png")]
    end_l: Handle<Image>,
    #[asset(path = "wave_collapse/end_r.png")]
    end_r: Handle<Image>,
    #[asset(path = "wave_collapse/end_t.png")]
    end_t: Handle<Image>,
    #[asset(path = "wave_collapse/l_to_r.png")]
    l_to_r: Handle<Image>,
    #[asset(path = "wave_collapse/straight_blc.png")]
    straight_blc: Handle<Image>,
    #[asset(path = "wave_collapse/straight_brc.png")]
    straight_brc: Handle<Image>,
    #[asset(path = "wave_collapse/straight_tlc.png")]
    straight_tlc: Handle<Image>,
    #[asset(path = "wave_collapse/straight_trc.png")]
    straight_trc: Handle<Image>,
    #[asset(path = "wave_collapse/t_to_b.png")]
    t_to_b: Handle<Image>,
    #[asset(path = "wave_collapse/tee_b.png")]
    tee_b: Handle<Image>,
    #[asset(path = "wave_collapse/tee_l.png")]
    tee_l: Handle<Image>,
    #[asset(path = "wave_collapse/tee_r.png")]
    tee_r: Handle<Image>,
    #[asset(path = "wave_collapse/tee_t.png")]
    tee_t: Handle<Image>,
}

struct WaveCollapseGamePlugin;

impl Plugin for WaveCollapseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_collection::<TileAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_grid_map))
        .add_system_set(SystemSet::on_update(GameState::Playing));
    }
}

fn spawn_grid_map(mut commands: Commands, assets: Res<TileAssets>) {
    let mut wave = WaveCollapse::new(TILE_COUNT, TILE_COUNT);
    wave.collapse(Pos { row: 0, col: 0 });
    wave.collapse(Pos { row: 0, col: 1 });
    let grid = wave.collapsed_grid;

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if let Some(tile) = col {
                let mut sprite_bundle = SpriteBundle {
                    transform: Transform::from_xyz(
                        (TILE_SIZE * j as f32) - TILE_OFFSET,
                        (TILE_SIZE * i as f32 * -1.0) + TILE_OFFSET,
                        0.0,
                    ),
                    ..default()
                };
                let asset = tile.of_type.asset(&assets);

                match asset {
                    Some(texture) => {
                        sprite_bundle.texture = texture;
                    }
                    None => {}
                };

                commands.spawn((
                    tile.clone(),
                    sprite_bundle,
                    Name::new(format!("{}x{}", j, i)),
                ));
            }
        }
    }
}
