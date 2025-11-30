// src/core/board.rs

use crate::core::player::CorePlayer;

#[derive(Clone)]
pub struct CoreBoard {
    size: u8,
    cells: Vec<Option<CorePlayer>>,
}

impl CoreBoard {
    /// Create new empty board with given size (size x size).
    pub fn new(size: u8) -> Self {
        let len = (size as usize) * (size as usize);
        Self {
            size,
            cells: vec![None; len],
        }
    }

    /// Board size (N for NxN board).
    pub fn size(&self) -> u8 {
        self.size
    }

    fn index(&self, row: u8, col: u8) -> usize {
        (row as usize) * (self.size as usize) + (col as usize)
    }

    /// Get cell value at (row, col).
    pub fn get(&self, row: u8, col: u8) -> Option<CorePlayer> {
        if row >= self.size || col >= self.size {
            return None;
        }
        let idx = self.index(row, col);
        self.cells[idx]
    }

    /// Set cell value at (row, col) directly.
    pub fn set(&mut self, row: u8, col: u8, value: Option<CorePlayer>) {
        if row >= self.size || col >= self.size {
            return;
        }
        let idx = self.index(row, col);
        self.cells[idx] = value;
    }

    /// Try to place mark for a player. Returns true if move was accepted.
    pub fn place_mark(
        &mut self,
        row: u8,
        col: u8,
        player: CorePlayer,
    ) -> bool {
        if row >= self.size || col >= self.size {
            return false;
        }

        let idx = self.index(row, col);
        if self.cells[idx].is_some() {
            // Cell is already occupied.
            return false;
        }

        self.cells[idx] = Some(player);
        true
    }

    /// Clear all cells to empty state.
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = None;
        }
    }

    /// Returns iterator over internal cells (row-major order).
    pub fn iter_cells(&self) -> impl Iterator<Item = Option<CorePlayer>> + '_ {
        self.cells.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::player::CorePlayer;

    #[test]
    fn new_board_is_empty() {
        let b = CoreBoard::new(3);
        assert_eq!(b.size(), 3);
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(b.get(row, col), None);
            }
        }
    }

    #[test]
    fn place_mark_once() {
        let mut b = CoreBoard::new(3);
        assert!(b.place_mark(1, 1, CorePlayer::X));
        assert_eq!(b.get(1, 1), Some(CorePlayer::X));
    }

    #[test]
    fn cannot_overwrite_cell() {
        let mut b = CoreBoard::new(3);
        assert!(b.place_mark(0, 0, CorePlayer::X));
        assert!(!b.place_mark(0, 0, CorePlayer::O));
        assert_eq!(b.get(0, 0), Some(CorePlayer::X));
    }
}
