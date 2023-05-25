use crate::common::dff;
use crate::common::gate;

pub struct Bit {
    pub bit: dff::Dff,
}

impl Bit {
    pub fn new(init_state: bool) -> Bit {
        Bit {
            bit: dff::Dff::new(init_state),
        }
    }

    pub fn load(&mut self, input: bool, load: bool) -> bool {
        self.bit.load(gate::mux(self.bit.state, input, load))
    }
}

pub struct Register {
    pub reg: [Bit; 16],
}

impl Register {
    pub fn new() -> Register {
        Register {
            reg: [
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
                Bit::new(false),
            ],
        }
    }

    pub fn load(&mut self, input: [bool; 16], load: [bool; 16]) -> [bool; 16] {
        [
            self.reg[0].load(input[0], load[0]),
            self.reg[1].load(input[1], load[1]),
            self.reg[2].load(input[2], load[2]),
            self.reg[3].load(input[3], load[3]),
            self.reg[4].load(input[4], load[4]),
            self.reg[5].load(input[5], load[5]),
            self.reg[6].load(input[6], load[6]),
            self.reg[7].load(input[7], load[7]),
            self.reg[8].load(input[8], load[8]),
            self.reg[9].load(input[9], load[9]),
            self.reg[10].load(input[10], load[10]),
            self.reg[11].load(input[11], load[11]),
            self.reg[12].load(input[12], load[12]),
            self.reg[13].load(input[13], load[13]),
            self.reg[14].load(input[14], load[14]),
            self.reg[15].load(input[15], load[15]),
        ]
    }
}
