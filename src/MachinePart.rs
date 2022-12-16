use crate::cpu::CPU;
use crate::gpu::GPU;
use crate::harddrive::HardDrive;
use crate::ram::RAM;
use crate::screen::Screen;

pub enum MachinePart{
    CPU(CPU),
    RAM(RAM),
    Storage(HardDrive),
    GPU(GPU)
}