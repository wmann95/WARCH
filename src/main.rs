use std::sync::mpsc::{Receiver, RecvError, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use warch::machine::Machine;
use warch::screen::Screen;
use clap::Parser;
use warch::gpu::{GpuSignal};
use warch::ram::RAM;

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
        false => unsafe {
            
            // Allows for the cpu to be interrupted via the event_pump
            let (sx, sdl_interrupter) = std::sync::mpsc::channel();

            let (a1, b1) = std::sync::mpsc::channel();
            let (a2, b2): (Sender<Option<GpuSignal>>, Receiver<Option<GpuSignal>>) = std::sync::mpsc::channel();
            
            let screen_thread = thread::spawn(move|| {
                let sdl_context = sdl2::init().unwrap();

                let mut signal: Option<GpuSignal> = None;
                
                a1.send(String::from("ready for connection")).ok();
                
                for received in b2{
                    signal = received;
                }
                
                let mut screen = Screen::new(8, 6, 100, 100, 255, &sdl_context);

                let mut event_pump = sdl_context.event_pump().unwrap();
                
                let mut pc: u64 = 0;
                'running: loop{
                    for event in event_pump.poll_iter(){
                        match event{
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                                sx.send((7 << 28) as u128).ok();
                                break 'running
                            },
                            _ => {}
                        }
                    }
                    
                    // TODO: Have to create segment[0] for GPU and give it some instructions to run.
                    (*(signal.as_mut().unwrap()).signal).run(pc);
                    
                    screen.draw(&signal);

                    pc += 1;
                    thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
                }
            });
            
            machine.add_signaler(sdl_interrupter);
            
            machine.boot(file.as_deref(), b1, a2);
            
            screen_thread.join().unwrap();
        }
    }
    
    

}