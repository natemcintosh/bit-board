use bit_board::bitboard::BitBoard;
use bit_board::bitboarddyn::BitBoardDyn;

fn main() {
    let mut bb = BitBoardDyn::new(4, 13);
    bb.set(0, 0, true);
    bb.set_col(4, true);
    println!("{bb}");
}
