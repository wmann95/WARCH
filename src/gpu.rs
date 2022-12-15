use std::io::{Read, stdin};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;
use crate::machine::{Machine};
use crate::ram::RAM;

#[derive(Copy, Clone, Debug)]
pub enum GpuOpcode { 
    NOP,
    Clear, // Clear screen display
    MovC, // Move cursor to r[b], r[c]
    Print, // Print char in r[c] to cursor
    Jump, // Jump to r[c] in program
    Run, // Dupe segment r[b] into m[0] and goto r[c]
    MapSeg, // Map segment of r[c] length and place id into r[b]
    UMapSeg, // Unmap segment r[b]
    Load, // load register r[a] with m[r[b]][r[c]]
    Store, // m[r[a]][r[b]] = r[c]
    CMov, // if r[c] != 0, r[a] = r[b]
    Add, // r[a] = r[b] + r[c]
    Mul, // r[a] = r[b] * r[c]
    Div, // r[a] = r[b] / r[c]
    NAND, //r[a] = ~(r[b] & r[c])
    MovI // r[l] = lv
}

pub struct GPU{
    pub video_out: Vec<Vec<[u8; 3]>>,
    clock_speed: u64,
    register_width: usize,
    registers: Vec<u64>,
    program_counter: u64,
    stack: Vec<u64>,
    signalers: Vec<Receiver<u128>>,
    dd_ram: RAM // Display Data RAM
}

pub struct GpuSignal {
    pub signal: *mut GPU
}
unsafe impl Send for GpuSignal {}

pub fn get_opcode(code: u32) -> GpuOpcode {
}

pub fn mask(width: u64) -> u64{
    (1 << width) - 1
}

pub fn get_bits(instruction: u32, width: u32, lsb: u32) -> u32{
    (instruction >> lsb) & (mask(width as u64) as u32)
}

pub fn check_fits(val: u64, bits: u32) -> bool {
    val < 2_u32.pow(bits) as u64
}

impl GPU{
    pub fn new(clock_speed: u64, register_width: usize, register_count: usize, x_res: usize, y_res: usize) -> Self{
        let registers = vec![0u64; register_count];
        let stack: Vec<u64> = Vec::new();
        let video_out = vec![vec![[0_u8; 3]; x_res]; y_res];
        let dd_ram = RAM::new();

        GPU{
            video_out,
            clock_speed,
            register_width,
            registers,
            stack,
            program_counter: 0,
            signalers: Vec::new(),
            dd_ram
        }
    }
    
    pub fn init(&mut self, b1: Receiver<String>, a2: Sender<Option<GpuSignal>>){
        let signal = GpuSignal {
            signal: self as *mut GPU
        };

        match b1.recv(){
            Ok(string) => {
                a2.send(Some(signal)).ok();
            }
            Err(e) => {
                a2.send(None).ok();
            }
        }
    }

    pub fn add_signaler(&mut self, r: Receiver<u128>) {
        self.signalers.push(r);
    }

    pub fn run(&mut self, pc: u64){
        self.program_counter = pc;
        let instruction = self.ddram.get(0, self.program_counter as usize);
        //let instruction = machine.as_mut().unwrap().get_ram().get(0, self.program_counter as usize);

        self.compute(instruction as u32);

        sleep(Duration::from_millis(1000/self.clock_speed));
    }

    pub fn build_instruction(&self, op: GpuOpcode, ra: usize, rb: usize, rc: usize) -> u32{
        if op as u32 > GpuOpcode::LV as u32 ||
            ra >= self.registers.len() ||
            rb >= self.registers.len() ||
            rc >= self.registers.len()
        {
            panic!("Bad instruction parameters!");
        }
        ((op as u32) << 28) | (ra << 6) as u32 | (rb << 3) as u32 | rc as u32
    }

    pub fn instruction(&mut self, op: GpuOpcode, ra: usize, rb: usize, rc: usize){
        let inst = self.build_instruction(op, ra, rb, rc);
        self.compute(inst);
    }

    pub fn build_lv_inst(&self, rl: usize, lv: u32) -> u32{
        if !check_fits(lv as u64, 25){
            panic!("value won't fit into 25 bits!")
        }
        ((GpuOpcode::LV as u32) << 28) | (rl << 25) as u32 | (lv) as u32
    }

    pub fn lv_instruction(&mut self, rl: usize, lv: u32){
        let inst = self.build_lv_inst(rl, lv);
        self.compute(inst);
    }

    pub fn disassemble(&self, instruction: u32) -> String {
        let op = get_bits(instruction, 4, 28);
        let ra: usize = get_bits(instruction, 3, 6) as usize;
        let rb: usize = get_bits(instruction, 3, 3) as usize;
        let rc: usize = get_bits(instruction, 3, 0) as usize;
        let rl: usize = get_bits(instruction, 3, 25) as usize;
        let lval = get_bits(instruction, 25, 0);

        //println!("{:x}", instruction);

        if op == GpuOpcode::LV as u32 {
            format!("{:?} {} {}", get_opcode(op), rl, lval)
        }
        else if op != GpuOpcode::INVALID as u32{
            format!("{:?} {} {} {}", get_opcode(op), ra, rb, rc)
        }
        else{
            format!("Junk or invalid operation.")
        }
    }

    pub fn compute(&mut self, instruction: u32){
        let op = get_bits(instruction, 4, 28);
        let ra: usize = get_bits(instruction, 3, 6) as usize;
        let rb: usize = get_bits(instruction, 3, 3) as usize;
        let rc: usize = get_bits(instruction, 3, 0) as usize;
        let rl: usize = get_bits(instruction, 3, 25) as usize;
        let lval = get_bits(instruction, 25, 0);

        match op{
            opcode =>{
                if opcode == GpuOpcode::CMov as u32{
                    self.cmov(ra, rb, rc);
                }
                else if opcode == GpuOpcode::Load as u32{
                    self.load(ra, rb, rc);
                }
                else if opcode == GpuOpcode::Store as u32{
                    self.store(ra, rb, rc);
                }
                else if opcode == GpuOpcode::Add as u32{
                    self.add(ra, rb, rc);
                }
                else if opcode == GpuOpcode::Mul as u32{
                    self.mul(ra, rb, rc);
                }
                else if opcode == GpuOpcode::Div as u32{
                    self.div(ra, rb, rc);
                }
                else if opcode == GpuOpcode::NAND as u32{
                    self.nand(ra, rb, rc);
                }
                else if opcode == GpuOpcode::MapSeg as u32{
                    self.map_seg(rb, rc);
                }
                else if opcode == GpuOpcode::UnmapSeg as u32{
                    self.unmap_seg(rc);
                }
                else if opcode == GpuOpcode::LP as u32{
                    self.load_program(rb, rc);
                }
                else if opcode == GpuOpcode::LV as u32{
                    self.load_val(rl, lval as u64);
                }
                else{
                    panic!("Bad Opcode! No operation found!");
                }
            }
        }
    }

    fn cmov(&mut self, ra: usize, rb: usize, rc: usize){
        if self.registers[rc] != 0{
            let b = self.registers[rb];
            self.registers[ra] = b;
        }
    }

    fn load(&mut self, ra: usize, rb: usize, rc: usize){
        let seg_id = self.registers[rb];
        let index = self.registers[rc];
        self.registers[ra] = self.ddram.get(seg_id as usize, index as usize) as u64;
    }

    fn store(&mut self, ra: usize, rb: usize, rc: usize){
        let seg_id = self.registers[ra] as usize;
        let index = self.registers[rb] as usize;
        let value = self.registers[rc];
        self.ddram.set(seg_id, index, value);
    }

    fn add(&mut self, ra: usize, rb: usize, rc: usize){
        let vb = self.registers[rb];
        let vc = self.registers[rc];
        self.registers[ra] = ((vb as u128 + vc as u128) % (1_u128 << 64)) as u64;
    }

    fn mul(&mut self, ra: usize, rb: usize, rc: usize){
        let vb = self.registers[rb];
        let vc = self.registers[rc];
        self.registers[ra] = ((vb as u128 * vc as u128) % (1_u128 << 64)) as u64;
    }

    fn div(&mut self, ra: usize, rb: usize, rc: usize){
        let vb = self.registers[rb];
        let vc = self.registers[rc];
        if vc == 0 {
            panic!("Division by 0!");
        }
        self.registers[ra] = ((vb as u128 / vc as u128) % (1_u128 << 64)) as u64;
    }

    fn nand(&mut self, ra: usize, rb: usize, rc: usize){
        let vb = self.registers[rb];
        let vc = self.registers[rc];
        self.registers[ra] = !(vb & vc);
    }
    
    fn map_seg(&mut self, rb: usize, rc: usize){
        let word_count = self.registers[rc];
        let seg_id = self.ddram.request_segment(word_count as usize) as u64;
        self.registers[rb] = seg_id;
    }

    fn unmap_seg(&mut self, rc: usize){
        let seg_id = self.registers[rc];
        self.ddram.release_segment(seg_id as usize);
    }
    
    fn load_program(&mut self, rb: usize, rc :usize){
        let vb = self.registers[rb];
        let vc = self.registers[rc];

        if vb == 0{
            self.program_counter = vc - 1;
            return
        }
        
        self.ddram.duplicate_segment(vb as usize, 0);
        self.program_counter = vc - 1;
    }

    fn load_val(&mut self, rl: usize, lv: u64){
        self.registers[rl] = lv;
    }

    pub fn print_state(&self){
        println!("Registers:");

        for i in 0..self.registers.len(){
            println!("R[{}]: {}", i, self.registers[i] as i32)
        }
        println!("PC: {}", self.program_counter)
    }

    pub fn interrupt(&mut self, _signal: u128){

    }
}
