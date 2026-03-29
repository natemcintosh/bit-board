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
        assert!(
            index < self.n_rows() * self.n_cols(),
            "index out of bounds: index {index} >= board size {}",
            self.n_rows() * self.n_cols()
        );
        let row = index / self.n_cols();
        let col = index % self.n_cols();
        (row, col)
    }

    /// Set all bits to the desired value.
    fn fill(&mut self, value: bool) {
        self.board_mut().fill(value);
    }

    /// Returns a new board with the logical OR of the two boards.
    ///
    /// # Errors
    ///
    /// This function will return an error if the two boards have different dimensions.
    fn or(&self, other: &Self) -> Result<Self, DimensionMismatch>;

    /// Returns a new board with the logical AND of the two boards.
    ///
    /// # Errors
    ///
    /// This function will return an error if the two boards have different dimensions.
    fn and(&self, other: &Self) -> Result<Self, DimensionMismatch>;

    /// Set the value at index [row, col] to be the `new_val`.
    fn set(&mut self, row: usize, col: usize, value: bool) {
        let new_ind = self.index_of(row, col);
        self.board_mut().set(new_ind, value);
    }

    /// Get the value at index [row, col].
    ///
    /// # Panics
    ///
    /// Panics if `row` or `col` is out of bounds.
    fn get(&self, row: usize, col: usize) -> bool {
        let new_ind = self.index_of(row, col);
        *self.board().get(new_ind).as_deref().unwrap_or(&false)
    }

    /// Get the value at index [row, col], returning `None` if out of bounds.
    fn try_get(&self, row: usize, col: usize) -> Option<bool> {
        if row >= self.n_rows() || col >= self.n_cols() {
            return None;
        }
        let new_ind = self.index_of(row, col);
        Some(*self.board().get(new_ind).as_deref().unwrap_or(&false))
    }

    /// Set an entire column to a certain value
    fn set_col(&mut self, col: usize, value: bool) {
        for r_idx in 0..self.n_rows() {
            let idx = self.index_of(r_idx, col);
            self.board_mut().set(idx, value);
        }
    }

    /// Get the values in a given col.
    ///
    /// # Panics
    ///
    /// Panics if `col` is out of bounds.
    fn get_col(&self, col: usize) -> impl Iterator<Item = bool> {
        assert!(
            col <= (self.n_cols() - 1),
            "col cannot be greater than n_cols"
        );
        (0..self.n_rows()).map(move |row| self.get(row, col))
    }

    /// Set an entire row to a certain value
    fn set_row(&mut self, row: usize, value: bool) {
        for cidx in 0..self.n_cols() {
            let idx = self.index_of(row, cidx);
            self.board_mut().set(idx, value);
        }
    }

    /// Get the values in a given row.
    ///
    /// # Panics
    ///
    /// Panics if `row` is out of bounds.
    fn get_row(&self, row: usize) -> impl Iterator<Item = bool> {
        assert!(
            row <= (self.n_rows() - 1),
            "row cannot be greater than n_rows"
        );
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

    #[test]
    #[should_panic(expected = "col cannot be greater than n_col")]
    fn set_col_oob_panics() {
        let mut bb = BitBoardStatic::<1>::new(3, 3);
        bb.set_col(5, true);
    }

    #[test]
    #[should_panic(expected = "row cannot be greater than n_rows")]
    fn set_row_oob_panics() {
        let mut bb = BitBoardStatic::<1>::new(3, 3);
        bb.set_row(5, true);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_col_of_oob() {
        let bb = BitBoardStatic::<1>::new(3, 3);
        bb.row_col_of(9);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn row_col_of_way_oob() {
        let bb = BitBoardStatic::<1>::new(2, 2);
        bb.row_col_of(100);
    }

    #[rstest]
    #[case(0, 0, false)]
    #[case(0, 1, true)]
    #[case(1, 0, false)]
    #[case(1, 1, false)]
    fn get_2x2(#[case] row: usize, #[case] col: usize, #[case] expected: bool) {
        let mut bb = BitBoardStatic::<1>::new(2, 2);
        bb.set(0, 1, true);
        assert_eq!(bb.get(row, col), expected);
    }

    #[test]
    #[should_panic(expected = "row cannot be greater than n_rows")]
    fn get_oob_row_panics() {
        let bb = BitBoardStatic::<1>::new(2, 2);
        bb.get(2, 0);
    }

    #[test]
    #[should_panic(expected = "col cannot be greater than n_col")]
    fn get_oob_col_panics() {
        let bb = BitBoardStatic::<1>::new(2, 2);
        bb.get(0, 2);
    }

    #[rstest]
    #[case(0, 0, Some(false))]
    #[case(0, 1, Some(true))]
    #[case(1, 0, Some(false))]
    #[case(1, 1, Some(false))]
    #[case(2, 0, None)]
    #[case(0, 2, None)]
    #[case(2, 2, None)]
    fn try_get_2x2(#[case] row: usize, #[case] col: usize, #[case] expected: Option<bool>) {
        let mut bb = BitBoardStatic::<1>::new(2, 2);
        bb.set(0, 1, true);
        assert_eq!(bb.try_get(row, col), expected);
    }

    #[rstest]
    #[case(0, vec![true, false, true])]
    #[case(1, vec![false, true, false])]
    #[case(2, vec![true, true, true])]
    fn get_row_3x3(#[case] row: usize, #[case] expected: Vec<bool>) {
        let mut bb = BitBoardStatic::<1>::new(3, 3);
        bb.set(0, 0, true);
        bb.set(0, 2, true);
        bb.set(1, 1, true);
        bb.set_row(2, true);

        assert_eq!(bb.get_row(row).collect::<Vec<bool>>(), expected);
    }

    #[test]
    #[should_panic(expected = "row cannot be greater than n_rows")]
    fn get_row_oob_panics() {
        let bb = BitBoardStatic::<1>::new(3, 3);
        let _ = bb.get_row(3).collect::<Vec<bool>>();
    }

    #[rstest]
    #[case(0, vec![true, false, true])]
    #[case(1, vec![false, true, true])]
    #[case(2, vec![true, false, true])]
    fn get_col_3x3(#[case] col: usize, #[case] expected: Vec<bool>) {
        let mut bb = BitBoardStatic::<1>::new(3, 3);
        bb.set(0, 0, true);
        bb.set(0, 2, true);
        bb.set(1, 1, true);
        bb.set(2, 0, true);
        bb.set(2, 1, true);
        bb.set(2, 2, true);

        assert_eq!(bb.get_col(col).collect::<Vec<bool>>(), expected);
    }

    #[test]
    #[should_panic(expected = "col cannot be greater than n_col")]
    fn get_col_oob_panics() {
        let bb = BitBoardStatic::<1>::new(3, 3);
        let _ = bb.get_col(3).collect::<Vec<bool>>();
    }
}
