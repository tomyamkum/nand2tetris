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

    pub fn load(&mut self, input: [bool; 16], load: bool) -> [bool; 16] {
        [
            self.reg[0].load(input[0], load),
            self.reg[1].load(input[1], load),
            self.reg[2].load(input[2], load),
            self.reg[3].load(input[3], load),
            self.reg[4].load(input[4], load),
            self.reg[5].load(input[5], load),
            self.reg[6].load(input[6], load),
            self.reg[7].load(input[7], load),
            self.reg[8].load(input[8], load),
            self.reg[9].load(input[9], load),
            self.reg[10].load(input[10], load),
            self.reg[11].load(input[11], load),
            self.reg[12].load(input[12], load),
            self.reg[13].load(input[13], load),
            self.reg[14].load(input[14], load),
            self.reg[15].load(input[15], load),
        ]
    }
}

pub struct RAM8 {
    pub ram8: [Register; 8],
}

impl RAM8 {
    pub fn new() -> RAM8 {
        RAM8 {
            ram8: [
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
                Register::new(),
            ],
        }
    }

    pub fn load(&mut self, input: [bool; 16], address: [bool; 3], load: bool) -> [bool; 16] {
        gate::mux8way16(
            self.ram8[0].load(input, gate::dmux8way(load, address)[0]),
            self.ram8[1].load(input, gate::dmux8way(load, address)[1]),
            self.ram8[2].load(input, gate::dmux8way(load, address)[2]),
            self.ram8[3].load(input, gate::dmux8way(load, address)[3]),
            self.ram8[4].load(input, gate::dmux8way(load, address)[4]),
            self.ram8[5].load(input, gate::dmux8way(load, address)[5]),
            self.ram8[6].load(input, gate::dmux8way(load, address)[6]),
            self.ram8[7].load(input, gate::dmux8way(load, address)[7]),
            address,
        )
    }
}

pub struct RAM64 {
    pub ram64: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> RAM64 {
        RAM64 {
            ram64: [
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
                RAM8::new(),
            ],
        }
    }

    pub fn load(&mut self, input: [bool; 16], address: [bool; 6], load: bool) -> [bool; 16] {
        gate::mux8way16(
            self.ram64[0].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[0],
            ),
            self.ram64[1].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[1],
            ),
            self.ram64[2].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[2],
            ),
            self.ram64[3].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[3],
            ),
            self.ram64[4].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[4],
            ),
            self.ram64[5].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[5],
            ),
            self.ram64[6].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[6],
            ),
            self.ram64[7].load(
                input,
                [address[0], address[1], address[2]],
                gate::dmux8way(load, [address[3], address[4], address[5]])[7],
            ),
            [address[3], address[4], address[5]],
        )
    }
}

pub struct RAM512 {
    pub ram512: [RAM64; 8],
}

impl RAM512 {
    pub fn new() -> RAM512 {
        RAM512 {
            ram512: [
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
                RAM64::new(),
            ],
        }
    }

    pub fn load(&mut self, input: [bool; 16], address: [bool; 9], load: bool) -> [bool; 16] {
        gate::mux8way16(
            self.ram512[0].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[0],
            ),
            self.ram512[1].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[1],
            ),
            self.ram512[2].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[2],
            ),
            self.ram512[3].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[3],
            ),
            self.ram512[4].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[4],
            ),
            self.ram512[5].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[5],
            ),
            self.ram512[6].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[6],
            ),
            self.ram512[7].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                ],
                gate::dmux8way(load, [address[6], address[7], address[8]])[7],
            ),
            [address[6], address[7], address[8]],
        )
    }
}

pub struct RAM4K {
    pub ram4k: [RAM512; 8],
}

impl RAM4K {
    pub fn new() -> RAM4K {
        RAM4K {
            ram4k: [
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
                RAM512::new(),
            ],
        }
    }

    pub fn load(&mut self, input: [bool; 16], address: [bool; 12], load: bool) -> [bool; 16] {
        gate::mux8way16(
            self.ram4k[0].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[0],
            ),
            self.ram4k[1].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[1],
            ),
            self.ram4k[2].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[2],
            ),
            self.ram4k[3].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[3],
            ),
            self.ram4k[4].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[4],
            ),
            self.ram4k[5].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[5],
            ),
            self.ram4k[6].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[6],
            ),
            self.ram4k[7].load(
                input,
                [
                    address[0], address[1], address[2], address[3], address[4], address[5],
                    address[6], address[7], address[8],
                ],
                gate::dmux8way(load, [address[9], address[10], address[11]])[7],
            ),
            [address[9], address[10], address[11]],
        )
    }
}

pub struct RAM16K {
    pub ram16k: [RAM4K; 4],
}

impl RAM16K {
    pub fn new() -> RAM16K {
        RAM16K {
            ram16k: [RAM4K::new(), RAM4K::new(), RAM4K::new(), RAM4K::new()],
        }
    }

    pub fn load(&mut self, input: [bool; 16], address: [bool; 14], load: bool) -> [bool; 16] {
        gate::mux4way16(
            self.ram16k[0].load(
                input,
                [
                    address[0],
                    address[1],
                    address[2],
                    address[3],
                    address[4],
                    address[5],
                    address[6],
                    address[7],
                    address[8],
                    address[9],
                    address[10],
                    address[11],
                ],
                gate::dmux4way(load, [address[12], address[13]])[0],
            ),
            self.ram16k[1].load(
                input,
                [
                    address[0],
                    address[1],
                    address[2],
                    address[3],
                    address[4],
                    address[5],
                    address[6],
                    address[7],
                    address[8],
                    address[9],
                    address[10],
                    address[11],
                ],
                gate::dmux4way(load, [address[12], address[13]])[1],
            ),
            self.ram16k[2].load(
                input,
                [
                    address[0],
                    address[1],
                    address[2],
                    address[3],
                    address[4],
                    address[5],
                    address[6],
                    address[7],
                    address[8],
                    address[9],
                    address[10],
                    address[11],
                ],
                gate::dmux4way(load, [address[12], address[13]])[2],
            ),
            self.ram16k[3].load(
                input,
                [
                    address[0],
                    address[1],
                    address[2],
                    address[3],
                    address[4],
                    address[5],
                    address[6],
                    address[7],
                    address[8],
                    address[9],
                    address[10],
                    address[11],
                ],
                gate::dmux4way(load, [address[12], address[13]])[3],
            ),
            [address[12], address[13]],
        )
    }
}
