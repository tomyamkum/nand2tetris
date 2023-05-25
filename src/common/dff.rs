pub struct Dff {
    pub state: bool,
}

impl Dff {
    pub fn new(init_state: bool) -> Dff {
        Dff { state: init_state }
    }

    pub fn load(&mut self, input: bool) -> bool {
        let result = self.state;
        self.state = input;
        result
    }
}
