pub mod common;
use common::gate;

fn main() {
    let mut a: [bool; 16] = [true, true, true, true, true, false, false, true, false, false, true, true, false, false, false, false];
    a = gate::not16(a);
    println!("{:?}", a);
}
