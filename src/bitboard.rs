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

    /// Get the row and column of the linear index
    fn row_col_of(&self, index: usize) -> (usize, usize) {
        let row = index / self.n_cols();
        let col = index % self.n_cols();
        (row, col)
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

    /// Get the value at index [row, col]. If the index is out of bounds, return false.
    fn get(&self, row: usize, col: usize) -> bool {
        let new_ind = self.index_of(row, col);
        *self.board().get(new_ind).as_deref().unwrap_or(&false)
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

    /// Get the values in a given col
    fn get_col(&self, col: usize) -> impl Iterator<Item = bool> {
        (0..self.n_rows()).map(move |row| self.get(row, col))
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

    /// Get the values in a given row
    fn get_row(&self, row: usize) -> impl Iterator<Item = bool> {
        (0..self.n_cols()).map(move |col| self.get(row, col))
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

#[cfg(test)]
mod tests {
    use crate::{bitboard::BitBoard, bitboardstatic::BitBoardStatic};
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 0, 1)]
    #[case(2, 1, 0)]
    #[case(3, 1, 1)]
    fn index_of_and_row_col_of(#[case] index: usize, #[case] row: usize, #[case] col: usize) {
        let bb = BitBoardStatic::<1>::new(2, 2);
        assert_eq!(bb.index_of(row, col), index);
        assert_eq!(bb.row_col_of(index), (row, col));
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 0, 1)]
    #[case(2, 0, 2)]
    #[case(3, 1, 0)]
    #[case(4, 1, 1)]
    #[case(5, 1, 2)]
    #[case(6, 2, 0)]
    #[case(7, 2, 1)]
    #[case(8, 2, 2)]
    fn index_of_and_row_col_of_3x3(#[case] index: usize, #[case] row: usize, #[case] col: usize) {
        let bb = BitBoardStatic::<1>::new(3, 3);
        assert_eq!(bb.index_of(row, col), index);
        assert_eq!(bb.row_col_of(index), (row, col));
    }

    #[test]
    fn index_of_and_row_col_of_2x10() {
        let bb = BitBoardStatic::<1>::new(2, 10);
        for index in 0..20 {
            let (row, col) = bb.row_col_of(index);
            assert_eq!(bb.index_of(row, col), index);
        }
    }
}
