#[derive(PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}

pub struct Cell {
    pub state: CellState   
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        self.state == CellState::Alive
    }

    pub fn is_dead(&self) -> bool {
        self.state == CellState::Dead
    }
}
