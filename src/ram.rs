pub struct RAM {
    data: Vec<u8>,
    size: usize,
    curr_addr: usize,
}

impl RAM{
    pub fn new(size: usize) -> Self{
        let mut buffer = vec![0; size as usize];
        RAM{
            data: buffer,
            size,
            curr_addr: 0
        }
    }

    // pub fn read_by_bytes(&self, addr: usize) -> [u8; 4] {
    //     if !self.check_in_bounds(addr){
    //         panic!("Attempted to read outside of ram!");
    //     }
    //
    //     let w1 = ((self.data[addr] & (((1 << 8) - 1) << 24)) >> 24) as u8;
    //     let w2 = ((self.data[addr] & (((1 << 8) - 1) << 16)) >> 16) as u8;
    //     let w3 = ((self.data[addr] & (((1 << 8) - 1) << 8)) >> 8) as u8;
    //     let w4 = (self.data[addr] & ((1 << 8) - 1)) as u8;
    //     [w1, w2, w3, w4]
    // }

    pub fn read(&mut self, addr: usize) -> u8{
        if !self.check_in_bounds(addr){
            panic!("Attempted to read outside of ram!");
        }

        self.curr_addr = addr;
        self.data[addr]
    }

    pub fn read_next(&mut self) -> u8{
        let buffer = self.data[self.curr_addr % self.size];
        self.curr_addr += 1;
        buffer
    }

    pub fn load_bytes(&mut self, addr: usize, words: Vec<u8>){
        if !self.check_in_bounds(addr){
            panic!("Attempted to load words outside of ram!");
        }
        for word in 0..words.len(){
            self.data[addr + word] = words[word];
        }
    }

    pub fn load(&mut self, addr: usize, word: u8){
        if !self.check_in_bounds(addr) {
            panic!("Attempted to load into invalid address!");
        }

        self.data[addr] = word;
    }

    pub fn size(&self) -> usize{
        self.size
    }

    fn check_in_bounds(&self, addr: usize) -> bool{
        addr >= 0 && addr < self.size
    }
}