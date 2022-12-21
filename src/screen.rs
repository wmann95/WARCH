use crossbeam::channel::Receiver;
use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas};
use sdl2::Sdl;
use crate::machine::VideoOutWrapper;
// use winit::dpi::LogicalSize;
// use winit::event_loop::EventLoop;
// use winit::window::{Window, WindowBuilder};

/// A struct that represents the screen of the emulated computer.
///
/// Fields:
/// * `pixel_width`: The width of the individual pixels of the monitor.
/// * `pixel_height`: The height of the individual pixels of the monitor.
/// * `width`: The pixel count width of the monitor.
/// * `height`:  The pixel count height of the monitor.
/// * `color_width`
/// * `pixels`: Array representing the pixels to be rendered.
pub struct Screen {
    width: u32,
    height: u32,
    color_width: u64,
    clear_color: Color,
    pixels: Vec<Vec<(Rect, [u8; 3])>>,
    canvas: WindowCanvas,
    rng: ThreadRng,
    receiver: Receiver<VideoOutWrapper>
}


impl Screen{
    pub fn new(pixel_width: u32, pixel_height: u32, x_size: u32, y_size: u32, color_width: u64, sdl_context: &Sdl, receiver: Receiver<VideoOutWrapper>) -> Self{

        let video_subsystem = sdl_context.video().unwrap();

        let width = pixel_width * x_size;
        let height = pixel_height * y_size;

        let window = video_subsystem.window("rust-sdl2 demo", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let clear_color = Color::RGB(0,0,0);

        let rng = rand::thread_rng();

        let mut pixels = Vec::new();

        for y in 0..y_size{
            let mut buffer = Vec::new();
            for x in 0..x_size{
                let rect: Rect = Rect::new(
                    (x * pixel_width) as i32,
                    (y * pixel_height) as i32,
                    pixel_width,
                    pixel_height
                );

                buffer.push((rect, [0,0,0]));
            }
            pixels.push(buffer);
        }

        println!("{} {}", pixels.len(), pixels[0].len());

        Screen{
            width: x_size,
            height: y_size,
            color_width,
            pixels,
            clear_color,
            canvas,
            rng,
            receiver
        }
    }

    pub unsafe fn draw(&mut self) {
        self.canvas.set_draw_color(self.clear_color);
        self.canvas.clear();
        
        if self.receiver.len() > 0 {
            
            //println!("Debug1");
            
            let pixels = self.receiver.iter().nth(0).unwrap().data;

            //println!("Debug2");
            //println!("{}", pixels.len());
            
            if (*pixels).len() == self.pixels.len() * self.pixels[0].len() * 3 {

                //println!("{}", (*pixels).len());
                for mut pixel in (0..(*pixels).len()).step_by(3){
                    //println!("{}", pixel);
                    let x = (pixel / 3) % (self.pixels[0].len());
                    let y = (pixel / 3) / (self.pixels[0].len());

                    //println!("{pixel}");
                    self.pixels[y][x].1 = [(*pixels)[pixel] as u8, (*pixels)[pixel + 1] as u8, (*pixels)[pixel + 2] as u8];
                    
                }
            }
            else{
                for y in 0..self.height as usize{
                    for x in 0..self.width as usize{
                        let red = self.rng.gen::<u8>();
                        let green = self.rng.gen::<u8>();
                        let blue = self.rng.gen::<u8>();
                
                        self.pixels[y][x].1 = [red, green, blue];
                    }
                }
            }
            
        }

        for y in 0..self.height as usize{
            for x in 0..self.width as usize{
                //println!("{} {}", x, y);
                
                self.canvas.set_draw_color(
                    Color::RGB(
                        self.pixels[y][x].1[0],
                        self.pixels[y][x].1[1],
                        self.pixels[y][x].1[2],
                    )
                );
                self.canvas.fill_rect(
                    self.pixels[y][x].0
                ).unwrap();
            }
        }

        // 
        // match signal{
        //     Some(s) => unsafe {
        //         for y in 0..self.height as usize{
        //             for x in 0..self.width as usize{
        //                 //println!("{} {}", x, y);
        //                 
        //                 self.canvas.set_draw_color(
        //                     Color::RGB(
        //                         (*s.signal).video_out[y][x][0],
        //                         (*s.signal).video_out[y][x][1],
        //                         (*s.signal).video_out[y][x][2],
        //                     )
        //                 );
        //                 self.canvas.fill_rect(
        //                     self.pixels[y][x]
        //                 ).unwrap();
        //             }
        //         }
        //     }
        //     None => { 
        //         
        //     }
        // }
        self.canvas.present();
    }
}