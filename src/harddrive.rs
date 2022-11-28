use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct HardDrive{
    image: File
}

impl HardDrive{

    pub fn from_file(name: Option<&str>) -> Result<HardDrive, &str>{
        let file = File::open(
            match name{
                None => { "maindisk.wmiso" },
                Some(filename) => {
                    filename
                }
            }
        );

        match file {
            Err(e) => {
                Err("Could not open file!")
            }
            Ok(file) => {
                Ok(
                HardDrive{
                    image: file
                }
            )}
        }
    }

    pub fn new(name: Option<&str>, size: usize) -> HardDrive{
        let mut file = match File::create(
            match name{
                None => { "maindisk.wmiso" },
                Some(filename) => {
                    filename
                }
            }
        ){
            Ok(f) => {
                f
            }
            Err(e) => {
                panic!("Could not create file!");
            }
        };
        let buffer = vec![0u8; size];

        file.write(&*buffer).expect("Could not write bytes!");

        HardDrive{
            image: file
        }
    }

    pub fn load_segment(&mut self, addr: usize, length: usize) -> Vec<u8>{
        let mut buffer = vec![0u8; length];
        let a = self.image.read_exact(&mut buffer);
        buffer.to_vec()
    }
}