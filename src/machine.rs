use crate::cpu::{CPU, Opcode};
use crate::harddrive::HardDrive;
use crate::ram::RAM;
use crate::screen::Screen;

pub struct Machine{
    cpu: CPU,
    ram: RAM,
    storage: Vec<HardDrive>,
    video_out: Vec<Vec<u64>>
}

impl Machine{

    pub fn new() -> Self{
        let ram: RAM = RAM::new();
        let storage: Vec<HardDrive> = Vec::new();
        let cpu: CPU = CPU::new(500000000, 32, 8);
        Self{
            cpu,
            ram,
            storage,
            video_out: vec![vec![0_u64; (800_usize * 600_usize)]]
        }
    }
    
    pub fn disassemble(&mut self, input: Option<&str>){
        
        self.storage.push(HardDrive::from_file(input).unwrap());
        let t = self as *mut Machine;
        
        unsafe {
            // set up m[0]
            let size = self.storage[0].get_byte_length().clone();
            let mut prog = self.storage[0].load_segment(0, size);

            let mut i = 0;
            for bytes in prog.chunks_exact_mut(4){
                let word = Self::get_instruction(bytes);
                println!("{:x}: {:x}", i, word);
                let dasm = self.cpu.disassemble(word);
                println!("{dasm}");
                i += 4;
            };
        }
    }
    
    fn get_instruction(bytes: &mut [u8]) -> u32{
        ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | bytes[3] as u32
    }
    
    pub fn get_ram(&mut self) -> &mut RAM{
        &mut self.ram
    }
    pub fn get_storage(&mut self) -> &mut Vec<HardDrive>{
        &mut self.storage
    }
    
    pub fn update_video_signal(&mut self, x: usize, y: usize, c: u64){
        self.video_out[y][x] = c;
    }

    pub fn send_char(&mut self, x: usize, y: usize, c: u8){
        
    }

    pub fn boot(&mut self, input: Option<&str>) {
        
        self.storage.push(HardDrive::from_file(input).unwrap());
        let t = self as *mut Machine;
        
        unsafe {
            // set up the instructions that will go into m[0]
            let size = self.storage[0].get_byte_length().clone();
            let mut prog = self.storage[0].load_segment(0, size);
            
            // make the original segment m[0]
            self.cpu.lv_instruction(t, 0, (prog.len() / 4) as u32);
            self.cpu.instruction(t, Opcode::MapSeg, 0, 0, 0);
            
            // load the instructions into m[0]
            let mut i = 0;
            for bytes in prog.chunks_exact_mut(4){
                let word = Self::get_instruction(bytes);
                (*t).ram.set(0, i, word as u64);
                i += 1;
            };
            
            self.cpu.instruction(t, Opcode::LP, 0, 0, 0);
            self.cpu.run(t);
        }
    }
}