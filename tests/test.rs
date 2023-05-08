extern crate nand_to_tetris;

#[test]
fn nand_test() {
  assert_eq!(false, nand_to_tetris::common::gate::nand(true, true));
  assert_eq!(true, nand_to_tetris::common::gate::nand(true, false));
  assert_eq!(true, nand_to_tetris::common::gate::nand(false, true));
  assert_eq!(true, nand_to_tetris::common::gate::nand(false, false));
}

