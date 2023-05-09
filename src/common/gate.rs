pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(a: bool) -> bool {
    nand(a, true)
}

pub fn not16(mut a: [bool; 16]) -> [bool; 16] {
    for i in 0..16 {
        a[i] = not(a[i]);
    }
    a
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a, b))
}

pub fn and16(

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn xor(a: bool, b: bool) -> bool {
    and(nand(a, b), or(a, b))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    or(and(sel, b), and(not(sel), a))
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
    (and(input, not(sel)), and(input, sel))
}
