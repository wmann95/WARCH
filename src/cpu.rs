use std::collections::HashMap;

pub struct CPU{
    clock_speed: u64,
    register_width: usize,
    registers: Vec<u64>,
    stack: Vec<u64>
}

impl CPU{
    pub fn new(clock_speed: u64, register_width: usize, register_count: usize) -> Self{
        let registers = vec![0u64; register_count];
        let stack: Vec<u64> = Vec::new();

        CPU{
            clock_speed,
            register_width,
            registers,
            stack
        }
    }
}

