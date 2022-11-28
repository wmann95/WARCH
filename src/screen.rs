
/// A struct that represents the screen of the emulated computer.
///
/// Fields:
/// * `pixel_width`: The width of the individual pixels of the monitor.
/// * `pixel_height`: The height of the individual pixels of the monitor.
/// * `width`: The pixel count width of the monitor.
/// * `height`:  The pixel count height of the monitor.
struct Screen{
    pixel_width: u64,
    pixel_height: u64,
    width: usize,
    height: usize
}


impl Screen{
    pub fn new(pixel_width: u64, pixel_height: u64, width: usize, height: usize) -> Self{
        Screen{
            pixel_width,
            pixel_height,
            width,
            height
        }
    }

    pub fn draw() {

    }
}