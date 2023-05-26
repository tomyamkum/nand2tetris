extern crate nand2tetris;
use nand2tetris::common::alu;
use nand2tetris::common::gate;
//use nand2tetris::common::dff;
use nand2tetris::common::memory;

const ZERO: [bool; 16] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];
const ONE: [bool; 16] = [
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];
const MINUS_ONE: [bool; 16] = [
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
];

fn minus_input(mut x: [bool; 16]) -> [bool; 16] {
    x = gate::flip(x, true);
    x = alu::inc16(x);
    x
}

#[test]
fn nand_test() {
    assert_eq!(false, gate::nand(true, true));
    assert_eq!(true, gate::nand(true, false));
    assert_eq!(true, gate::nand(false, true));
    assert_eq!(true, gate::nand(false, false));
}

#[test]
fn not_test() {
    assert_eq!(true, gate::not(false));
    assert_eq!(false, gate::not(true));
}

#[test]
fn and_test() {
    assert_eq!(true, gate::and(true, true));
    assert_eq!(false, gate::and(true, false));
    assert_eq!(false, gate::and(false, true));
    assert_eq!(false, gate::and(false, false));
}

#[test]
fn or_test() {
    assert_eq!(true, gate::or(true, true));
    assert_eq!(true, gate::or(true, false));
    assert_eq!(true, gate::or(false, true));
    assert_eq!(false, gate::or(false, false));
}

#[test]
fn xor_test() {
    assert_eq!(false, gate::xor(true, true));
    assert_eq!(true, gate::xor(true, false));
    assert_eq!(true, gate::xor(false, true));
    assert_eq!(false, gate::xor(false, false));
}

#[test]
fn mux_test() {
    for a in [true, false] {
        for b in [true, false] {
            for sel in [true, false] {
                if sel {
                    assert_eq!(b, gate::mux(a, b, sel));
                } else {
                    assert_eq!(a, gate::mux(a, b, sel));
                }
            }
        }
    }
}

#[test]
fn dmux_test() {
    for input in [true, false] {
        for sel in [true, false] {
            if sel {
                assert_eq!([false, input], gate::dmux(input, sel));
            } else {
                assert_eq!([input, false], gate::dmux(input, sel));
            }
        }
    }
}

#[test]
fn dmux4way_test() {
    assert_eq!(
        [true, false, false, false],
        gate::dmux4way(true, [false, false])
    );
    assert_eq!(
        [false, true, false, false],
        gate::dmux4way(true, [true, false])
    );
    assert_eq!(
        [false, false, true, false],
        gate::dmux4way(true, [false, true])
    );
    assert_eq!(
        [false, false, false, true],
        gate::dmux4way(true, [true, true])
    );
}

#[test]
fn dmux8way_test() {
    assert_eq!(
        [true, false, false, false, false, false, false, false],
        gate::dmux8way(true, [false, false, false])
    );
    assert_eq!(
        [false, true, false, false, false, false, false, false],
        gate::dmux8way(true, [true, false, false])
    );
    assert_eq!(
        [false, false, true, false, false, false, false, false],
        gate::dmux8way(true, [false, true, false])
    );
    assert_eq!(
        [false, false, false, true, false, false, false, false],
        gate::dmux8way(true, [true, true, false])
    );
    assert_eq!(
        [false, false, false, false, true, false, false, false],
        gate::dmux8way(true, [false, false, true])
    );
    assert_eq!(
        [false, false, false, false, false, true, false, false],
        gate::dmux8way(true, [true, false, true])
    );
    assert_eq!(
        [false, false, false, false, false, false, true, false],
        gate::dmux8way(true, [false, true, true])
    );
    assert_eq!(
        [false, false, false, false, false, false, false, true],
        gate::dmux8way(true, [true, true, true])
    );
}

#[test]
#[rustfmt::skip]
fn not16_test() {
    let a: [bool; 16] = [true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false];
    assert_eq!([false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true], gate::not16(a));
}

#[test]
#[rustfmt::skip]
fn mux4way16_test() {
    let a: [bool; 16] = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
    let b: [bool; 16] = [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]; 
    let c: [bool; 16] = [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]; 
    let d: [bool; 16] = [true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]; 

    assert_eq!(a, gate::mux4way16(a, b, c, d, [false, false]));
    assert_eq!(b, gate::mux4way16(a, b, c, d, [true, false]));
    assert_eq!(c, gate::mux4way16(a, b, c, d, [false, true]));
    assert_eq!(d, gate::mux4way16(a, b, c, d, [true, true]));
}

#[test]
#[rustfmt::skip]
fn mux8way16_test() {
    let a: [bool; 16] = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false];
    let b: [bool; 16] = [true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false]; 
    let c: [bool; 16] = [true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false]; 
    let d: [bool; 16] = [true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false]; 
    let e: [bool; 16] = [true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false]; 
    let f: [bool; 16] = [true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false]; 
    let g: [bool; 16] = [true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false]; 
    let h: [bool; 16] = [true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false]; 

    assert_eq!(a, gate::mux8way16(a, b, c, d, e, f, g, h, [false, false, false]));
    assert_eq!(b, gate::mux8way16(a, b, c, d, e, f, g, h, [true, false, false]));
    assert_eq!(c, gate::mux8way16(a, b, c, d, e, f, g, h, [false, true, false]));
    assert_eq!(d, gate::mux8way16(a, b, c, d, e, f, g, h, [true, true, false]));
    assert_eq!(e, gate::mux8way16(a, b, c, d, e, f, g, h, [false, false, true]));
    assert_eq!(f, gate::mux8way16(a, b, c, d, e, f, g, h, [true, false, true]));
    assert_eq!(g, gate::mux8way16(a, b, c, d, e, f, g, h, [false, true, true]));
    assert_eq!(h, gate::mux8way16(a, b, c, d, e, f, g, h, [true, true, true]));
}

#[test]
fn halfadder_test() {
    assert_eq!([false, false], alu::halfadder(false, false));
    assert_eq!([true, false], alu::halfadder(true, false));
    assert_eq!([true, false], alu::halfadder(false, true));
    assert_eq!([false, true], alu::halfadder(true, true));
}

#[test]
fn fulladder_test() {
    assert_eq!([false, false], alu::fulladder(false, false, false));
    assert_eq!([true, false], alu::fulladder(false, false, true));
    assert_eq!([true, false], alu::fulladder(false, true, false));
    assert_eq!([false, true], alu::fulladder(false, true, true));
    assert_eq!([true, false], alu::fulladder(true, false, false));
    assert_eq!([false, true], alu::fulladder(true, false, true));
    assert_eq!([false, true], alu::fulladder(true, true, false));
    assert_eq!([true, true], alu::fulladder(true, true, true));
}

#[test]
fn zero_test() {
    let x: [bool; 16] = [
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false,
    ];
    assert_eq!(ZERO, gate::zero(x, true));
}

#[test]
fn alu_test() {
    let x: [bool; 16] = [
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false,
    ];
    let y: [bool; 16] = [
        true, false, true, false, true, false, true, false, false, false, false, false, false,
        false, false, false,
    ];
    assert_eq!(
        (ZERO, true, false),
        alu::alu(x, y, true, false, true, false, true, false)
    );
    assert_eq!(
        (ONE, false, false),
        alu::alu(x, y, true, true, true, true, true, true)
    );
    assert_eq!(
        (MINUS_ONE, false, true),
        alu::alu(x, y, true, true, true, false, true, false)
    );
    assert_eq!(
        (MINUS_ONE, false, true),
        alu::alu(x, y, true, false, true, true, true, false)
    );
    assert_eq!(
        (x, false, false),
        alu::alu(x, y, false, false, true, true, false, false)
    );
    assert_eq!(
        (y, false, false),
        alu::alu(x, y, true, true, false, false, false, false)
    );
    assert_eq!(
        (gate::flip(x, true), false, true),
        alu::alu(x, y, false, false, true, true, false, true)
    );
    assert_eq!(
        (gate::flip(y, true), false, true),
        alu::alu(x, y, true, true, false, false, false, true)
    );
    assert_eq!(
        (minus_input(x), false, true),
        alu::alu(x, y, false, false, true, true, true, true)
    );
    assert_eq!(
        (minus_input(y), false, true),
        alu::alu(x, y, true, true, false, false, true, true)
    );
    assert_eq!(
        (alu::inc16(x), false, false),
        alu::alu(x, y, false, true, true, true, true, true)
    );
    assert_eq!(
        (alu::inc16(y), false, false),
        alu::alu(x, y, true, true, false, true, true, true)
    );
    assert_eq!(
        (alu::add16(x, MINUS_ONE), false, false),
        alu::alu(x, y, false, false, true, true, true, false)
    );
    assert_eq!(
        (alu::add16(y, MINUS_ONE), false, false),
        alu::alu(x, y, true, true, false, false, true, false)
    );
    assert_eq!(
        (alu::add16(x, y), false, false),
        alu::alu(x, y, false, false, false, false, true, false)
    );
    assert_eq!(
        (alu::add16(x, minus_input(y)), false, false),
        alu::alu(x, y, false, true, false, false, true, true)
    );
    assert_eq!(
        (alu::add16(minus_input(x), y), false, true),
        alu::alu(x, y, false, false, false, true, true, true)
    );
    assert_eq!(
        (gate::and16(x, y), false, false),
        alu::alu(x, y, false, false, false, false, false, false)
    );
    assert_eq!(
        (gate::or16(x, y), false, false),
        alu::alu(x, y, false, true, false, true, false, true)
    );
}

#[test]
fn bit_test() {
    let init_state: bool = true;
    let mut bit: memory::Bit = memory::Bit::new(init_state);
    assert_eq!(true, bit.load(false, false));
    assert_eq!(true, bit.load(false, true));
    assert_eq!(false, bit.load(true, false));
    assert_eq!(false, bit.load(true, true));
    assert_eq!(true, bit.load(false, false));
}

#[test]
fn register_test() {
    let mut reg: memory::Register = memory::Register::new();
    let input1: [bool; 16] = [
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false,
    ];
    let input2: [bool; 16] = [
        false, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false,
    ];
    let load: [bool; 16] = [true; 16];
    reg.load(input1, load);
    assert_eq!(input1, reg.load(input2, load));
    assert_eq!(input2, reg.load(input2, load));
}
