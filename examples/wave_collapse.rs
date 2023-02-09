use bevy::prelude::*;
use rand::seq::SliceRandom;

const WINDOW_TITLE: &str = "Wave Collapse";
const WINDOW_WIDTH: f32 = 1133.0;
const WINDOW_HEIGHT: f32 = 744.0;

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
        .add_startup_system(spawn_camera)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Clone, Copy)]
struct Tile {
    // name: String,
    /// CSS Margin Rules -> Top, Right, Bottom, Left
    edges: [bool; 4],
}

impl Tile {
    fn new(edges: [bool; 4], name: &str) -> Self {
        Tile {
            edges,
            // name: name.to_owned(),
        }
    }

    fn copy(&self) -> Self {
        Self {
            edges: self.edges,
            // name: self.name,
        }
    }
}

fn gen_tile_list() -> Vec<Tile> {
    let cross = Tile::new([true, true, true, true], "cross");
    let curve_blc = Tile::new([true, true, false, false], "curve_blc");
    let curve_brc = Tile::new([true, false, false, true], "curve_brc");
    let curve_tlc = Tile::new([false, true, true, false], "curve_tlc");
    let curve_trc = Tile::new([false, false, true, true], "curve_trc");
    let end_t = Tile::new([true, false, false, false], "end_t");
    let end_r = Tile::new([false, true, false, false], "end_r");
    let end_l = Tile::new([false, false, false, true], "end_l");
    let end_b = Tile::new([false, false, true, false], "end_b");
    let l_to_r = Tile::new([false, true, false, true], "l_to_r");
    let straight_blc = Tile::new([true, true, false, false], "straight_blc");
    let straight_brc = Tile::new([true, false, false, true], "straight_brc");
    let straight_tlc = Tile::new([false, true, true, false], "straight_tlc");
    let straight_trc = Tile::new([false, false, true, true], "straight_trc");
    let t_to_b = Tile::new([true, false, true, false], "t_to_b");
    let tee_b = Tile::new([true, true, false, true], "tee_b");
    let tee_l = Tile::new([true, true, true, false], "tee_l");
    let tee_r = Tile::new([true, false, true, true], "tee_r");
    let tee_t = Tile::new([false, true, true, true], "tee_t");
    let empty = Tile::new([false, false, false, false], "empty");

    vec![
        cross,
        curve_blc,
        curve_brc,
        curve_tlc,
        curve_trc,
        end_b,
        end_l,
        end_r,
        end_t,
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
        empty,
    ]
}

struct WaveCollapse {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<Vec<Tile>>>,
    collapsed_grid: Vec<Vec<Option<Tile>>>,
}

struct Pos {
    row: usize,
    col: usize,
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

    fn collapse_cell(&mut self, at: &Pos) {
        let cell = self.get_cell(&at);
        let chosen_tile = cell.choose(&mut rand::thread_rng()).unwrap();

        self.collapsed_grid[at.row][at.col] = Some(*chosen_tile);
    }

    fn collapse(&mut self, at: Pos) {
        if self.grid_is_collapsed() {
            return;
        }

        self.collapse_cell(&at);
        self.propogate(&at);
    }

    fn propogate(&mut self, from: &Pos) {
        todo!();
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
    fn collapse_cell() {
        let mut wave = WaveCollapse::new(1, 1);
        let pos = Pos { row: 0, col: 0 };
        wave.collapse_cell(&pos);
        let cell = wave.collapsed_grid[pos.row][pos.col];

        assert_eq!(true, cell.is_some());
    }
}
