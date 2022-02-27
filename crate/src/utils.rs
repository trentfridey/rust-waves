extern crate num;
use num::complex::Complex;
use colors::HexColor;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub const ALPHA: u32 = 0xFF_00_00_00; 
pub const I32_CAP_MIN: i32 = i32::MIN >> 1;
pub const I32_CAP_MAX: i32 = i32::MAX >> 1;

#[derive(Clone, Copy)]
pub enum Status {
    Default,
    Wall,
}
#[derive(Clone)]
pub struct Arena {
    pub width: u32,
    pub height: u32,
    pub status: Vec<Status>,
}

impl Arena {
    pub fn new(width: u32, height: u32) -> Arena {
        let w: usize = width as usize;
        let h: usize = height as usize;
        let status = (0..w * h).map(|x| {
            if x < w { return Status::Wall };                       // Top
            if (x + 1) % w == 0 { return Status::Wall };            // Right
            if h * (w - 1) < x && x < h * w { return Status::Wall };// Bottom
            if x % w == 0 { return Status::Wall };                  // Left
            return Status::Default;
        }).collect();

        return Arena {
            height,
            width,
            status,
        };
    }
    pub fn to_xy(&self, idx: usize) -> (i32, i32) {
        // returns coordinates with origin at center
        // up = +y, right = +x
        let w = self.width as usize;
        let h = self.height as usize;
        let (x0, y0) = (w / 2, h / 2);
        let y = y0 - idx / (self.width as usize);
        let x = (idx % (self.width as usize)) - x0;
        (x as i32, y as i32)
    }
    pub fn to_idx(&self, x: u32, y: u32) -> usize {
        (y * &self.width) as usize + x as usize
    }
}


pub trait Rectified {
    fn apply_cap(self) -> Self;
}

impl Rectified for i32 {
    fn apply_cap(self) -> i32 {
        if self < I32_CAP_MIN {
            return I32_CAP_MIN;
        } else if self > I32_CAP_MAX {
            return I32_CAP_MAX;
        } else {
            return self;
        }
    } 
}

impl Rectified for Complex<i32> {
    fn apply_cap(self) -> Complex<i32> {
        return Complex { re: self.re.apply_cap(), im: self.im.apply_cap() };
    }
}

impl Rectified for f32 {
    fn apply_cap(self) -> Self {
        if self > 1.0 { return 1.0 }
        else if self < -1.0 { return -1.0 }
        else { return self } 
    }
}

// Convert i32 to float in [-1, 1]
pub fn to_amp (x: i32) -> f32 {
    return ((x as f32) + 0.5)/((I32_CAP_MAX) as f32 - 0.5)
}

// Convert float in [-1, 1] to i32 in [I32_CAP_MIN, I32_CAP_MAX]
pub fn from_amp (x: f32) -> i32 {
    return ((I32_CAP_MAX as f32) * x) as i32
}

pub fn gen_test_pattern(width: u32, height: u32) -> Vec<u32> {
    let (min, max) = (I32_CAP_MIN, I32_CAP_MAX);
    // map [0, width] -> [min, max]
    let sampling_scale = |x: u32| { 
        let s = (((max as f32 - min as f32)*(x as f32) / width as f32) as i32) + min;
        return s
    };     
    let mut test: Vec<u32> =  vec![0u32; (width * height) as usize];
    for x in 0..width {
        for y in 0..height {
            let index = (y*width + x) as usize;
            let complex_x = to_amp(sampling_scale(x));
            let arc: f32 = (1.0 - complex_x*complex_x).sqrt();
            let complex_y = -to_amp(sampling_scale(y));
            if -arc <= complex_y && complex_y < arc {
                test[index] = Complex { re: sampling_scale(x), im: sampling_scale(y) }.to_rgba(false);
            }
            else { 
                test[index] = 0xFF_000000;
            }
        }
    }
    return test
}