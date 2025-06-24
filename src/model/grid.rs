use super::cell;
use std::collections::HashSet;

pub struct CellGrid {
    alive_cells: HashSet<(i32, i32)>,
    upper_scale: i32,
}

impl CellGrid {
    pub fn upper_scale(&self) -> i32 {
        self.upper_scale
    }
    pub fn new(upper_scale: i32) -> Self {
        CellGrid { alive_cells: HashSet::new(), upper_scale }
    }

    pub fn is_valid(&self, (x, y): &(i32, i32)) -> bool {
        if -self.upper_scale < *x && *x < self.upper_scale
        && -self.upper_scale < *y && *y < self.upper_scale {
            return true;
        }
        false
    }

    pub fn is_alive(&self, addr: &(i32, i32)) -> bool {
        self.alive_cells.contains(&addr)
    }

    pub fn alive_cells(&self) -> &HashSet<(i32, i32)> {
        &self.alive_cells
    }

    pub fn from_alive_cells(alive_cells: HashSet<(i32, i32)>, upper_scale: i32) -> Self {
        CellGrid { alive_cells, upper_scale }
    }
}

impl From<Vec<Vec<cell::Cell>>> for CellGrid {
    fn from(grid: Vec<Vec<cell::Cell>>) -> Self {
        if grid.is_empty() {
            return CellGrid::new(1024);
        }

        let alive_cells = grid
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row
                    .iter()
                    .enumerate()
                    .filter(|(_, cell)| cell.is_alive())
                    .map(move |(y, _)| (x as i32, y as i32))
            }).collect();
        let upper_scale = std::cmp::max(grid.len(), grid[0].len()) as i32;
        CellGrid { alive_cells, upper_scale }
    }
}
