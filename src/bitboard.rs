use bitvec::slice::BitSlice;

use crate::DimensionMismatch;

pub trait BitBoard: Sized {
    /// Returns the number of rows in the board.
    fn n_rows(&self) -> usize;

    /// Returns the number of columns in the board.
    fn n_cols(&self) -> usize;

    /// Returns a mutable reference to the underlying bits.
    fn board_mut(&mut self) -> &mut BitSlice;

    /// Returns an immutable reference to the underlying bits.
    fn board(&self) -> &BitSlice;

    /// Get the index that we can use to directly access a certain spot on the board
    fn index_of(&self, row: usize, col: usize) -> usize {
        assert!(
            row <= (self.n_rows() - 1),
            "row cannot be greater than n_rows"
        );
        assert!(
            col <= (self.n_cols() - 1),
            "col cannot be greater than n_cols"
        );
        (row * self.n_cols()) + col
    }

    /// Set all bits to the desired value.
    fn fill(&mut self, value: bool) {
        self.board_mut().fill(value);
    }

    fn or(&self, other: &impl BitBoard) -> Result<Self, DimensionMismatch>;
    fn and(&self, other: &impl BitBoard) -> Result<Self, DimensionMismatch>;

    /// Set the value at index [row, col] to be the `new_val`.
    fn set(&mut self, row: usize, col: usize, value: bool) {
        let new_ind = self.index_of(row, col);
        self.board_mut().set(new_ind, value);
    }
    /// Set an entire column to a certain value
    fn set_col(&mut self, col: usize, value: bool) {
        // For each row
        for r_idx in 0..self.n_rows() {
            // Calculate the index
            let idx = (r_idx * self.n_cols()) + col;
            self.board_mut().set(idx, value);
        }
    }

    /// Set an entire row to a certain value
    fn set_row(&mut self, row: usize, value: bool) {
        // For each column in the row
        for cidx in 0..self.n_cols() {
            // Calculate the index
            let idx = (row * self.n_cols()) + cidx;
            self.board_mut().set(idx, value);
        }
    }

    /// Will set the neighbors immediately above, below, left, and right to `value`. If
    /// the neighbor is out of bounds, nothing will happen
    fn set_cardinal_neighbors(&mut self, row: usize, col: usize, value: bool) {
        // Above
        if row > 0 {
            self.set(row - 1, col, value);
        }

        // Below
        if row < self.n_rows() - 1 {
            self.set(row + 1, col, value);
        }

        // Left
        if col > 0 {
            self.set(row, col - 1, value);
        }

        // Right
        if col < self.n_cols() - 1 {
            self.set(row, col + 1, value);
        }
    }

    /// Set just the spots diagonal from the given position to `value`. If
    /// the neighbor is out of bounds, nothing will happen
    fn set_diagonals(&mut self, row: usize, col: usize, value: bool) {
        // Above left
        if row > 0 && col > 0 {
            self.set(row - 1, col - 1, value);
        }

        // Above right
        if row > 0 && col < self.n_cols() - 1 {
            self.set(row - 1, col + 1, value);
        }

        // Below left
        if row < self.n_rows() - 1 && col > 0 {
            self.set(row + 1, col - 1, value);
        }

        // Below right
        if row < self.n_rows() - 1 && col < self.n_cols() - 1 {
            self.set(row + 1, col + 1, value);
        }
    }

    /// Set the cardinal neighbors and the diagonal neighbors to `value`. If
    /// the neighbor is out of bounds, nothing will happen
    fn set_all_neighbors(&mut self, row: usize, col: usize, value: bool) {
        self.set_cardinal_neighbors(row, col, value);
        self.set_diagonals(row, col, value);
    }
}
