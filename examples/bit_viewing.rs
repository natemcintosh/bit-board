use bitvec::prelude::*;

fn main() {
    let data = 0x1u16;
    let lsb_bits = data.view_bits::<Lsb0>();
    let msb_bits = data.view_bits::<Msb0>();
    println!("{}", lsb_bits);
    println!("{}", msb_bits);
}
