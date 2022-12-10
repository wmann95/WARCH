use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use WARCH::cpu::CPU;
use WARCH::machine;
use WARCH::harddrive::HardDrive;
use WARCH::machine::Machine;
use WARCH::ram::RAM;
use WARCH::screen::Screen;
use clap::Parser;
/// First computer specs:
/// CPU: Intel 8088
/// Monitor: 720x350 pixel green screen
/// RAM: 16KB

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short = 'i', long = "input")]
    input: Option<String>,

    #[arg(short = 'd', long = "disassemble", required = false)]
    disassemble: bool,

}

fn main() {
    let args = Args::parse();
    /*
    TODO:
    Figure out how a configuration file would look
    For now though, worry about making a rudimentary system.
    */
    // 
    // if std::env::args().len() > 2 {
    //     panic!("Usage: TODO");
    // }
    
    let file: Option<String> = args.input;
    let dasm: bool = args.disassemble;

    let mut machine: Machine = Machine::new();
    
    match dasm{
        true => {
            machine.disassemble(file.as_deref());
        },
        false => {
            machine.boot(file.as_deref());
        }
    }
    
    // let sdl_context = sdl2::init().unwrap();
    // 
    // let mut screen = Screen::new(4, 3, 100, 100, 255, &sdl_context);
    // 
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // 'running: loop{
    //     for event in event_pump.poll_iter(){
    //         match event{
    //             Event::Quit {..} |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
    //                 break 'running
    //             },
    //             _ => {}
    //         }
    //     }
    // 
    //     screen.draw(Some(vec![vec![[0xba; 3]; 100]; 100]));
    // 
    //     std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    // }

}