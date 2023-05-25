use crate::common::gate;

const ONE: [bool; 16] = [
    true, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false,
];

pub fn halfadder(a: bool, b: bool) -> [bool; 2] {
    [gate::xor(a, b), gate::and(a, b)]
}

pub fn fulladder(a: bool, b: bool, c: bool) -> [bool; 2] {
    [
        halfadder(halfadder(a, b)[0], c)[0],
        gate::or(halfadder(a, b)[1], halfadder(halfadder(a, b)[0], c)[1]),
    ]
}

pub fn add16(a: [bool; 16], b: [bool; 16]) -> [bool; 16] {
    let mut ans: [bool; 16] = [false; 16];
    let mut c = false;
    for i in 0..16 {
        [ans[i], c] = fulladder(a[i], b[i], c);
    }
    ans
}

pub fn inc16(input: [bool; 16]) -> [bool; 16] {
    add16(input, ONE)
}

pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> ([bool; 16], bool, bool) {
    (
        gate::flip(
            gate::mux16(
                gate::and16(
                    gate::flip(gate::zero(x, zx), nx),
                    gate::flip(gate::zero(y, zy), ny),
                ),
                add16(
                    gate::flip(gate::zero(x, zx), nx),
                    gate::flip(gate::zero(y, zy), ny),
                ),
                f,
            ),
            no,
        ),
        gate::and16way(gate::not16(gate::flip(
            gate::mux16(
                gate::and16(
                    gate::flip(gate::zero(x, zx), nx),
                    gate::flip(gate::zero(y, zy), ny),
                ),
                add16(
                    gate::flip(gate::zero(x, zx), nx),
                    gate::flip(gate::zero(y, zy), ny),
                ),
                f,
            ),
            no,
        ))),
        gate::and(
            gate::flip(
                gate::mux16(
                    gate::and16(
                        gate::flip(gate::zero(x, zx), nx),
                        gate::flip(gate::zero(y, zy), ny),
                    ),
                    add16(
                        gate::flip(gate::zero(x, zx), nx),
                        gate::flip(gate::zero(y, zy), ny),
                    ),
                    f,
                ),
                no,
            )[15],
            true,
        ),
    )
}
