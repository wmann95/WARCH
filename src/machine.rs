use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use sdl2::libc::system;
use sdl2::mouse::SystemCursor::No;
use crate::cpu::{CPU, CPU_Opcode};
use crate::gpu::{GPU};
use crate::harddrive::HardDrive;
use crate::MachinePart::MachinePart;
use crate::ram::RAM;

pub struct Machine{
    cpu: Option<CPU>,
    gpu: Option<GPU>,
    ram: Option<RAM>,
    storage: Vec<HardDrive>,
    parts: Option<MachineWrapper>
}

pub struct MachineWrapper{
    pub cpu: *mut CPU,
    pub gpu: *mut GPU,
    pub ram: *mut RAM,
    pub storage: *mut Vec<HardDrive>
}

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
        for bytes in prog.chunks_exact_mut(4){
            let word = Self::get_instruction(bytes);
            println!("{:x}: {:x}", i, word);
            let dasm = self.cpu.as_mut().unwrap().disassemble(word);
            println!("{dasm}");
            i += 4;
        };
    }
    
    pub fn add_signaler(&mut self, r: Receiver<u128>) {
        self.cpu.as_mut().unwrap().add_signaler(r);
    }
    
    fn get_instruction(bytes: &mut [u8]) -> u32{
        ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | bytes[3] as u32
    }
    
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
    
    pub fn boot(&mut self, input: Option<&str>) {
        //self.gpu.unwrap().init(b1, a2);
        
        self.storage.push(HardDrive::from_file(input).unwrap());
        let m = self.parts.as_ref().unwrap();
        
        // GPU
        
        // CPU
        
        let t = self.parts.as_ref().unwrap();
        
        unsafe {
            // set up the instructions that will go into m[0]
            let size = self.storage[0].get_byte_length().clone();
            let mut prog = self.storage[0].load_segment(0, size);
        
            // make the original segment m[0]
            (*m.cpu).lv_instruction(m, 0, (prog.len() / 4) as u32);
            (*m.cpu).instruction(m, CPU_Opcode::MapSeg, 0, 0, 0);
        
            // load the instructions into m[0]
            let mut i = 0;
            for bytes in prog.chunks_exact_mut(4) {
                let word = Self::get_instruction(bytes);
                (*t.ram).set(0, i, word as u64);
                i += 1;
            };

            (*m.cpu).run(m);
        }
    }
    
    pub fn halt(&mut self){
        self.cpu.as_mut().unwrap().halt();
    }
}