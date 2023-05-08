extern crate nand2tetris;
use nand2tetris::common::gate;

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
        assert_eq!((false, input), gate::dmux(input, sel))
      }
      else {
        assert_eq!((input, false), gate::dmux(input, sel))
      }
    }
  }
}
