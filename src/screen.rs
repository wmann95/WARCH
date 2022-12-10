use std::slice::ChunksExactMut;
use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::Sdl;
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
    pixel_width: u32,
    pixel_height: u32,
    width: u32,
    height: u32,
    color_width: u64,
    clear_color: Color,
    pixels: Vec<Vec<Rect>>,
    canvas: WindowCanvas,
    rng: ThreadRng,
}


impl Screen{
    pub fn new(pixel_width: u32, pixel_height: u32, x_size: u32, y_size: u32, color_width: u64, sdl_context: &Sdl) -> Self{

        let video_subsystem = sdl_context.video().unwrap();

        let width = pixel_width * x_size;
        let height = pixel_height * y_size;

        let window = video_subsystem.window("rust-sdl2 demo", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let clear_color = Color::RGB(0,0,0);

        let mut rng = rand::thread_rng();

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

                buffer.push(rect);
            }
            pixels.push(buffer);
        }

        println!("{} {}", pixels.len(), pixels[0].len());

        Screen{
            pixel_width,
            pixel_height,
            width: x_size,
            height: y_size,
            color_width,
            pixels,
            clear_color,
            canvas,
            rng
        }
    }

    pub fn draw(&mut self, signal: Option<Vec<Vec<[u8; 3]>>>) {
        self.canvas.set_draw_color(self.clear_color);
        self.canvas.clear();

        // self.canvas.set_draw_color(Color::RGBA(255,255,255, 255));
        //
        // self.canvas.fill_rect(Rect::new(20, 30, 20, 10)).unwrap();

        if signal == None {
            for y in 0..self.height as usize{
                for x in 0..self.width as usize{
                    let red = self.rng.gen::<u8>();
                    let green = self.rng.gen::<u8>();
                    let blue = self.rng.gen::<u8>();

                    self.canvas.set_draw_color(Color::RGB(
                       red, green, blue
                    ));
                    self.canvas.fill_rect(
                        self.pixels[y][x]
                    ).unwrap();
                }
            }
        }
        else{

            // println!("{} {}",
            //         signal.as_ref().unwrap().len(),
            //         signal.as_ref().unwrap()[0].len());

            for y in 0..self.height as usize{
                for x in 0..self.width as usize{
                    //println!("{} {}", x, y);

                    self.canvas.set_draw_color(
                        Color::RGB(
                            signal.as_ref().unwrap()[y][x][0],
                            signal.as_ref().unwrap()[y][x][1],
                            signal.as_ref().unwrap()[y][x][2]
                        )
                    );
                    self.canvas.fill_rect(
                        self.pixels[y][x]
                    ).unwrap();
                }
            }
        }
        self.canvas.present();
    }

    // fn send_pixels(&mut self, pixels: Vec<Vec<[u8; 4]>>){
    //     assert_eq!(pixels.len(), self.height);
    //     assert_eq!(pixels[0].len(), self.width);
    //     self.pixels = Some(pixels);
    // }
    //
    // fn set_pixel(&mut self, xpos: usize, ypos: usize, screen: &mut [u8], color: [u8; 4]){
    //
    //     // let mut pixels = Vec::new();
    //     //
    //     // for py in 0..self.pixel_height as usize{
    //     //     for px in 0..self.pixel_width as usize{
    //     //         pixels.push(color[0]);
    //     //         pixels.push(color[1]);
    //     //         pixels.push(color[2]);
    //     //         pixels.push(color[3]);
    //     //     }
    //     // }
    //
    //     let width = self.pixel_width * 4;
    //
    //     let mut s = 0;
    //
    //     for y in 0..self.pixel_height{
    //         for mut x in (0..width).step_by(4){
    //             //println!("{} {} {} {}", self.pixel_width, self.pixel_height, self.width, self.height);
    //             let i = (ypos) * self.width * self.pixel_width * 4 * self.pixel_height + xpos * self.pixel_width * 4 + (x + y*self.width*self.pixel_width * 4);
    //
    //             screen[i] = color[0];
    //             screen[i + 1] = color[1];
    //             screen[i + 2] = color[2];
    //             screen[i + 3] = color[3];
    //         }
    //     }
    //
    //     // for y in 0..self.pixel_height {
    //     //     let i = xpos * 4 + ypos * self.width * 4 + y * self.height * 4;
    //     //
    //     //     // Merge pixels from sprite into screen
    //     //     let zipped = screen[i..i + width].iter_mut().zip(&pixels[s..s + width]);
    //     //     for (left, &right) in zipped {
    //     //         if right > 0 {
    //     //             *left = right;
    //     //         }
    //     //     }
    //     //
    //     //     s += width;
    //     // }
    //
    //     // match self.pixels{
    //     //     None => (),
    //     //     Some(_) =>{
    //     //         (self.pixels.as_mut().unwrap())[xpos][ypos] = color;
    //     //     }
    //     // }
    // }
}