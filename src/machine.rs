use crate::cpu::{CPU, Opcode};
use crate::harddrive::HardDrive;
use crate::ram::RAM;
use crate::screen::Screen;

pub struct Machine{
    cpu: CPU,
    ram: RAM,
    storage: Vec<HardDrive>,
    screen: Option<Screen>
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
            screen: None,
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
                let word = ((bytes[0] as u32) << 24) |
                    ((bytes[1] as u32) << 16) |
                    ((bytes[2] as u32) << 8) |
                    bytes[3] as u32;
                //println!("{:b}", word);
                let dasm = self.cpu.disassemble(word);
                println!("{dasm}");
                i += 1;
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

    pub fn boot(&mut self, input: Option<&str>) {
        
        self.storage.push(HardDrive::from_file(input).unwrap());
        let t = self as *mut Machine;
        
        unsafe {
            // set up m[0]
            let size = self.storage[0].get_byte_length().clone();
            let mut prog = self.storage[0].load_segment(0, size);
            
            
            self.cpu.lv_instruction(t, 0, (prog.len() / 4) as u32);
            self.cpu.instruction(t, Opcode::MapSeg, 0, 0, 0);
            
            let mut i = 0;
            for bytes in prog.chunks_exact_mut(4){
                let word = ((bytes[0] as u32) << 24) |
                    ((bytes[1] as u32) << 16) |
                    ((bytes[2] as u32) << 8) |
                    bytes[3] as u32;
                //println!("{:b}", word);
                (*t).ram.set(0, i, word as u64);
                i += 1;
            };

            self.cpu.instruction(t, Opcode::LP, 0, 0, 0);
            self.cpu.run(t);
            // 
            // self.cpu.lv_instruction(t, 0, 1);
            // self.cpu.instruction(t, Opcode::MapSeg, 0, 0, 0);
            // self.cpu.print_state();
        }
    }
}