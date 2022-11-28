use std::env;
use log::{debug, error};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event;
use winit::event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event::DeviceEvent::Key;
use winit::event::WindowEvent::KeyboardInput;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use WARCH::cpu::CPU;
use WARCH::machine;
use WARCH::harddrive::HardDrive;
use WARCH::ram::RAM;

/// First computer specs:
/// CPU: Intel 8088
/// Monitor: 720x350 pixel green screen
/// RAM: 16KB

fn main() -> Result<(), Error>{

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(400, 300);
        let scaled_size = LogicalSize::new(1200, 900);
        WindowBuilder::new()
            .with_title("WARCH")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(400, 300, surface_texture)?
    };

    let mut time = std::time::Instant::now();
    let mut frame_timer = time.clone();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        //control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing...");
                control_flow.set_exit();
            },
            Event::DeviceEvent {
                event: DeviceEvent::Key(key),
                ..
            } => {
                match key.state{
                    ElementState::Pressed => {
                        println!("{} pressed", key.scancode);
                    }
                    ElementState::Released => {
                        println!("{} released", key.scancode);
                    }
                }
            },
            Event::MainEventsCleared => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                //window.request_redraw();
            },
            Event::RedrawRequested(window) => {

                let raster = pixels.get_frame_mut().chunks_exact_mut(4);

                for x in raster{

                    let color= [0, 0, 0, 0];
                    x.copy_from_slice(&color);
                }

                if pixels.render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }

        if frame_timer.elapsed().as_millis() >= (60.0/1000.0) as u128{

            window.request_redraw();
        }

        if input.update(&event){
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit(){
                *control_flow = ControlFlow::Exit;
                return;
            }

        }
    });

    Ok(())

    // let input = env::args().nth(1);
    //
    // let mut harddrive: HardDrive = match HardDrive::from_file(input.as_deref()){
    //     Ok(h) => { h }
    //     Err(_) => {
    //         HardDrive::new(Some("maindisk.wmiso"), 65536)
    //     }
    // };
    //
    // let mut ram: RAM = RAM::new(16384);
    // let mut cpu: CPU = CPU::new(51, 8, 4);
    //
    // machine::boot(&mut harddrive, &mut ram, &mut cpu);
}