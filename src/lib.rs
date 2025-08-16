use bitvec::prelude::*;

/// BitBoard is a 2D array of booleans, stored in the bits of integers. It does
/// assumes that the boundaries are hard, and going past a boundary does *not* take
/// you back to the other side.
#[derive(Debug, Clone)]
pub struct BitBoard {
    // The slice of bits that represent the board.
    board: BitVec,

    /// How many rows does the board have
    pub n_rows: usize,

    /// How many columns does the board have
    pub n_cols: usize,
}

impl BitBoard {
    /// Create a new board with `n_rows` and `n_cols`.
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        BitBoard {
            board: BitVec::with_capacity(n_rows * n_cols),
            n_rows,
            n_cols,
        }
    }

    /// Get the index that we can use to directly access a certain spot on the board
    pub fn index_of(&self, row: usize, col: usize) -> usize {
        assert!(
            row <= (self.n_rows - 1),
            "row cannot be greater than n_rows"
        );
        assert!(
            col <= (self.n_cols - 1),
            "col cannot be greater than n_cols"
        );
        (row * self.n_cols) + col
    }

    /// Set the value at index [row, col] to be the `new_val`.
    pub fn set(mut self, row: usize, col: usize, value: bool) {
        let new_ind = self.index_of(row, col);
        self.board.set(new_ind, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn can_construct() {
        let bb = BitBoard::new(2, 2);
        println!("{:?}", bb);
    }

    #[test]
    #[should_panic(expected = "row cannot be greater than n_rows")]
    fn row_too_big() {
        let bb = BitBoard::new(2, 2);
        bb.index_of(10, 0);
    }

    #[test]
    #[should_panic(expected = "col cannot be greater than n_col")]
    fn col_too_big() {
        let bb = BitBoard::new(2, 2);
        bb.index_of(0, 10);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(0, 1, 1)]
    #[case(1, 0, 2)]
    #[case(1, 1, 3)]
    fn index_of(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let bb = BitBoard::new(2, 2);
        assert_eq!(expected, bb.index_of(row, col))
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 0, 1)]
    #[case(2, 0, 2)]
    #[case(3, 0, 3)]
    #[case(4, 0, 4)]
    fn col_vec_index_of(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let bb = BitBoard::new(5, 1);
        assert_eq!(expected, bb.index_of(row, col))
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(0, 1, 1)]
    #[case(0, 2, 2)]
    #[case(0, 3, 3)]
    #[case(0, 4, 4)]
    fn row_vec_index_of(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let bb = BitBoard::new(1, 5);
        assert_eq!(expected, bb.index_of(row, col))
    }
}
