use crate::cpu::CPU;
use crate::harddrive::HardDrive;
use crate::ram::RAM;

pub fn boot(harddrive: &mut HardDrive, ram: &mut RAM, cpu: &mut CPU){

    // Think of this as the motherboard. It's main purpose in this context
    // is to run the simple program that will load the OS into memory
    // and give control to the cpu.

    //


    let mbr = harddrive.load_segment(0, 64);
    for i in 0..64 {
        println!("{:b}", mbr[i]);
    }



}

pub fn send_output_to_screen(){

}