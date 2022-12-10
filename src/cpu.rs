use std::collections::HashMap;
use std::io::{Read, stdin};
use std::thread::sleep;
use std::time::Duration;
use crate::harddrive::HardDrive;
use crate::machine::Machine;
use crate::ram::RAM;

pub struct CPU{
    clock_speed: u64,
    register_width: usize,
    registers: Vec<u64>,
    program_counter: u64,
    stack: Vec<u64>,
    halt_flag: bool
}

#[derive(Copy, Clone, Debug)]
pub enum Opcode { CMov, Load, Store, Add, Mul, Div, NAND, HALT, MapSeg, UnmapSeg, Out, In, LP, LV }
pub fn get_opcode(code: u32) -> Opcode{
    match code{
        0 => { Opcode::CMov },
        1 => { Opcode::Load },
        2 => { Opcode::Store },
        3 => { Opcode::Add },
        4 => { Opcode::Mul },
        5 => { Opcode::Div },
        6 => { Opcode::NAND },
        7 => { Opcode::HALT },
        8 => { Opcode::MapSeg },
        9 => { Opcode::UnmapSeg },
        10 => { Opcode::Out },
        11=> { Opcode::In },
        12 => { Opcode::LP },
        13 => { Opcode::LV },
        n => {
            panic!("Unknown instruction: {}", n);
        }
    }
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

impl CPU{
    pub fn new(clock_speed: u64, register_width: usize, register_count: usize) -> Self{
        let registers = vec![0u64; register_count];
        let stack: Vec<u64> = Vec::new();

        CPU{
            clock_speed,
            register_width,
            registers,
            stack,
            program_counter: 0,
            halt_flag: false
        }
    }
    
    pub unsafe fn run(&mut self, machine: *mut Machine){
        self.program_counter = 0;
        'run: loop{
            let instruction = machine.as_mut().unwrap().get_ram().get(0, self.program_counter as usize);

            self.compute(machine, instruction as u32);
            // println!("{}", self.disassemble(instruction as u32));
            //self.print_state();
            // println!();
            
            if self.halt_flag{
                break 'run
            }
            
            self.program_counter += 1;

            //sleep(Duration::from_millis(1000/self.clock_speed));
        }
    }

    pub fn build_instruction(&self, op: Opcode, ra: usize, rb: usize, rc: usize) -> u32{
        if op as u32 > Opcode::LV as u32 ||
            ra >= self.registers.len() ||
            rb >= self.registers.len() ||
            rc >= self.registers.len()
        {
            panic!("Bad instruction parameters!");
        }
        ((op as u32) << 28) | (ra << 6) as u32 | (rb << 3) as u32 | rc as u32
    }
    
    pub unsafe fn instruction(&mut self, machine: *mut Machine, op: Opcode, ra: usize, rb: usize, rc: usize){
        let inst = self.build_instruction(op, ra, rb, rc);
        self.compute(machine, inst);
    }
    
    pub fn build_lv_inst(&self, rl: usize, lv: u32) -> u32{
        if !check_fits(lv as u64, 25){
            panic!("value won't fit into 25 bits!")
        }
        ((Opcode::LV as u32) << 28) | (rl << 25) as u32 | (lv) as u32
    }

    pub unsafe fn lv_instruction(&mut self, machine: *mut Machine, rl: usize, lv: u32){
        let inst = self.build_lv_inst(rl, lv);
        self.compute(machine, inst);
    }
    
    pub fn disassemble(&self, instruction: u32) -> String {
        let op = get_bits(instruction, 4, 28);
        let ra: usize = get_bits(instruction, 3, 6) as usize;
        let rb: usize = get_bits(instruction, 3, 3) as usize;
        let rc: usize = get_bits(instruction, 3, 0) as usize;
        let rl: usize = get_bits(instruction, 3, 25) as usize;
        let lval = get_bits(instruction, 25, 0);
        
        //println!("{:x}", instruction);
        
        if op == Opcode::LV as u32 {
            format!("{:?} {} {}", get_opcode(op), rl, lval)
        }
        else{
            format!("{:?} {} {} {}", get_opcode(op), ra, rb, rc)
        }
    }

    pub unsafe fn compute(&mut self, machine: *mut Machine, instruction: u32){
        let op = get_bits(instruction, 4, 28);
        let ra: usize = get_bits(instruction, 3, 6) as usize;
        let rb: usize = get_bits(instruction, 3, 3) as usize;
        let rc: usize = get_bits(instruction, 3, 0) as usize;
        let rl: usize = get_bits(instruction, 3, 25) as usize;
        let lval = get_bits(instruction, 25, 0);

        match op{
            opcode =>{
                if opcode == Opcode::CMov as u32{
                    self.cmov(ra, rb, rc);
                }
                else if opcode == Opcode::Load as u32{
                    self.load(machine, ra, rb, rc);
                }
                else if opcode == Opcode::Store as u32{
                    self.store(machine, ra, rb, rc);
                }
                else if opcode == Opcode::Add as u32{
                    self.add(ra, rb, rc);
                }
                else if opcode == Opcode::Mul as u32{
                    self.mul(ra, rb, rc);
                }
                else if opcode == Opcode::Div as u32{
                    self.div(ra, rb, rc);
                }
                else if opcode == Opcode::NAND as u32{
                    self.nand(ra, rb, rc);
                }
                else if opcode == Opcode::HALT as u32{
                    self.halt();
                }
                else if opcode == Opcode::MapSeg as u32{
                    self.map_seg(machine, rb, rc);
                }
                else if opcode == Opcode::UnmapSeg as u32{
                    self.unmap_seg(machine, rc);
                }
                else if opcode == Opcode::Out as u32{
                    self.out(rc);
                }
                else if opcode == Opcode::In as u32{
                    self.await_in(rc);
                }
                else if opcode == Opcode::LP as u32{
                    self.load_program(machine, rb, rc);
                }
                else if opcode == Opcode::LV as u32{
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
    
    unsafe fn load(&mut self, machine: *mut Machine, ra: usize, rb: usize, rc: usize){
        let seg_id = self.registers[rb];
        let index = self.registers[rc];
        self.registers[ra] = machine.as_mut().unwrap().get_ram().get(seg_id as usize, index as usize) as u64;
    }
    
    unsafe fn store(&self, machine: *mut Machine, ra: usize, rb: usize, rc: usize){
        let seg_id = self.registers[ra] as usize;
        let index = self.registers[rb] as usize;
        let value = self.registers[rc];
        machine.as_mut().unwrap().get_ram().set(seg_id, index, value);
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
    
    fn halt(&mut self){
        self.halt_flag = true;
    }
    
    unsafe fn map_seg(&mut self, machine: *mut Machine, rb: usize, rc: usize){
        let word_count = self.registers[rc];
        let seg_id = machine.as_mut().unwrap().get_ram().request_segment(word_count as usize) as u64;
        self.registers[rb] = seg_id;
    }
    
    unsafe fn unmap_seg(&mut self, machine: *mut Machine, rc: usize){
        let seg_id = self.registers[rc];
        machine.as_mut().unwrap().get_ram().release_segment(seg_id as usize);
    }
    
    fn out(&self, rc: usize){
        if (self.registers[rc] as u32) > 255 {
            panic!("Value in rc is greater than 255!");
        }

        print!("{}", std::char::from_u32(self.registers[rc] as u32).unwrap());
    }
    
    fn await_in(&mut self, rc: usize){
        let val = stdin().bytes().next();
        
        match val{
            None => {
                self.registers[rc] = (((1 as u128) << 64) - 1) as u64;
            }
            Some(value) => {
                self.registers[rc] = value.unwrap() as u64;
            }
        }
    }
    
    unsafe fn load_program(&mut self, machine: *mut Machine, rb: usize, rc :usize){
        let vb = self.registers[rb];
        let vc = self.registers[rc];
        
        if vb == 0{
            self.program_counter = vc - 1;
            return
        }
        
        machine.as_mut().unwrap().get_ram().duplicate_segment(vb as usize, 0);
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
}

#[cfg(test)]
mod tests{
    use crate::cpu::{get_bits, mask};

    #[test]
    fn mask_test(){
        assert_eq!(mask(5), 0b11111);
        assert_eq!(mask(10), 0b1111111111);
        assert_eq!(mask(20), 0b11111111111111111111);
        assert_eq!(mask(32), 0b11111111111111111111111111111111);
    }

    #[test]
    fn get_bits_test(){
        assert_eq!(get_bits(0b0,5,0), 0);
        assert_eq!(get_bits(0b10010,5,0), 0b10010);
        assert_eq!(get_bits(0b1001000,5,2), 0b10010);
        assert_eq!(get_bits(0b01010000000000000000000000000000,4,28), 0b0101);
    }
    
}

