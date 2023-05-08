pub mod common;

fn main() {
    let a: bool = common::gate::nand(false, true);
    println!("{}", a);
}
