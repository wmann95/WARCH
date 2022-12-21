
use std::thread;
use crossbeam::channel::Sender;
use sdl2::libc::system;
use sdl2::mouse::SystemCursor::No;
use crate::cpu::{CPU, CPU_Opcode, get_bits};
use crate::gpu::{GPU};
use crate::harddrive::HardDrive;
use crate::MachinePart::MachinePart;
use crate::ram::RAM;

pub struct Machine{
    cpu: Option<CPU>,
    gpu: Option<GPU>,
    ram: Option<RAM>,
    storage: Vec<HardDrive>,
    parts: Option<MachineWrapper>,
}

pub struct MachineWrapper{
    pub cpu: *mut CPU,
    pub gpu: *mut GPU,
    pub ram: *mut RAM,
    pub storage: *mut Vec<HardDrive>
}

pub struct VideoOutWrapper{
    pub data: Vec<u64>
}

unsafe impl Send for VideoOutWrapper{}

impl Machine{

    pub fn new() -> Self{
        Self{
            cpu: None,
            gpu: None,
            ram: None,
            storage: Vec::new(),
            parts: None
        }
    }

    pub fn insert(&mut self, part: MachinePart) {
        match part{
            MachinePart::RAM(ram) => {
                self.ram = Some(ram);
            }
            MachinePart::CPU(cpu) => {
                self.cpu = Some(cpu);
            }
            MachinePart::GPU(gpu) => {
                self.gpu = Some(gpu);
            }
            MachinePart::Storage(drive) => {
                self.storage.push(drive);
            }
        }
    }

    pub fn disassemble(&mut self, input: Option<&str>){
        
        self.storage.push(HardDrive::from_file(input).unwrap());
        
        // set up m[0]
        let size = self.storage[0].get_byte_length().clone();
        let mut prog = self.storage[0].load_segment(0, size);
    
        let mut i = 0;
        for bytes in prog.chunks_exact_mut(8){
            let word = Self::get_32bit_instruction(bytes);
            println!("{:x}: {:x}", i, word);
            let dasm = self.cpu.as_mut().unwrap().disassemble(word);
            println!("{dasm}");
            i += 4;
        };
    }

    fn get_32bit_instruction(bytes: &mut [u8]) -> u32{
        ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | bytes[3] as u32
    }

    fn get_64bit_instruction(bytes: &mut [u8]) -> u64{
        ((bytes[0] as u64) << 56) | ((bytes[1] as u64) << 48) | ((bytes[2] as u64) << 40) | ((bytes[3] as u64) << 32) |
            ((bytes[4] as u64) << 24) | ((bytes[5] as u64) << 16) | ((bytes[6] as u64) << 8) | bytes[7] as u64
    }
    
    // fn convert_from_rum(bytes: &mut [u8]) -> u64{
    //     let old = Self::get_32bit_instruction(bytes) as u64;
    //     let op = get_bits(old, 4, 28);
    //     let ra = get_bits(old, 3, 6);
    //     let rb = get_bits(old, 3, 3);
    //     let rc= get_bits(old, 3, 0);
    //     let rl= get_bits(old, 3, 25);
    //     let lval = get_bits(old, 25, 0);
    //     
    //     let mut new = 0;
    //     
    //     if op as u64 != CPU_Opcode::LV as u64{
    //         new = op << 58 | ra << 8 | rb << 4 | rc;
    //     } 
    //     else{
    //         new = op << 58 | rl << 54 | lval;
    //     }
    //     println!("{:b}", old);
    //     println!("{:b}", new);
    //     
    //     new
    // }

    pub fn get_ram(&mut self) -> &mut RAM{
        self.ram.as_mut().unwrap()
    }
    pub fn get_storage(&mut self) -> &mut Vec<HardDrive>{
        &mut self.storage
    }
    
    pub fn power_on_self_test(&mut self) {
        let mut flag = false;
        if self.cpu.is_none() {
            eprintln!("NO CPU FOUND");
            flag = true;
        }
        if self.gpu.is_none(){
            eprintln!("NO GPU FOUND");
            flag = true;
        }
        if self.ram.is_none() {
            eprintln!("NO RAM FOUND");
            flag = true;
        }
        if self.storage.len() == 0{
            eprintln!("NO STORAGE FOUND");
            flag = true;
        }

        if flag{
            std::process::exit(0);
        }
        
        let t = MachineWrapper{
            cpu: self.cpu.as_mut().unwrap() as *mut CPU,
            gpu: self.gpu.as_mut().unwrap() as *mut GPU,
            ram: self.ram.as_mut().unwrap() as *mut RAM,
            storage: self.storage.as_mut() as *mut Vec<HardDrive>
        };
        
        self.parts = Some(t);
        
        println!("BEEP!")
    }
    
    pub fn add_sender(&mut self, sender: Sender<VideoOutWrapper>){
        self.cpu.as_mut().unwrap().add_signaler(sender);
    }
    
    pub fn boot(&mut self, input: Option<&str>) {
        //self.gpu.unwrap().init(b1, a2);
        
        self.storage.push(HardDrive::from_file(input).unwrap());
        //let t = self.parts.as_ref().unwrap();
        
        // GPU
        let m = self.parts.as_ref().unwrap();
        
        unsafe {
            // set up the instructions that will go into m[0]
            let size = self.storage[0].get_byte_length().clone();
            let mut prog = self.storage[0].load_segment(0, size);

            // make the original segment m[0] for program
            (*m.cpu).lv_instruction(m, 0, (prog.len() / 4) as u32);
            (*m.cpu).instruction(m, CPU_Opcode::MapSeg, 0, 0, 0);
            
            

            // // make the original segment m[1] for video out
            (*m.cpu).lv_instruction(m, 0, (100 * 100 * 3) as u32);
            (*m.cpu).instruction(m, CPU_Opcode::MapSeg, 0, 0, 0);

            //println!("Test1");
            // load the instructions into m[0]
            let mut i = 0;
            for bytes in prog.chunks_exact_mut(4) {
                let word = Self::get_32bit_instruction(bytes);
                (*m.ram).set(0, i, word as u64);
                i += 1;
            };

            (*m.cpu).run(m);
        }
    }
    
    pub fn halt(&mut self){
        self.cpu.as_mut().unwrap().halt();
    }
}