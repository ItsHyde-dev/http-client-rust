pub struct Brackets {
    open: i32,
    close: i32,
}

impl Brackets {
    pub fn new() -> Self {
        Brackets { open: 0, close: 0 }
    }

    pub fn open(&mut self) {
        self.open += 1;
    }

    pub fn close(&mut self) {
        self.close += 1;
    }

    pub fn is_valid(&self) -> bool {
        return self.open == self.close;
    }
}
