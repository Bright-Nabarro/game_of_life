use std::collections::HashMap;
use super::grid;
pub trait RulesInterface {
    fn next_gen(grid: &grid::CellGrid) -> grid::CellGrid;
}

struct SimpleRules;

impl RulesInterface for SimpleRules {
    fn next_gen(grid: &grid::CellGrid) -> grid::CellGrid {
        let mut neighbors: HashMap<(i32, i32), usize> = HashMap::new();
        grid.alive_cells()
            .iter()
            .for_each(|(x, y)| {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let neighbor = neighbors.entry((*x+dx, *y+dy)).or_insert(0);
                        *neighbor += 1;
                    }
                }
            });

        let next_grid = neighbors
            .iter()
            .filter(|&(addr, cnt)| -> bool {
                if !grid.is_valid(addr) {
                        return false;
                    }
                match cnt {
                    2 => {
                        if grid.is_alive(addr) {
                            return true;
                        } else {
                            return false;
                        }
                    },
                    3 => { return true; },
                    _ => { return false; }
                }
            })
            .map(|((x, y), _)| -> (i32, i32) {
                (*x, *y)
            })
            .collect();
            
        grid::CellGrid::from_alive_cells(next_grid, grid.upper_scale())
    }
}
