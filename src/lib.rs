use std::error::Error;
use std::fmt;

use bitvec::prelude::*;

use crate::bitboard::BitBoard;

pub mod bitboard;

#[derive(Debug)]
pub struct DimensionMismatch;

impl fmt::Display for DimensionMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dimensions do not match.")
    }
}

impl Error for DimensionMismatch {}

/// BitBoard is a 2D array of booleans, stored in the bits of integers. It does
/// assumes that the boundaries are hard, and going past a boundary does *not* take
/// you back to the other side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitBoardDyn {
    // The slice of bits that represent the board.
    pub board: BitVec,

    /// How many rows does the board have
    pub n_rows: usize,

    /// How many columns does the board have
    pub n_cols: usize,
}

impl fmt::Display for BitBoardDyn {
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

impl BitBoard for BitBoardDyn {
    fn n_rows(&self) -> usize {
        self.n_rows
    }

    fn n_cols(&self) -> usize {
        self.n_cols
    }

    fn board(&mut self) -> &mut BitSlice {
        &mut self.board
    }

    fn or(&self, other: &BitBoardDyn) -> Result<BitBoardDyn, DimensionMismatch> {
        if (self.n_rows != other.n_rows) || (self.n_cols != other.n_cols) {
            return Err(DimensionMismatch);
        }
        let mut new_board = BitBoardDyn::new(self.n_rows, self.n_cols);
        new_board.board = self.board.clone() | other.board.clone();
        Ok(new_board)
    }

    fn and(&self, other: &BitBoardDyn) -> Result<BitBoardDyn, DimensionMismatch> {
        if (self.n_rows != other.n_rows) || (self.n_cols != other.n_cols) {
            return Err(DimensionMismatch);
        }
        let mut new_board = BitBoardDyn::new(self.n_rows, self.n_cols);
        new_board.board = self.board.clone() & other.board.clone();
        Ok(new_board)
    }
}

impl BitBoardDyn {
    /// Create a new empty board with `n_rows` and `n_cols`.
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        BitBoardDyn {
            board: bitvec![0; n_rows * n_cols],
            n_rows,
            n_cols,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn can_construct() {
        let bb = BitBoardDyn::new(2, 2);
        println!("{:?}", bb);
    }

    #[test]
    #[should_panic(expected = "row cannot be greater than n_rows")]
    fn row_too_big() {
        let bb = BitBoardDyn::new(2, 2);
        bb.index_of(10, 0);
    }

    #[test]
    #[should_panic(expected = "col cannot be greater than n_col")]
    fn col_too_big() {
        let bb = BitBoardDyn::new(2, 2);
        bb.index_of(0, 10);
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(0, 1, 1)]
    #[case(1, 0, 2)]
    #[case(1, 1, 3)]
    fn index_of(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let bb = BitBoardDyn::new(2, 2);
        assert_eq!(expected, bb.index_of(row, col))
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(1, 0, 1)]
    #[case(2, 0, 2)]
    #[case(3, 0, 3)]
    #[case(4, 0, 4)]
    fn col_vec_index_of(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let bb = BitBoardDyn::new(5, 1);
        assert_eq!(expected, bb.index_of(row, col))
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(0, 1, 1)]
    #[case(0, 2, 2)]
    #[case(0, 3, 3)]
    #[case(0, 4, 4)]
    fn row_vec_index_of(#[case] row: usize, #[case] col: usize, #[case] expected: usize) {
        let bb = BitBoardDyn::new(1, 5);
        assert_eq!(expected, bb.index_of(row, col))
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    #[case(4)]
    fn set_col(#[case] col: usize) {
        let mut bb = BitBoardDyn::new(5, 5);
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
        let mut bb = BitBoardDyn::new(5, 5);
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
        let mut bb = BitBoardDyn::new(nr, nc);

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
    #[case(0, 0, BitBoardDyn { board: bitvec![0, 1, 1, 0], n_rows: 2, n_cols: 2 })]
    #[case(0, 1, BitBoardDyn { board: bitvec![1, 0, 0, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 0, BitBoardDyn { board: bitvec![1, 0, 0, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 1, BitBoardDyn { board: bitvec![0, 1, 1, 0], n_rows: 2, n_cols: 2 })]
    fn set_caridnal_neighbors_2x2(
        #[case] row: usize,
        #[case] col: usize,
        #[case] expect: BitBoardDyn,
    ) {
        let mut bb = BitBoardDyn::new(2, 2);
        bb.set_cardinal_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(0, 0, BitBoardDyn { board: bitvec![0, 1, 0, 1, 0, 0, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 1, BitBoardDyn { board: bitvec![1, 0, 1, 0, 1, 0, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 2, BitBoardDyn { board: bitvec![0, 1, 0, 0, 0, 1, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 0, BitBoardDyn { board: bitvec![1, 0, 0, 0, 1, 0, 1, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 1, BitBoardDyn { board: bitvec![0, 1, 0, 1, 0, 1, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 2, BitBoardDyn { board: bitvec![0, 0, 1, 0, 1, 0, 0, 0, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 0, BitBoardDyn { board: bitvec![0, 0, 0, 1, 0, 0, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(2, 1, BitBoardDyn { board: bitvec![0, 0, 0, 0, 1, 0, 1, 0, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 2, BitBoardDyn { board: bitvec![0, 0, 0, 0, 0, 1, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    fn set_caridnal_neighbors_3x3(
        #[case] row: usize,
        #[case] col: usize,
        #[case] expect: BitBoardDyn,
    ) {
        let mut bb = BitBoardDyn::new(3, 3);
        bb.set_cardinal_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(0, 0, BitBoardDyn { board: bitvec![0, 1, 1, 1], n_rows: 2, n_cols: 2 })]
    #[case(0, 1, BitBoardDyn { board: bitvec![1, 0, 1, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 0, BitBoardDyn { board: bitvec![1, 1, 0, 1], n_rows: 2, n_cols: 2 })]
    #[case(1, 1, BitBoardDyn { board: bitvec![1, 1, 1, 0], n_rows: 2, n_cols: 2 })]
    fn set_all_neighbors_2x2(#[case] row: usize, #[case] col: usize, #[case] expect: BitBoardDyn) {
        let mut bb = BitBoardDyn::new(2, 2);
        bb.set_all_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(0, 0, BitBoardDyn { board: bitvec![0, 1, 0, 1, 1, 0, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 1, BitBoardDyn { board: bitvec![1, 0, 1, 1, 1, 1, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(0, 2, BitBoardDyn { board: bitvec![0, 1, 0, 0, 1, 1, 0, 0, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 0, BitBoardDyn { board: bitvec![1, 1, 0, 0, 1, 0, 1, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(1, 1, BitBoardDyn { board: bitvec![1, 1, 1, 1, 0, 1, 1, 1, 1], n_rows: 3, n_cols: 3 })]
    #[case(1, 2, BitBoardDyn { board: bitvec![0, 1, 1, 0, 1, 0, 0, 1, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 0, BitBoardDyn { board: bitvec![0, 0, 0, 1, 1, 0, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    #[case(2, 1, BitBoardDyn { board: bitvec![0, 0, 0, 1, 1, 1, 1, 0, 1], n_rows: 3, n_cols: 3 })]
    #[case(2, 2, BitBoardDyn { board: bitvec![0, 0, 0, 0, 1, 1, 0, 1, 0], n_rows: 3, n_cols: 3 })]
    fn set_all_neighbors_3x3(#[case] row: usize, #[case] col: usize, #[case] expect: BitBoardDyn) {
        let mut bb = BitBoardDyn::new(3, 3);
        bb.set_all_neighbors(row, col, true);
        assert_eq!(expect, bb);
    }

    #[rstest]
    #[case(1, 1, 1, 2)]
    #[case(2, 1, 1, 2)]
    #[case(2, 1, 2, 7)]
    fn and_dimension_mismatch(
        #[case] b1r: usize,
        #[case] b1c: usize,
        #[case] b2r: usize,
        #[case] b2c: usize,
    ) {
        let bb1 = BitBoardDyn::new(b1r, b1c);
        let bb8 = BitBoardDyn::new(b2r, b2c);
        assert!(bb1.and(&bb8).is_err());
    }

    #[rstest]
    #[case(1, 1, 1, 2)]
    #[case(2, 1, 1, 2)]
    #[case(2, 1, 2, 7)]
    fn or_dimension_mismatch(
        #[case] b1r: usize,
        #[case] b1c: usize,
        #[case] b2r: usize,
        #[case] b2c: usize,
    ) {
        let bb1 = BitBoardDyn::new(b1r, b1c);
        let bb8 = BitBoardDyn::new(b2r, b2c);
        assert!(bb1.or(&bb8).is_err());
    }

    #[rstest]
    #[case(bitvec![0, 0, 0, 0], bitvec![0, 0, 0, 0], bitvec![0, 0, 0, 0])] // empty AND empty
    #[case(bitvec![1, 1, 1, 1], bitvec![1, 1, 1, 1], bitvec![1, 1, 1, 1])] // full AND full
    #[case(bitvec![0, 0, 0, 0], bitvec![1, 1, 1, 1], bitvec![0, 0, 0, 0])] // empty AND full
    #[case(bitvec![1, 1, 1, 1], bitvec![1, 0, 0, 1], bitvec![1, 0, 0, 1])] // full AND partial
    #[case(bitvec![1, 0, 1, 0], bitvec![0, 1, 0, 1], bitvec![0, 0, 0, 0])] // alternating patterns
    #[case(bitvec![1, 1, 0, 0], bitvec![1, 0, 1, 0], bitvec![1, 0, 0, 0])] // partial patterns
    fn and_operations(#[case] board1: BitVec, #[case] board2: BitVec, #[case] expected: BitVec) {
        let bb1 = BitBoardDyn {
            board: board1,
            n_rows: 2,
            n_cols: 2,
        };
        let bb2 = BitBoardDyn {
            board: board2,
            n_rows: 2,
            n_cols: 2,
        };

        let result = bb1.and(&bb2).unwrap();
        assert_eq!(result.board, expected);
        assert_eq!(result.n_rows, 2);
        assert_eq!(result.n_cols, 2);
    }

    #[rstest]
    #[case(bitvec![0, 0, 0, 0], bitvec![0, 0, 0, 0], bitvec![0, 0, 0, 0])] // empty OR empty
    #[case(bitvec![1, 1, 1, 1], bitvec![1, 1, 1, 1], bitvec![1, 1, 1, 1])] // full OR full
    #[case(bitvec![0, 0, 0, 0], bitvec![1, 1, 1, 1], bitvec![1, 1, 1, 1])] // empty OR full
    #[case(bitvec![0, 0, 0, 0], bitvec![1, 0, 0, 1], bitvec![1, 0, 0, 1])] // empty OR partial
    #[case(bitvec![1, 0, 1, 0], bitvec![0, 1, 0, 1], bitvec![1, 1, 1, 1])] // alternating patterns
    #[case(bitvec![1, 1, 0, 0], bitvec![0, 0, 1, 1], bitvec![1, 1, 1, 1])] // complementary patterns
    #[case(bitvec![1, 0, 0, 1], bitvec![0, 1, 1, 0], bitvec![1, 1, 1, 1])] // diagonal patterns
    fn or_operations(#[case] board1: BitVec, #[case] board2: BitVec, #[case] expected: BitVec) {
        let bb1 = BitBoardDyn {
            board: board1,
            n_rows: 2,
            n_cols: 2,
        };
        let bb2 = BitBoardDyn {
            board: board2,
            n_rows: 2,
            n_cols: 2,
        };

        let result = bb1.or(&bb2).unwrap();
        assert_eq!(result.board, expected);
        assert_eq!(result.n_rows, 2);
        assert_eq!(result.n_cols, 2);
    }

    #[test]
    fn and_or_larger_boards() {
        let mut bb1 = BitBoardDyn::new(3, 3);
        bb1.set_row(0, true); // First row all true
        bb1.set(2, 2, true); // Bottom right corner

        let mut bb2 = BitBoardDyn::new(3, 3);
        bb2.set_col(0, true); // First column all true
        bb2.set(1, 1, true); // Center

        // Test AND operation
        let and_result = bb1.and(&bb2).unwrap();
        assert!(and_result.board[0]); // (0,0) - both have true
        assert!(!and_result.board[1]); // (0,1) - only bb1 has true
        assert!(!and_result.board[2]); // (0,2) - only bb1 has true
        assert!(!and_result.board[3]); // (1,0) - only bb2 has true
        assert!(!and_result.board[4]); // (1,1) - only bb2 has true
        assert!(!and_result.board[6]); // (2,0) - only bb2 has true

        // Test OR operation
        let or_result = bb1.or(&bb2).unwrap();
        assert!(or_result.board[0]); // (0,0) - both have true
        assert!(or_result.board[1]); // (0,1) - bb1 has true
        assert!(or_result.board[2]); // (0,2) - bb1 has true
        assert!(or_result.board[3]); // (1,0) - bb2 has true
        assert!(or_result.board[4]); // (1,1) - bb2 has true
        assert!(!or_result.board[5]); // (1,2) - neither has true
        assert!(or_result.board[6]); // (2,0) - bb2 has true
        assert!(!or_result.board[7]); // (2,1) - neither has true
        assert!(or_result.board[8]); // (2,2) - bb1 has true
    }

    #[test]
    fn and_or_preserve_original_boards() {
        let mut bb1 = BitBoardDyn::new(2, 2);
        bb1.set(0, 0, true);
        let bb1_original = bb1.clone();

        let mut bb2 = BitBoardDyn::new(2, 2);
        bb2.set(1, 1, true);
        let bb2_original = bb2.clone();

        // Perform operations
        let _and_result = bb1.and(&bb2).unwrap();
        let _or_result = bb1.or(&bb2).unwrap();

        // Original boards should be unchanged
        assert_eq!(bb1, bb1_original);
        assert_eq!(bb2, bb2_original);
    }
}
