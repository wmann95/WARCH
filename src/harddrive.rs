use std::fs::File;
use std::io::{Read, Write};

pub struct HardDrive{
    image: Vec<u8>,
    size: usize
}

impl HardDrive{

    pub fn from_file(name: Option<&str>) -> Result<HardDrive, &str>{
        let mut file = File::open(
            match name{
                None => { "maindisk.wmiso" },
                Some(filename) => {
                    filename
                }
            }
        ).unwrap();
        
        
        
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let size = buffer.len();
        
        //println!("{}", size);
        Ok(
            HardDrive{
                image: buffer,
                size
            }
        )
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
                panic!("{}", e);
            }
        };
        let buffer = vec![0u8; size];

        file.write(&*buffer).expect("Could not write bytes!");

        HardDrive{
            image: buffer,
            size: 0
        }
    }
    
    pub fn get_byte_length(&self) -> usize{
        self.size
    }

    pub fn load_segment(&mut self, addr: usize, length: usize) -> Vec<u8>{
        (&self.image)[addr..(addr+length)].to_vec()
    }
}