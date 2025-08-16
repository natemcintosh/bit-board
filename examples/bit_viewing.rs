use bit_board::BitBoard;

fn main() {
    let mut bb = BitBoard::new(4, 13);
    bb.set(0, 0, true);
    println!("{bb}");
}
