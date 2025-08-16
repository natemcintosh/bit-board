use std::fmt;

use bitvec::prelude::*;

/// BitBoard is a 2D array of booleans, stored in the bits of integers. It does
/// assumes that the boundaries are hard, and going past a boundary does *not* take
/// you back to the other side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitBoard {
    // The slice of bits that represent the board.
    pub board: BitVec,

    /// How many rows does the board have
    pub n_rows: usize,

    /// How many columns does the board have
    pub n_cols: usize,
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // column indices
        write!(f, "   ")?; // space for row labels
        for col in 0..self.n_cols {
            write!(f, "{}", col % 10)?; // wrap every 10 for readability
        }
        writeln!(f)?;

        for row in 0..self.n_rows {
            // row index, right-aligned to 2 spaces
            write!(f, "{:>2} ", row)?;
            for col in 0..self.n_cols {
                let idx = row * self.n_cols + col;
                let bit = self.board[idx];
                let c = if bit { 'X' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl BitBoard {
    /// Create a new board with `n_rows` and `n_cols`.
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        BitBoard {
            board: bitvec![0; n_rows * n_cols],
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

    /// Set all bits to the desired value.
    pub fn fill(&mut self, value: bool) {
        self.board.fill(value);
    }

    /// Set the value at index [row, col] to be the `new_val`.
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        let new_ind = self.index_of(row, col);
        self.board.set(new_ind, value);
    }

    /// Set an entire column to a certain value
    pub fn set_col(&mut self, col: usize, value: bool) {
        // For each row
        for r_idx in 0..self.n_rows {
            // Calculate the index
            let idx = (r_idx * self.n_cols) + col;
            self.board.set(idx, value);
        }
    }

    /// Set an entire row to a certain value
    pub fn set_row(&mut self, row: usize, value: bool) {
        // For each column in the row
        for cidx in 0..self.n_cols {
            // Calculate the index
            let idx = (row * self.n_cols) + cidx;
            self.board.set(idx, value);
        }
    }

    /// Will set the neighbors immediately above, below, left, and right to `value`. If
    /// the neighbor is out of bounds, nothing will happen
    pub fn set_cardinal_neighbors(&mut self, row: usize, col: usize, value: bool) {
        // Above
        if row > 0 {
            self.set(row - 1, col, value);
        }

        // Below
        if row < self.n_rows - 1 {
            self.set(row + 1, col, value);
        }

        // Left
        if col > 0 {
            self.set(row, col - 1, value);
        }

        // Right
        if col < self.n_cols - 1 {
            self.set(row, col + 1, value);
        }
    }

    /// Set just the spots diagonal from the given position to `value`. If
    /// the neighbor is out of bounds, nothing will happen
    pub fn set_diagonals(&mut self, row: usize, col: usize, value: bool) {
        // Above left
        if row > 0 && col > 0 {
            self.set(row - 1, col - 1, value);
        }

        // Above right
        if row > 0 && col < self.n_cols - 1 {
            self.set(row - 1, col + 1, value);
        }

        // Below left
        if row < self.n_rows - 1 && col > 0 {
            self.set(row + 1, col - 1, value);
        }

        // Below right
        if row < self.n_rows - 1 && col < self.n_cols - 1 {
            self.set(row + 1, col + 1, value);
        }
    }

    /// Set the cardinal neighbors and the diagonal neighbors to `value`. If
    /// the neighbor is out of bounds, nothing will happen
    pub fn set_all_neighbors(&mut self, row: usize, col: usize, value: bool) {
        self.set_cardinal_neighbors(row, col, value);
        self.set_diagonals(row, col, value);
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

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    fn set_col(#[case] col: usize) {
        let mut bb = BitBoard::new(5, 5);
        bb.set_col(col, true);
        for ridx in 0..bb.n_rows {
            for cidx in 0..bb.n_cols {
                if cidx == col {
                    assert!(bb.board[bb.index_of(ridx, cidx)])
                } else {
                    assert!(!bb.board[bb.index_of(ridx, cidx)])
                }
            }
        }
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    fn set_row(#[case] row: usize) {
        let mut bb = BitBoard::new(5, 5);
        bb.set_row(row, true);
        for ridx in 0..bb.n_rows {
            for cidx in 0..bb.n_cols {
                if ridx == row {
                    assert!(bb.board[bb.index_of(ridx, cidx)])
                } else {
                    assert!(!bb.board[bb.index_of(ridx, cidx)])
                }
            }
        }
    }

    #[test]
    fn can_set_all_bits() {
        // Create the board
        let nr = 3;
        let nc = 3;
        let mut bb = BitBoard::new(nr, nc);

        bb.set(0, 0, true);

        // Set each bit, and check that all bits are 1
        for ridx in 0..nr {
            for cidx in 0..nc {
                bb.set(ridx, cidx, true);
            }
        }
        assert!(bb.board.all());

        // Unset each bit, and check that all bits are 0
        for ridx in 0..nr {
            for cidx in 0..nc {
                bb.set(ridx, cidx, false);
            }
        }
        assert!(bb.board.not_any());
    }

    #[rstest]
    #[case(0, 0, BitBoard { board: bitvec![0, 1, 1, 0], n_rows: 2, n_cols: 2 })]
    #[case(0, 1, BitBoard { board: bitvec![1, 0, 0, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 0, BitBoard { board: bitvec![1, 0, 0, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 1, BitBoard { board: bitvec![0, 1, 1, 0], n_rows: 2, n_cols: 2 })]
    fn set_caridnal_neighbors_2x2(
        #[case] row: usize,
        #[case] col: usize,
        #[case] expect: BitBoard,
    ) {
        let mut bb = BitBoard::new(2, 2);
        bb.set_cardinal_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(0, 0, BitBoard { board: bitvec![0, 1, 0, 1, 0, 0, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 1, BitBoard { board: bitvec![1, 0, 1, 0, 1, 0, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 2, BitBoard { board: bitvec![0, 1, 0, 0, 0, 1, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 0, BitBoard { board: bitvec![1, 0, 0, 0, 1, 0, 1, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 1, BitBoard { board: bitvec![0, 1, 0, 1, 0, 1, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 2, BitBoard { board: bitvec![0, 0, 1, 0, 1, 0, 0, 0, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 0, BitBoard { board: bitvec![0, 0, 0, 1, 0, 0, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(2, 1, BitBoard { board: bitvec![0, 0, 0, 0, 1, 0, 1, 0, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 2, BitBoard { board: bitvec![0, 0, 0, 0, 0, 1, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    fn set_caridnal_neighbors_3x3(
        #[case] row: usize,
        #[case] col: usize,
        #[case] expect: BitBoard,
    ) {
        let mut bb = BitBoard::new(3, 3);
        bb.set_cardinal_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(0, 0, BitBoard { board: bitvec![0, 1, 1, 1], n_rows: 2, n_cols: 2 })]
    #[case(0, 1, BitBoard { board: bitvec![1, 0, 1, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 0, BitBoard { board: bitvec![1, 1, 0, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 1, BitBoard { board: bitvec![1, 1, 1, 0], n_rows: 2, n_cols: 2 })]
    fn set_all_neighbors_2x2(#[case] row: usize, #[case] col: usize, #[case] expect: BitBoard) {
        let mut bb = BitBoard::new(2, 2);
        bb.set_all_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(0, 0, BitBoard { board: bitvec![0, 1, 0, 1, 1, 0, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 1, BitBoard { board: bitvec![1, 0, 1, 1, 1, 1, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 2, BitBoard { board: bitvec![0, 1, 0, 0, 1, 1, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 0, BitBoard { board: bitvec![1, 1, 0, 0, 1, 0, 1, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 1, BitBoard { board: bitvec![1, 1, 1, 1, 0, 1, 1, 1, 1], n_rows: 3, n_cols: 3 })]
    #[case(1, 2, BitBoard { board: bitvec![0, 1, 1, 0, 1, 0, 0, 1, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 0, BitBoard { board: bitvec![0, 0, 0, 1, 1, 0, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(2, 1, BitBoard { board: bitvec![0, 0, 0, 1, 1, 1, 1, 0, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 2, BitBoard { board: bitvec![0, 0, 0, 0, 1, 1, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    fn set_all_neighbors_3x3(#[case] row: usize, #[case] col: usize, #[case] expect: BitBoard) {
        let mut bb = BitBoard::new(3, 3);
        bb.set_all_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }
}
